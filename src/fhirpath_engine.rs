//! FHIRPath engine abstraction for constraint evaluation
//!
//! This module provides the `FhirPathEngine` trait that allows fhirschema
//! to evaluate FHIRPath constraints without directly depending on a specific
//! FHIRPath implementation, thus avoiding circular dependencies.

use crate::constraints::{ConstraintInfo, ConstraintResult};
use crate::error::{ModelError, Result};
use async_trait::async_trait;
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Configuration for FHIRPath constraint evaluation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FhirPathEvaluationConfig {
    /// Maximum evaluation time in milliseconds
    pub timeout_ms: u64,
    /// Maximum recursion depth
    pub max_recursion_depth: usize,
    /// Whether to collect detailed metrics
    pub collect_metrics: bool,
    /// Whether to include evaluation details in results
    pub include_details: bool,
    /// Additional configuration parameters
    pub parameters: HashMap<String, String>,
}

impl Default for FhirPathEvaluationConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 5000, // 5 seconds for constraint evaluation
            max_recursion_depth: 100,
            collect_metrics: false,
            include_details: false,
            parameters: HashMap::new(),
        }
    }
}

/// Context information for FHIRPath evaluation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FhirPathEvaluationContext {
    /// Current path being evaluated
    pub current_path: String,
    /// Context variables for evaluation
    pub variables: HashMap<String, serde_json::Value>,
    /// Root resource being validated
    pub root_resource: serde_json::Value,
    /// Current resource in evaluation context
    pub current_resource: Option<serde_json::Value>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl FhirPathEvaluationContext {
    /// Create new evaluation context
    pub fn new(root_resource: serde_json::Value) -> Self {
        Self {
            current_path: String::new(),
            variables: HashMap::new(),
            root_resource,
            current_resource: None,
            metadata: HashMap::new(),
        }
    }

    /// Set a variable in the evaluation context
    pub fn set_variable(&mut self, name: impl Into<String>, value: serde_json::Value) {
        self.variables.insert(name.into(), value);
    }

    /// Set the current resource context
    pub fn with_current_resource(mut self, resource: serde_json::Value) -> Self {
        self.current_resource = Some(resource);
        self
    }

    /// Set the current evaluation path
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.current_path = path.into();
        self
    }
}

/// Batch evaluation result for multiple constraints
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BatchConstraintResult {
    /// Results for each constraint in order
    pub results: Vec<ConstraintResult>,
    /// Overall batch evaluation metrics
    pub batch_metrics: Option<BatchEvaluationMetrics>,
    /// Any batch-level errors
    pub batch_errors: Vec<String>,
}

/// Metrics for batch constraint evaluation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BatchEvaluationMetrics {
    /// Total batch execution time in microseconds
    pub total_time_us: u64,
    /// Number of constraints processed
    pub constraints_processed: usize,
    /// Number of successful evaluations
    pub successful_evaluations: usize,
    /// Number of failed evaluations
    pub failed_evaluations: usize,
    /// Number of evaluation errors
    pub evaluation_errors: usize,
}

/// Abstraction for FHIRPath engines used in constraint validation
///
/// This trait allows fhirschema to evaluate FHIRPath constraints without
/// directly depending on a specific FHIRPath implementation, preventing
/// circular dependencies between fhirschema and fhirpath libraries.
#[async_trait]
pub trait FhirPathEngine: Send + Sync + std::fmt::Debug {
    /// Evaluate a single FHIRPath constraint against a FHIR resource
    ///
    /// # Arguments
    /// * `resource` - The FHIR resource to validate against
    /// * `constraint` - The constraint definition containing the FHIRPath expression
    /// * `context` - Evaluation context with variables and path information
    /// * `config` - Configuration for the evaluation
    ///
    /// # Returns
    /// * `Ok(ConstraintResult)` - The evaluation result
    /// * `Err(ModelError)` - If evaluation fails due to engine errors
    async fn evaluate_constraint(
        &self,
        resource: &serde_json::Value,
        constraint: &ConstraintInfo,
        context: &FhirPathEvaluationContext,
        config: &FhirPathEvaluationConfig,
    ) -> Result<ConstraintResult>;

    /// Evaluate multiple constraints in a batch for better performance
    ///
    /// The default implementation calls `evaluate_constraint` for each constraint,
    /// but implementations can override this for batch optimizations.
    ///
    /// # Arguments
    /// * `resource` - The FHIR resource to validate against
    /// * `constraints` - Vector of constraints to evaluate
    /// * `context` - Evaluation context with variables and path information  
    /// * `config` - Configuration for the evaluation
    ///
    /// # Returns
    /// * `Ok(BatchConstraintResult)` - Results for all constraints
    /// * `Err(ModelError)` - If batch evaluation fails
    async fn evaluate_constraints_batch(
        &self,
        resource: &serde_json::Value,
        constraints: &[&ConstraintInfo],
        context: &FhirPathEvaluationContext,
        config: &FhirPathEvaluationConfig,
    ) -> Result<BatchConstraintResult> {
        let start_time = std::time::Instant::now();
        let mut results = Vec::with_capacity(constraints.len());
        let mut successful = 0;
        let mut failed = 0;
        let mut errors = 0;
        let mut batch_errors = Vec::new();

        for constraint in constraints {
            match self
                .evaluate_constraint(resource, constraint, context, config)
                .await
            {
                Ok(result) => {
                    if result.is_success() {
                        successful += 1;
                    } else {
                        failed += 1;
                    }
                    results.push(result);
                }
                Err(e) => {
                    errors += 1;
                    batch_errors.push(format!("Constraint '{}': {}", constraint.key, e));
                    // Create error result for this constraint
                    results.push(ConstraintResult::error(
                        context.current_path.clone(),
                        format!("Evaluation error: {e}"),
                    ));
                }
            }
        }

        let batch_metrics = if config.collect_metrics {
            Some(BatchEvaluationMetrics {
                total_time_us: start_time.elapsed().as_micros() as u64,
                constraints_processed: constraints.len(),
                successful_evaluations: successful,
                failed_evaluations: failed,
                evaluation_errors: errors,
            })
        } else {
            None
        };

        Ok(BatchConstraintResult {
            results,
            batch_metrics,
            batch_errors,
        })
    }

    /// Check if the engine can handle a specific FHIRPath expression
    ///
    /// This allows validation engines to determine whether to use this engine
    /// or fall back to simpler pattern matching.
    ///
    /// # Arguments
    /// * `expression` - The FHIRPath expression to check
    ///
    /// # Returns
    /// * `true` if the engine supports this expression
    /// * `false` if the expression is not supported
    fn supports_expression(&self, expression: &str) -> bool {
        // Default implementation assumes all expressions are supported
        let _ = expression;
        true
    }

    /// Get engine capabilities and metadata
    fn get_capabilities(&self) -> FhirPathEngineCapabilities {
        FhirPathEngineCapabilities::default()
    }

    /// Get the engine name for logging and debugging
    fn get_engine_name(&self) -> &str {
        "fhirpath-engine"
    }

    /// Get the engine version
    fn get_engine_version(&self) -> &str {
        "unknown"
    }

    /// Validate the engine configuration
    async fn validate_config(&self, config: &FhirPathEvaluationConfig) -> Result<()> {
        if config.timeout_ms == 0 {
            return Err(ModelError::InvalidConfiguration {
                message: "Timeout must be greater than 0".to_string(),
            });
        }

        if config.max_recursion_depth == 0 {
            return Err(ModelError::InvalidConfiguration {
                message: "Max recursion depth must be greater than 0".to_string(),
            });
        }

        Ok(())
    }

    /// Pre-compile/prepare a constraint expression for efficient repeated evaluation
    ///
    /// This is optional - engines that don't support pre-compilation can return Ok(()).
    /// The default implementation does nothing.
    async fn prepare_constraint(&self, constraint: &ConstraintInfo) -> Result<()> {
        let _ = constraint;
        Ok(())
    }
}

/// Capabilities and metadata for a FHIRPath engine
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FhirPathEngineCapabilities {
    /// Maximum supported recursion depth
    pub max_recursion_depth: usize,
    /// Whether the engine supports async evaluation
    pub supports_async: bool,
    /// Whether the engine can provide detailed evaluation metrics
    pub supports_metrics: bool,
    /// Whether the engine supports batch evaluation optimizations
    pub supports_batch_optimization: bool,
    /// Supported FHIRPath language features
    pub supported_features: Vec<String>,
    /// Whether the engine supports constraint pre-compilation
    pub supports_precompilation: bool,
    /// FHIR versions supported by this engine
    pub supported_fhir_versions: Vec<String>,
}

impl Default for FhirPathEngineCapabilities {
    fn default() -> Self {
        Self {
            max_recursion_depth: 100,
            supports_async: true,
            supports_metrics: false,
            supports_batch_optimization: false,
            supported_features: vec![],
            supports_precompilation: false,
            supported_fhir_versions: vec!["R4".to_string(), "R5".to_string()],
        }
    }
}

/// Factory for creating FHIRPath engines
pub trait FhirPathEngineFactory: Send + Sync {
    /// Create a new FHIRPath engine instance
    fn create_engine(
        &self,
    ) -> impl std::future::Future<Output = Result<Box<dyn FhirPathEngine>>> + Send;

    /// Create an engine with specific configuration
    fn create_engine_with_config(
        &self,
        config: &FhirPathEvaluationConfig,
    ) -> impl std::future::Future<Output = Result<Box<dyn FhirPathEngine>>> + Send;

    /// Get factory capabilities
    fn get_factory_info(&self) -> FactoryInfo;
}

/// Information about a FHIRPath engine factory
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FactoryInfo {
    /// Factory name
    pub name: String,
    /// Factory version
    pub version: String,
    /// Description of the engines this factory creates
    pub description: String,
    /// Default configuration used by this factory
    pub default_config: FhirPathEvaluationConfig,
}

impl BatchConstraintResult {
    /// Check if all constraints in the batch passed
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.is_success())
    }

    /// Get the number of successful constraint evaluations
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_success()).count()
    }

    /// Get the number of failed constraint evaluations
    pub fn failure_count(&self) -> usize {
        self.results.iter().filter(|r| !r.is_success()).count()
    }

    /// Check if there were any batch-level errors
    pub fn has_batch_errors(&self) -> bool {
        !self.batch_errors.is_empty()
    }
}

impl std::fmt::Display for FhirPathEngineCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FHIRPath Engine Capabilities: max_depth={}, async={}, metrics={}, batch={}, features={}",
            self.max_recursion_depth,
            self.supports_async,
            self.supports_metrics,
            self.supports_batch_optimization,
            self.supported_features.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluation_context() {
        let resource = serde_json::json!({"resourceType": "Patient"});
        let mut context = FhirPathEvaluationContext::new(resource.clone());

        context.set_variable("test", serde_json::Value::String("value".to_string()));
        assert!(context.variables.contains_key("test"));

        let context = context
            .with_current_resource(resource.clone())
            .with_path("Patient.name");

        assert_eq!(context.current_path, "Patient.name");
        assert!(context.current_resource.is_some());
    }

    #[test]
    fn test_batch_result() {
        let results = vec![
            ConstraintResult::success("path1"),
            ConstraintResult::failure("path2"),
            ConstraintResult::success("path3"),
        ];

        let batch_result = BatchConstraintResult {
            results,
            batch_metrics: None,
            batch_errors: vec!["error1".to_string()],
        };

        assert!(!batch_result.all_passed());
        assert_eq!(batch_result.success_count(), 2);
        assert_eq!(batch_result.failure_count(), 1);
        assert!(batch_result.has_batch_errors());
    }

    #[test]
    fn test_config_default() {
        let config = FhirPathEvaluationConfig::default();
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.max_recursion_depth, 100);
        assert!(!config.collect_metrics);
        assert!(!config.include_details);
    }
}
