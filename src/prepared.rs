//! Opaque evaluator handles and resource-local node identities.
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::Value;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn next_id() -> u64 {
    let mut current = NEXT_ID.load(Ordering::Relaxed);
    loop {
        let next = current
            .checked_add(1)
            .expect("Evaluator handle identity space exhausted");
        match NEXT_ID.compare_exchange_weak(current, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return current,
            Err(actual) => current = actual,
        }
    }
}

/// Identity of one retained expression; never reused after cache eviction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstraintId(u64);

/// Identity of a node in exactly one prepared resource. Not a JSON address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    resource: u64,
    index: usize,
}

impl NodeId {
    /// Preorder index. Validate ownership with `PreparedResource::owns` first.
    pub fn index(self) -> usize {
        self.index
    }
}

/// Retained implementation-specific executable, with a portable string fallback.
/// The payload must validate engine/registry ownership before execution.
#[derive(Clone)]
pub struct ExecutableExpression {
    id: ConstraintId,
    expression: Arc<str>,
    payload: Option<Arc<dyn Any + Send + Sync>>,
}

impl ExecutableExpression {
    /// String-only handle for evaluators without native prepared execution.
    pub fn new(expression: impl Into<Arc<str>>) -> Self {
        Self {
            id: ConstraintId(next_id()),
            expression: expression.into(),
            payload: None,
        }
    }

    /// Attach an immutable native executable without copying it.
    pub fn with_payload<T: Any + Send + Sync>(
        expression: impl Into<Arc<str>>,
        payload: Arc<T>,
    ) -> Self {
        Self {
            payload: Some(payload),
            ..Self::new(expression)
        }
    }

    /// Stable identity for resource-local constraint memoization.
    pub fn id(&self) -> ConstraintId {
        self.id
    }

    /// Original expression used by compatibility implementations.
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Borrow a concrete implementation's executable, if present.
    pub fn payload<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.payload.as_ref()?.downcast_ref()
    }
}

impl fmt::Debug for ExecutableExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutableExpression")
            .field("id", &self.id)
            .field("expression", &self.expression)
            .finish_non_exhaustive()
    }
}

/// One immutable resource and its evaluator-specific prepared state.
///
/// The strong source Arc prevents mutation/address reuse while node identities
/// are live. Nodes are assigned preorder indices (JSON object iteration order).
/// This handle is request-local; it is not a cross-resource cache.
#[derive(Clone)]
pub struct PreparedResource {
    id: u64,
    source: Arc<Value>,
    nodes: Arc<HashMap<usize, usize>>,
    payload: Arc<dyn Any + Send + Sync>,
}

impl PreparedResource {
    /// Index one owned immutable resource and attach its prepared state.
    pub fn new<T: Any + Send + Sync>(source: Arc<Value>, payload: Arc<T>) -> Self {
        let mut nodes = HashMap::new();
        let mut stack = vec![source.as_ref()];
        while let Some(node) = stack.pop() {
            nodes.insert(node as *const Value as usize, nodes.len());
            match node {
                Value::Array(items) => stack.extend(items.iter().rev()),
                Value::Object(items) => stack.extend(items.values().rev()),
                _ => {}
            }
        }
        Self {
            id: next_id(),
            source,
            nodes: Arc::new(nodes),
            payload,
        }
    }

    /// Shared source. Validation must walk this allocation, not a deep clone.
    pub fn source(&self) -> &Arc<Value> {
        &self.source
    }

    /// Resolve a borrowed node only if it belongs to the owned source tree.
    pub fn node_id(&self, node: &Value) -> Option<NodeId> {
        self.nodes
            .get(&(node as *const Value as usize))
            .map(|&index| NodeId {
                resource: self.id,
                index,
            })
    }

    /// Reject foreign/stale node handles without dereferencing raw addresses.
    pub fn owns(&self, node: NodeId) -> bool {
        node.resource == self.id && node.index < self.nodes.len()
    }

    /// Number of indexed nodes, including the root.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Borrow concrete evaluator state.
    pub fn payload<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.payload.downcast_ref()
    }
}

impl fmt::Debug for PreparedResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PreparedResource")
            .field("id", &self.id)
            .field("nodes", &self.nodes.len())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn node_identity_is_resource_local_and_cow_safe() {
        let mut source = Arc::new(serde_json::json!({"name": [{"family": "one"}]}));
        let first = PreparedResource::new(source.clone(), Arc::new(()));
        let node = first.node_id(&source["name"][0]).unwrap();
        assert_eq!(node.index(), 2);
        assert!(first.clone().owns(node));
        assert!(first.node_id(&source.as_ref().clone()).is_none());
        Arc::make_mut(&mut source)["name"][0]["family"] = "two".into();
        assert!(first.node_id(&source["name"][0]).is_none());
        assert_eq!(first.source()["name"][0]["family"], "one");
        let second = PreparedResource::new(source, Arc::new(()));
        assert!(!second.owns(node));
    }

    #[test]
    fn cloned_executable_retains_identity_and_payload() {
        let plan = ExecutableExpression::with_payload("true", Arc::new(42u64));
        assert_eq!(plan.clone().id(), plan.id());
        assert_eq!(plan.payload::<u64>(), Some(&42));
        assert_ne!(plan.id(), ExecutableExpression::new("true").id());
    }
}
