//! FHIRPath evaluation support types and traits
//!
//! This module provides types and traits for FHIRPath evaluation.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use rust_decimal::prelude::FromPrimitive;

/// Lightweight type information for FHIRPath type() function
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeInfoResult {
    /// Type namespace (e.g., "FHIR", "System")
    pub namespace: String,
    /// Type name (e.g., "Patient", "String")
    pub name: String,
}

impl TypeInfoResult {
    /// Create new type info result
    pub fn new(namespace: &str, name: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
        }
    }

    /// Create a System type (for primitive types)
    pub fn system(name: &str) -> Self {
        Self::new("System", name)
    }

    /// Create a FHIR type (for resources and complex types)
    pub fn fhir(name: &str) -> Self {
        Self::new("FHIR", name)
    }
}

/// Universal result type for FHIRPath expression evaluation
#[derive(Debug, Clone)]
pub enum EvaluationResult {
    /// No value or empty collection
    Empty,

    /// Boolean true/false value
    Boolean(bool, Option<TypeInfoResult>),

    /// Text string value
    String(String, Option<TypeInfoResult>),

    /// High-precision decimal number
    Decimal(rust_decimal::Decimal, Option<TypeInfoResult>),

    /// Whole number value
    Integer(i64, Option<TypeInfoResult>),

    /// 64-bit integer value (explicit)
    Integer64(i64, Option<TypeInfoResult>),

    /// Date value in ISO format
    Date(String, Option<TypeInfoResult>),

    /// DateTime value in ISO format
    DateTime(String, Option<TypeInfoResult>),

    /// Time value in ISO format
    Time(String, Option<TypeInfoResult>),

    /// Quantity with value and unit
    Quantity(rust_decimal::Decimal, String, Option<TypeInfoResult>),

    /// Ordered collection of evaluation results
    Collection {
        /// The ordered items in this collection
        items: Vec<EvaluationResult>,
        /// Whether the original source order was undefined
        has_undefined_order: bool,
        /// Optional type information
        type_info: Option<TypeInfoResult>,
    },

    /// Key-value object representing complex FHIR types
    Object {
        /// The object's properties
        map: HashMap<String, EvaluationResult>,
        /// Optional type information
        type_info: Option<TypeInfoResult>,
    },
}

impl EvaluationResult {
    /// Create a Boolean result with System type
    pub fn boolean(value: bool) -> Self {
        EvaluationResult::Boolean(value, Some(TypeInfoResult::system("Boolean")))
    }

    /// Create a Boolean result with FHIR type
    pub fn fhir_boolean(value: bool) -> Self {
        EvaluationResult::Boolean(value, Some(TypeInfoResult::fhir("boolean")))
    }

    /// Create a String result with System type
    pub fn string(value: String) -> Self {
        EvaluationResult::String(value, Some(TypeInfoResult::system("String")))
    }

    /// Create a String result with FHIR type
    pub fn fhir_string(value: String, fhir_type: &str) -> Self {
        EvaluationResult::String(value, Some(TypeInfoResult::fhir(fhir_type)))
    }

    /// Create an Integer result with System type
    pub fn integer(value: i64) -> Self {
        EvaluationResult::Integer(value, Some(TypeInfoResult::system("Integer")))
    }

    /// Create an Integer result with FHIR type
    pub fn fhir_integer(value: i64) -> Self {
        EvaluationResult::Integer(value, Some(TypeInfoResult::fhir("integer")))
    }

    /// Create a Decimal result with System type
    pub fn decimal(value: rust_decimal::Decimal) -> Self {
        EvaluationResult::Decimal(value, Some(TypeInfoResult::system("Decimal")))
    }

    /// Create a Decimal result with FHIR type
    pub fn fhir_decimal(value: rust_decimal::Decimal) -> Self {
        EvaluationResult::Decimal(value, Some(TypeInfoResult::fhir("decimal")))
    }

    /// Create a Date result with System type
    pub fn date(value: String) -> Self {
        EvaluationResult::Date(value, Some(TypeInfoResult::system("Date")))
    }

    /// Create a DateTime result with System type
    pub fn datetime(value: String) -> Self {
        EvaluationResult::DateTime(value, Some(TypeInfoResult::system("DateTime")))
    }

    /// Create a Time result with System type
    pub fn time(value: String) -> Self {
        EvaluationResult::Time(value, Some(TypeInfoResult::system("Time")))
    }

    /// Create a Quantity result with System type
    pub fn quantity(value: rust_decimal::Decimal, unit: String) -> Self {
        EvaluationResult::Quantity(value, unit, Some(TypeInfoResult::system("Quantity")))
    }

    /// Create a Collection result
    pub fn collection(items: Vec<EvaluationResult>) -> Self {
        EvaluationResult::Collection {
            items,
            has_undefined_order: false,
            type_info: None,
        }
    }

    /// Create an Object variant with just the map, no type information
    pub fn object(map: HashMap<String, EvaluationResult>) -> Self {
        EvaluationResult::Object {
            map,
            type_info: None,
        }
    }

    /// Create an Object variant with type information
    pub fn typed_object(
        map: HashMap<String, EvaluationResult>,
        type_namespace: &str,
        type_name: &str,
    ) -> Self {
        EvaluationResult::Object {
            map,
            type_info: Some(TypeInfoResult::new(type_namespace, type_name)),
        }
    }

    /// Check if this result represents a collection
    pub fn is_collection(&self) -> bool {
        matches!(self, EvaluationResult::Collection { .. })
    }

    /// Returns the count of items according to FHIRPath counting rules
    pub fn count(&self) -> usize {
        match self {
            EvaluationResult::Empty => 0,
            EvaluationResult::Collection { items, .. } => items.len(),
            _ => 1, // All non-collection variants count as 1
        }
    }

    /// Converts the result to a boolean value according to FHIRPath truthiness rules
    pub fn to_boolean(&self) -> bool {
        match self {
            EvaluationResult::Empty => false,
            EvaluationResult::Boolean(b, _) => *b,
            EvaluationResult::String(s, _) => !s.is_empty(),
            EvaluationResult::Decimal(d, _) => !d.is_zero(),
            EvaluationResult::Integer(i, _) => *i != 0,
            EvaluationResult::Integer64(i, _) => *i != 0,
            EvaluationResult::Quantity(q, _, _) => !q.is_zero(),
            EvaluationResult::Collection { items, .. } => !items.is_empty(),
            _ => true, // Date, DateTime, Time, Object are always truthy
        }
    }

    /// Convert to string representation
    pub fn to_string_value(&self) -> String {
        match self {
            EvaluationResult::Empty => "".to_string(),
            EvaluationResult::Boolean(b, _) => b.to_string(),
            EvaluationResult::String(s, _) => s.clone(),
            EvaluationResult::Decimal(d, _) => d.to_string(),
            EvaluationResult::Integer(i, _) => i.to_string(),
            EvaluationResult::Integer64(i, _) => i.to_string(),
            EvaluationResult::Date(d, _) => d.clone(),
            EvaluationResult::DateTime(dt, _) => dt.clone(),
            EvaluationResult::Time(t, _) => t.clone(),
            EvaluationResult::Quantity(val, unit, _) => {
                format!("{val} '{unit}'")
            }
            EvaluationResult::Collection { items, .. } => {
                if items.len() == 1 {
                    items[0].to_string_value()
                } else {
                    format!(
                        "[{}]",
                        items
                            .iter()
                            .map(|r| r.to_string_value())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            EvaluationResult::Object { .. } => "[object]".to_string(),
        }
    }

    /// Get the type name of this evaluation result
    pub fn type_name(&self) -> &'static str {
        match self {
            EvaluationResult::Empty => "Empty",
            EvaluationResult::Boolean(_, _) => "Boolean",
            EvaluationResult::String(_, _) => "String",
            EvaluationResult::Decimal(_, _) => "Decimal",
            EvaluationResult::Integer(_, _) => "Integer",
            EvaluationResult::Integer64(_, _) => "Integer64",
            EvaluationResult::Date(_, _) => "Date",
            EvaluationResult::DateTime(_, _) => "DateTime",
            EvaluationResult::Time(_, _) => "Time",
            EvaluationResult::Quantity(_, _, _) => "Quantity",
            EvaluationResult::Collection { .. } => "Collection",
            EvaluationResult::Object { .. } => "Object",
        }
    }
}

/// Universal conversion trait for transforming values into FHIRPath evaluation results
pub trait IntoEvaluationResult {
    /// Converts this value into a FHIRPath evaluation result
    fn to_evaluation_result(&self) -> EvaluationResult;
}

// === Standard Type Implementations ===

impl IntoEvaluationResult for String {
    fn to_evaluation_result(&self) -> EvaluationResult {
        EvaluationResult::string(self.clone())
    }
}

impl IntoEvaluationResult for &str {
    fn to_evaluation_result(&self) -> EvaluationResult {
        EvaluationResult::string(self.to_string())
    }
}

impl IntoEvaluationResult for bool {
    fn to_evaluation_result(&self) -> EvaluationResult {
        EvaluationResult::boolean(*self)
    }
}

impl IntoEvaluationResult for i32 {
    fn to_evaluation_result(&self) -> EvaluationResult {
        EvaluationResult::integer(*self as i64)
    }
}

impl IntoEvaluationResult for i64 {
    fn to_evaluation_result(&self) -> EvaluationResult {
        EvaluationResult::integer(*self)
    }
}

impl IntoEvaluationResult for f64 {
    fn to_evaluation_result(&self) -> EvaluationResult {
        rust_decimal::Decimal::from_f64(*self)
            .map(EvaluationResult::decimal)
            .unwrap_or(EvaluationResult::Empty)
    }
}

impl IntoEvaluationResult for rust_decimal::Decimal {
    fn to_evaluation_result(&self) -> EvaluationResult {
        EvaluationResult::decimal(*self)
    }
}

// === Generic Container Implementations ===

impl<T> IntoEvaluationResult for Option<T>
where
    T: IntoEvaluationResult,
{
    fn to_evaluation_result(&self) -> EvaluationResult {
        match self {
            Some(value) => value.to_evaluation_result(),
            None => EvaluationResult::Empty,
        }
    }
}

impl<T> IntoEvaluationResult for Vec<T>
where
    T: IntoEvaluationResult,
{
    fn to_evaluation_result(&self) -> EvaluationResult {
        let collection: Vec<EvaluationResult> = self
            .iter()
            .map(|item| item.to_evaluation_result())
            .collect();
        EvaluationResult::Collection {
            items: collection,
            has_undefined_order: false,
            type_info: None,
        }
    }
}

impl<T> IntoEvaluationResult for Box<T>
where
    T: IntoEvaluationResult + ?Sized,
{
    fn to_evaluation_result(&self) -> EvaluationResult {
        (**self).to_evaluation_result()
    }
}

// === Equality Implementation with decimal normalization ===

impl PartialEq for EvaluationResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EvaluationResult::Empty, EvaluationResult::Empty) => true,
            (EvaluationResult::Boolean(a, _), EvaluationResult::Boolean(b, _)) => a == b,
            (EvaluationResult::String(a, _), EvaluationResult::String(b, _)) => a == b,
            (EvaluationResult::Decimal(a, _), EvaluationResult::Decimal(b, _)) => {
                // Normalize decimals to handle precision differences (e.g., 1.0 == 1.00)
                a.normalize() == b.normalize()
            }
            (EvaluationResult::Integer(a, _), EvaluationResult::Integer(b, _)) => a == b,
            (EvaluationResult::Integer64(a, _), EvaluationResult::Integer64(b, _)) => a == b,
            (EvaluationResult::Date(a, _), EvaluationResult::Date(b, _)) => a == b,
            (EvaluationResult::DateTime(a, _), EvaluationResult::DateTime(b, _)) => a == b,
            (EvaluationResult::Time(a, _), EvaluationResult::Time(b, _)) => a == b,
            (
                EvaluationResult::Quantity(val_a, unit_a, _),
                EvaluationResult::Quantity(val_b, unit_b, _),
            ) => {
                // Quantities are equal if both value and unit match (normalized values)
                val_a.normalize() == val_b.normalize() && unit_a == unit_b
            }
            (
                EvaluationResult::Collection {
                    items: a_items,
                    has_undefined_order: a_undef,
                    ..
                },
                EvaluationResult::Collection {
                    items: b_items,
                    has_undefined_order: b_undef,
                    ..
                },
            ) => {
                // Collections are equal if both order flags and items match
                a_undef == b_undef && a_items == b_items
            }
            (EvaluationResult::Object { map: a, .. }, EvaluationResult::Object { map: b, .. }) => {
                a == b
            }
            _ => false,
        }
    }
}

impl Eq for EvaluationResult {}

/// Implement partial ordering for EvaluationResult
impl PartialOrd for EvaluationResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implement total ordering for EvaluationResult
impl Ord for EvaluationResult {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Order variants by type precedence
            (EvaluationResult::Empty, EvaluationResult::Empty) => Ordering::Equal,
            (EvaluationResult::Empty, _) => Ordering::Less,
            (_, EvaluationResult::Empty) => Ordering::Greater,

            (EvaluationResult::Boolean(a, _), EvaluationResult::Boolean(b, _)) => a.cmp(b),
            (EvaluationResult::Boolean(_, _), _) => Ordering::Less,
            (_, EvaluationResult::Boolean(_, _)) => Ordering::Greater,

            (EvaluationResult::Integer(a, _), EvaluationResult::Integer(b, _)) => a.cmp(b),
            (EvaluationResult::Integer(_, _), _) => Ordering::Less,
            (_, EvaluationResult::Integer(_, _)) => Ordering::Greater,

            (EvaluationResult::Integer64(a, _), EvaluationResult::Integer64(b, _)) => a.cmp(b),
            (EvaluationResult::Integer64(_, _), _) => Ordering::Less,
            (_, EvaluationResult::Integer64(_, _)) => Ordering::Greater,

            (EvaluationResult::Decimal(a, _), EvaluationResult::Decimal(b, _)) => a.cmp(b),
            (EvaluationResult::Decimal(_, _), _) => Ordering::Less,
            (_, EvaluationResult::Decimal(_, _)) => Ordering::Greater,

            (EvaluationResult::String(a, _), EvaluationResult::String(b, _)) => a.cmp(b),
            (EvaluationResult::String(_, _), _) => Ordering::Less,
            (_, EvaluationResult::String(_, _)) => Ordering::Greater,

            (EvaluationResult::Date(a, _), EvaluationResult::Date(b, _)) => a.cmp(b),
            (EvaluationResult::Date(_, _), _) => Ordering::Less,
            (_, EvaluationResult::Date(_, _)) => Ordering::Greater,

            (EvaluationResult::DateTime(a, _), EvaluationResult::DateTime(b, _)) => a.cmp(b),
            (EvaluationResult::DateTime(_, _), _) => Ordering::Less,
            (_, EvaluationResult::DateTime(_, _)) => Ordering::Greater,

            (EvaluationResult::Time(a, _), EvaluationResult::Time(b, _)) => a.cmp(b),
            (EvaluationResult::Time(_, _), _) => Ordering::Less,
            (_, EvaluationResult::Time(_, _)) => Ordering::Greater,

            (
                EvaluationResult::Quantity(val_a, unit_a, _),
                EvaluationResult::Quantity(val_b, unit_b, _),
            ) => {
                // Order by value first, then by unit string
                match val_a.cmp(val_b) {
                    Ordering::Equal => unit_a.cmp(unit_b),
                    other => other,
                }
            }
            (EvaluationResult::Quantity(_, _, _), _) => Ordering::Less,
            (_, EvaluationResult::Quantity(_, _, _)) => Ordering::Greater,

            (
                EvaluationResult::Collection {
                    items: a_items,
                    has_undefined_order: a_undef,
                    ..
                },
                EvaluationResult::Collection {
                    items: b_items,
                    has_undefined_order: b_undef,
                    ..
                },
            ) => {
                // Order by undefined_order flag first (false < true), then by items
                match a_undef.cmp(b_undef) {
                    Ordering::Equal => a_items.cmp(b_items),
                    other => other,
                }
            }
            (EvaluationResult::Collection { .. }, _) => Ordering::Less,
            (_, EvaluationResult::Collection { .. }) => Ordering::Greater,

            (EvaluationResult::Object { map: a, .. }, EvaluationResult::Object { map: b, .. }) => {
                // Compare objects by sorted keys, then by values
                let mut a_keys: Vec<_> = a.keys().collect();
                let mut b_keys: Vec<_> = b.keys().collect();
                a_keys.sort();
                b_keys.sort();

                match a_keys.cmp(&b_keys) {
                    Ordering::Equal => {
                        // Same keys: compare values in sorted key order
                        for key in a_keys {
                            match a[key].cmp(&b[key]) {
                                Ordering::Equal => continue,
                                non_equal => return non_equal,
                            }
                        }
                        Ordering::Equal
                    }
                    non_equal => non_equal,
                }
            }
        }
    }
}

/// Implement hashing for EvaluationResult
impl Hash for EvaluationResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the enum variant first to avoid cross-variant collisions
        core::mem::discriminant(self).hash(state);
        match self {
            EvaluationResult::Empty => {}
            EvaluationResult::Boolean(b, _) => b.hash(state),
            EvaluationResult::String(s, _) => s.hash(state),
            // Hash normalized decimal for consistency with equality
            EvaluationResult::Decimal(d, _) => d.normalize().hash(state),
            EvaluationResult::Integer(i, _) => i.hash(state),
            EvaluationResult::Integer64(i, _) => i.hash(state),
            EvaluationResult::Date(d, _) => d.hash(state),
            EvaluationResult::DateTime(dt, _) => dt.hash(state),
            EvaluationResult::Time(t, _) => t.hash(state),
            EvaluationResult::Quantity(val, unit, _) => {
                // Hash both normalized value and unit
                val.normalize().hash(state);
                unit.hash(state);
            }
            EvaluationResult::Collection {
                items,
                has_undefined_order,
                ..
            } => {
                // Hash order flag and items
                has_undefined_order.hash(state);
                items.len().hash(state);
                for item in items {
                    item.hash(state);
                }
            }
            EvaluationResult::Object { map, .. } => {
                // Hash objects with sorted keys for deterministic results
                let mut keys: Vec<_> = map.keys().collect();
                keys.sort();
                keys.len().hash(state);
                for key in keys {
                    key.hash(state);
                    map[key].hash(state);
                }
            }
        }
    }
}

/// Convenience function for converting values to evaluation results
pub fn convert_value_to_evaluation_result<T>(value: &T) -> EvaluationResult
where
    T: IntoEvaluationResult + ?Sized,
{
    value.to_evaluation_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversions() {
        assert_eq!(
            "test".to_evaluation_result(),
            EvaluationResult::string("test".to_string())
        );
        assert_eq!(true.to_evaluation_result(), EvaluationResult::boolean(true));
        assert_eq!(42i64.to_evaluation_result(), EvaluationResult::integer(42));
    }

    #[test]
    fn test_option_conversions() {
        let some_value = Some("test");
        assert_eq!(
            some_value.to_evaluation_result(),
            EvaluationResult::string("test".to_string())
        );

        let none_value: Option<String> = None;
        assert_eq!(none_value.to_evaluation_result(), EvaluationResult::Empty);
    }

    #[test]
    fn test_collection_conversions() {
        let vec = vec!["a", "b", "c"];
        let result = vec.to_evaluation_result();
        assert!(result.is_collection());
        assert_eq!(result.count(), 3);
    }

    #[test]
    fn test_equality_with_decimal_normalization() {
        let d1 = EvaluationResult::decimal(rust_decimal::Decimal::new(100, 2)); // 1.00
        let d2 = EvaluationResult::decimal(rust_decimal::Decimal::new(1, 0)); // 1
        assert_eq!(d1, d2); // Should be equal due to normalization
    }
}
