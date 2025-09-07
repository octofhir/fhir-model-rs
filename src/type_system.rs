//! Enhanced type system for FHIRPath type-aware evaluation
//!
//! This module provides comprehensive type system abstractions including
//! type hierarchies, compatibility matrices, polymorphic contexts, and
//! navigation metadata for advanced FHIRPath operations.

use papaya::HashMap as PapayaHashMap;
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type hierarchy with complete inheritance chain information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeHierarchy {
    /// The type name this hierarchy describes
    pub type_name: String,
    /// All ancestor types up to the root (ordered from immediate parent to root)
    pub ancestors: Vec<String>,
    /// All descendant types (all children and their descendants)
    pub descendants: Vec<String>,
    /// Immediate parent type (if any)
    pub direct_parent: Option<String>,
    /// Immediate child types
    pub direct_children: Vec<String>,
    /// Whether this is an abstract type
    pub is_abstract: bool,
    /// Type derivation method
    pub derivation: DerivationType,
    /// Depth in the type hierarchy (0 = root)
    pub hierarchy_depth: u32,
}

/// Type derivation methods in FHIR
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DerivationType {
    /// Type specialization (adding new elements)
    Specialization,
    /// Constraint application (restricting existing elements)
    Constraint,
    /// Extension (adding extensions)
    Extension,
}

/// Type compatibility matrix for FHIRPath conversions
#[derive(Debug, Clone)]
pub struct TypeCompatibilityMatrix {
    /// All type conversion mappings (from_type, to_type) -> ConversionInfo
    pub conversions: PapayaHashMap<(String, String), ConversionInfo>,
    /// Quick lookup for implicit conversions
    pub implicit_conversions: PapayaHashMap<String, Vec<String>>,
    /// Quick lookup for explicit conversions  
    pub explicit_conversions: PapayaHashMap<String, Vec<String>>,
    /// Function-based conversions (toString(), toInteger(), etc.)
    pub function_conversions: PapayaHashMap<String, Vec<ConversionFunction>>,
}

/// Information about a type conversion
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConversionInfo {
    /// Type of conversion
    pub conversion_type: ConversionType,
    /// Function name if function-based conversion
    pub conversion_function: Option<String>,
    /// Whether data loss is possible during conversion
    pub data_loss_possible: bool,
    /// Additional validation rules that apply
    pub validation_rules: Vec<ValidationRule>,
    /// Performance cost of this conversion (0.0 = free, 1.0 = expensive)
    pub performance_cost: f32,
}

/// Types of conversions supported in FHIRPath
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConversionType {
    /// Automatic conversion with no data loss (Integer -> Decimal)
    Implicit,
    /// Explicit cast required, may have data loss (String -> Integer)
    Explicit,
    /// Requires function call (toString(), toDecimal())
    Function,
    /// Conversion not allowed
    Forbidden,
    /// Conversion requires validation
    Conditional,
}

/// Function-based conversion definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConversionFunction {
    /// Function name (e.g., "toString", "toInteger")
    pub function_name: String,
    /// Target type this function converts to
    pub target_type: String,
    /// Whether the function can fail
    pub can_fail: bool,
    /// Validation requirements
    pub validation_requirements: Vec<String>,
}

/// Validation rule for type operations
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationRule {
    /// Rule identifier
    pub rule_id: String,
    /// Human-readable description
    pub description: String,
    /// FHIRPath expression to validate
    pub validation_expression: Option<String>,
    /// Error message if validation fails
    pub error_message: String,
}

/// Context for polymorphic type resolution
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PolymorphicContext {
    /// Current navigation path
    pub current_path: String,
    /// Base type being navigated from
    pub base_type: String,
    /// Types available in this polymorphic context
    pub available_types: Vec<String>,
    /// Context-specific constraints
    pub constraints: Vec<TypeConstraint>,
    /// Hints for type inference
    pub inference_hints: Vec<InferenceHint>,
    /// Strategy for resolving ambiguous types
    pub resolution_strategy: ResolutionStrategy,
    /// Additional context metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Type constraint for polymorphic resolution
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeConstraint {
    /// Constraint identifier
    pub constraint_id: String,
    /// Types this constraint applies to
    pub applicable_types: Vec<String>,
    /// Constraint expression
    pub constraint_expression: String,
    /// Constraint severity
    pub severity: ConstraintSeverity,
}

/// Hint for type inference
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InferenceHint {
    /// Hint type
    pub hint_type: InferenceHintType,
    /// Suggested type
    pub suggested_type: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Reasoning for this hint
    pub reasoning: String,
}

/// Types of inference hints
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InferenceHintType {
    /// Based on usage statistics
    Statistical,
    /// Based on context analysis
    Contextual,
    /// Based on type constraints
    Constraint,
    /// User-provided hint
    UserProvided,
    /// Based on default type rules
    Default,
}

/// Strategy for resolving polymorphic types
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ResolutionStrategy {
    /// Use first matching type
    FirstMatch,
    /// Use most specific type in hierarchy
    MostSpecific,
    /// Use statistically most common type
    MostCommon,
    /// Infer type from usage context
    ContextInferred,
    /// Require explicit type specification
    ExplicitOnly,
    /// Use confidence-based selection
    ConfidenceBased,
}

/// Severity levels for constraints
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConstraintSeverity {
    /// Must be satisfied
    Error,
    /// Should be satisfied
    Warning,
    /// For information only
    Information,
}

/// Metadata preserved during navigation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationMetadata {
    /// Full navigation path
    pub path: String,
    /// Source type at start of navigation
    pub source_type: String,
    /// Target type at end of navigation
    pub target_type: String,
    /// All intermediate types during navigation
    pub intermediate_types: Vec<String>,
    /// Collection information
    pub collection_info: CollectionInfo,
    /// Polymorphic resolution information
    pub polymorphic_resolution: Option<PolymorphicResolution>,
    /// Navigation warnings
    pub navigation_warnings: Vec<NavigationWarning>,
    /// Performance metadata
    pub performance_metadata: PerformanceMetadata,
}

/// Information about collection behavior
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CollectionInfo {
    /// Whether result is a collection
    pub is_collection: bool,
    /// Type of elements in the collection
    pub element_type: String,
    /// Cardinality constraints
    pub cardinality: Cardinality,
    /// Collection behavior semantics
    pub collection_semantics: CollectionSemantics,
}

/// Cardinality specification
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cardinality {
    /// Minimum number of elements
    pub min: u32,
    /// Maximum number of elements (None = unbounded)
    pub max: Option<u32>,
}

/// FHIRPath collection behavior
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CollectionSemantics {
    /// Whether collections maintain order
    pub is_ordered: bool,
    /// Whether duplicates are allowed
    pub allows_duplicates: bool,
    /// Indexing behavior
    pub indexing_type: IndexingType,
    /// Behavior for empty collections
    pub empty_behavior: EmptyBehavior,
    /// Singleton evaluation rules
    pub singleton_evaluation: SingletonEvaluation,
}

/// Collection indexing types
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IndexingType {
    /// 0-based indexing
    ZeroBased,
    /// 1-based indexing
    OneBased,
    /// Collection cannot be indexed
    NotIndexable,
}

/// Empty collection behavior
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EmptyBehavior {
    /// Empty collections propagate through operations
    Propagate,
    /// Empty collections are treated as null
    TreatAsNull,
    /// Empty collections cause errors
    Error,
}

/// Singleton evaluation rules
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SingletonEvaluation {
    /// Automatically unwrap single-item collections
    Automatic,
    /// Require explicit single() function call
    Explicit,
    /// Never unwrap collections
    Forbidden,
}

/// Result of polymorphic type resolution
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PolymorphicResolution {
    /// The resolved type
    pub resolved_type: String,
    /// Confidence score for this resolution (0.0 to 1.0)
    pub confidence_score: f64,
    /// Method used for resolution
    pub resolution_method: ResolutionMethod,
    /// Alternative types that were considered
    pub alternative_types: Vec<AlternativeType>,
    /// Context used for resolution
    pub resolution_context: PolymorphicContext,
}

/// Method used for polymorphic resolution
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ResolutionMethod {
    /// Type was explicitly specified
    ExplicitType,
    /// Inferred from usage context
    ContextInference,
    /// Based on statistical analysis
    StatisticalAnalysis,
    /// Used default/fallback type
    DefaultFallback,
    /// Based on user preferences
    UserPreference,
}

/// Alternative type considered during resolution
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AlternativeType {
    /// Alternative type name
    pub type_name: String,
    /// Confidence score for this alternative
    pub confidence: f64,
    /// Reason this type was considered
    pub reasoning: String,
}

/// Warning during navigation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationWarning {
    /// Warning type
    pub warning_type: NavigationWarningType,
    /// Warning message
    pub message: String,
    /// Path where warning occurred
    pub path: String,
    /// Suggested resolution
    pub suggestion: Option<String>,
}

/// Types of navigation warnings
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NavigationWarningType {
    /// Type safety concern
    TypeSafety,
    /// Performance concern
    Performance,
    /// Deprecated element usage
    Deprecated,
    /// Ambiguous type resolution
    AmbiguousType,
    /// Potential null reference
    NullReference,
}

/// Performance metadata for operations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceMetadata {
    /// Estimated operation cost (relative scale)
    pub operation_cost: f32,
    /// Whether result can be cached
    pub is_cacheable: bool,
    /// Cache key if cacheable
    pub cache_key: Option<String>,
    /// Memory usage estimate in bytes
    pub memory_estimate: Option<usize>,
}

impl TypeHierarchy {
    /// Create a new type hierarchy
    pub fn new(type_name: String) -> Self {
        Self {
            type_name,
            ancestors: Vec::new(),
            descendants: Vec::new(),
            direct_parent: None,
            direct_children: Vec::new(),
            is_abstract: false,
            derivation: DerivationType::Specialization,
            hierarchy_depth: 0,
        }
    }

    /// Check if this type is an ancestor of another type
    pub fn is_ancestor_of(&self, other_type: &str) -> bool {
        self.descendants.contains(&other_type.to_string())
    }

    /// Check if this type is a descendant of another type
    pub fn is_descendant_of(&self, other_type: &str) -> bool {
        self.ancestors.contains(&other_type.to_string())
    }

    /// Get the common ancestor with another type hierarchy
    pub fn common_ancestor(&self, other: &TypeHierarchy) -> Option<String> {
        for ancestor in &self.ancestors {
            if other.ancestors.contains(ancestor) {
                return Some(ancestor.clone());
            }
        }
        None
    }

    /// Calculate type distance (for type compatibility scoring)
    pub fn type_distance(&self, target_type: &str) -> Option<u32> {
        if self.type_name == target_type {
            return Some(0);
        }

        if let Some(pos) = self.ancestors.iter().position(|t| t == target_type) {
            return Some(pos as u32 + 1);
        }

        if let Some(pos) = self.descendants.iter().position(|t| t == target_type) {
            return Some(pos as u32 + 1);
        }

        None
    }
}

impl TypeCompatibilityMatrix {
    /// Create a new empty compatibility matrix
    pub fn new() -> Self {
        Self {
            conversions: PapayaHashMap::new(),
            implicit_conversions: PapayaHashMap::new(),
            explicit_conversions: PapayaHashMap::new(),
            function_conversions: PapayaHashMap::new(),
        }
    }

    /// Check if implicit conversion is possible
    pub fn can_convert_implicitly(&self, from_type: &str, to_type: &str) -> bool {
        if let Some(targets) = self
            .implicit_conversions
            .get(from_type, &self.implicit_conversions.guard())
        {
            targets.contains(&to_type.to_string())
        } else {
            false
        }
    }

    /// Check if explicit conversion is possible
    pub fn can_convert_explicitly(&self, from_type: &str, to_type: &str) -> bool {
        if let Some(targets) = self
            .explicit_conversions
            .get(from_type, &self.explicit_conversions.guard())
        {
            targets.contains(&to_type.to_string())
        } else {
            false
        }
    }

    /// Get conversion information
    pub fn get_conversion_info(&self, from_type: &str, to_type: &str) -> Option<ConversionInfo> {
        let key = (from_type.to_string(), to_type.to_string());
        self.conversions
            .get(&key, &self.conversions.guard())
            .cloned()
    }

    /// Find conversion path through intermediate types
    pub fn find_conversion_path(&self, from_type: &str, to_type: &str) -> Option<Vec<String>> {
        // Simple implementation - can be enhanced with graph algorithms
        if self.can_convert_implicitly(from_type, to_type) {
            Some(vec![from_type.to_string(), to_type.to_string()])
        } else {
            None
        }
    }
}

impl Default for TypeCompatibilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl Cardinality {
    /// Create cardinality with min and max
    pub fn new(min: u32, max: Option<u32>) -> Self {
        Self { min, max }
    }

    /// Create required cardinality (1..1)
    pub fn required() -> Self {
        Self::new(1, Some(1))
    }

    /// Create optional cardinality (0..1)
    pub fn optional() -> Self {
        Self::new(0, Some(1))
    }

    /// Create multiple cardinality (0..*)
    pub fn multiple() -> Self {
        Self::new(0, None)
    }

    /// Create required multiple cardinality (1..*)
    pub fn required_multiple() -> Self {
        Self::new(1, None)
    }

    /// Check if this cardinality is required
    pub fn is_required(&self) -> bool {
        self.min > 0
    }

    /// Check if this cardinality allows multiple values
    pub fn is_multiple(&self) -> bool {
        self.max.is_none() || self.max.unwrap() > 1
    }

    /// Check if a count satisfies this cardinality
    pub fn satisfies(&self, count: u32) -> bool {
        count >= self.min && (self.max.is_none() || count <= self.max.unwrap())
    }
}

impl Default for CollectionInfo {
    fn default() -> Self {
        Self {
            is_collection: false,
            element_type: "Unknown".to_string(),
            cardinality: Cardinality::optional(),
            collection_semantics: CollectionSemantics {
                is_ordered: true,
                allows_duplicates: true,
                indexing_type: IndexingType::ZeroBased,
                empty_behavior: EmptyBehavior::Propagate,
                singleton_evaluation: SingletonEvaluation::Automatic,
            },
        }
    }
}

impl Default for NavigationMetadata {
    fn default() -> Self {
        Self {
            path: String::new(),
            source_type: String::new(),
            target_type: String::new(),
            intermediate_types: Vec::new(),
            collection_info: CollectionInfo::default(),
            polymorphic_resolution: None,
            navigation_warnings: Vec::new(),
            performance_metadata: PerformanceMetadata {
                operation_cost: 0.0,
                is_cacheable: false,
                cache_key: None,
                memory_estimate: Some(0),
            },
        }
    }
}

impl Default for PolymorphicResolution {
    fn default() -> Self {
        Self {
            resolved_type: "Unknown".to_string(),
            confidence_score: 0.0,
            resolution_method: ResolutionMethod::DefaultFallback,
            alternative_types: Vec::new(),
            resolution_context: PolymorphicContext {
                current_path: String::new(),
                base_type: String::new(),
                available_types: Vec::new(),
                constraints: Vec::new(),
                inference_hints: Vec::new(),
                resolution_strategy: ResolutionStrategy::FirstMatch,
                metadata: std::collections::HashMap::new(),
            },
        }
    }
}

impl Default for CollectionSemantics {
    fn default() -> Self {
        Self {
            is_ordered: true,
            allows_duplicates: true,
            indexing_type: IndexingType::ZeroBased,
            empty_behavior: EmptyBehavior::Propagate,
            singleton_evaluation: SingletonEvaluation::Automatic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_hierarchy() {
        let mut hierarchy = TypeHierarchy::new("Patient".to_string());
        hierarchy.ancestors = vec!["DomainResource".to_string(), "Resource".to_string()];
        hierarchy.descendants = vec!["USCorePatient".to_string()];
        hierarchy.direct_parent = Some("DomainResource".to_string());

        assert!(hierarchy.is_ancestor_of("USCorePatient"));
        assert!(hierarchy.is_descendant_of("Resource"));
        assert_eq!(hierarchy.type_distance("Patient"), Some(0));
        assert_eq!(hierarchy.type_distance("Resource"), Some(2));
    }

    #[test]
    fn test_type_compatibility_matrix() {
        let matrix = TypeCompatibilityMatrix::new();
        assert!(!matrix.can_convert_implicitly("String", "Integer"));
        assert!(matrix.get_conversion_info("String", "Integer").is_none());
    }

    #[test]
    fn test_cardinality() {
        let required = Cardinality::required();
        let optional = Cardinality::optional();
        let multiple = Cardinality::multiple();

        assert!(required.is_required());
        assert!(!required.is_multiple());

        assert!(!optional.is_required());
        assert!(!optional.is_multiple());

        assert!(!multiple.is_required());
        assert!(multiple.is_multiple());

        assert!(required.satisfies(1));
        assert!(!required.satisfies(0));
        assert!(!required.satisfies(2));

        assert!(optional.satisfies(0));
        assert!(optional.satisfies(1));
        assert!(!optional.satisfies(2));

        assert!(multiple.satisfies(0));
        assert!(multiple.satisfies(1));
        assert!(multiple.satisfies(100));
    }
}
