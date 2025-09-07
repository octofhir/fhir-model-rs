//! Navigation types for type-safe FHIRPath traversal
//!
//! This module provides comprehensive navigation types that enable type-safe
//! FHIRPath operations with collection semantics and validation.

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::reflection::TypeReflectionInfo;
use crate::type_system::{Cardinality, CollectionInfo, PerformanceMetadata};

/// Optimization hint for performance improvements
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OptimizationHint {
    /// Type of optimization
    pub optimization_type: String,
    /// Description of the optimization
    pub description: String,
    /// Estimated performance impact (0.0 = no impact, 1.0 = significant)
    pub impact: f32,
    /// Suggested action to apply the optimization
    pub suggested_action: String,
}

/// Type-safe path representation for FHIRPath navigation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationPath {
    /// Ordered segments that make up this path
    pub segments: Vec<NavigationSegment>,
    /// The full path as a string (e.g., "Patient.name.given")
    pub full_path: String,
    /// Whether this path is type-safe (all segments validated)
    pub is_type_safe: bool,
    /// Any validation errors found during path analysis
    pub validation_errors: Vec<String>,
    /// Optimization hints for this path
    pub optimization_hints: Vec<OptimizationHint>,
    /// Path complexity metrics
    pub complexity: PathComplexity,
}

/// Individual segment in a navigation path
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationSegment {
    /// Segment name (e.g., "name", "given", "valueString")
    pub name: String,
    /// Type of navigation segment
    pub segment_type: SegmentType,
    /// Source type for this segment
    pub source_type: String,
    /// Target type after this segment
    pub target_type: String,
    /// Cardinality of the target
    pub cardinality: Cardinality,
    /// Whether this segment can fail at runtime
    pub can_fail: bool,
    /// Performance cost of this segment (0.0 = free, 1.0 = expensive)
    pub cost: f32,
}

/// Types of navigation segments
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SegmentType {
    /// Simple property access (e.g., "name")
    Property,
    /// Choice type expansion (e.g., "value" -> "valueString")
    ChoiceExpansion {
        /// Base property name
        base_property: String,
        /// Expanded type name
        expanded_type: String,
    },
    /// Collection navigation (e.g., "name[0]")
    Collection {
        /// Optional collection index
        index: Option<usize>,
    },
    /// Function call (e.g., "exists()", "first()")
    Function {
        /// Function name
        function_name: String,
        /// Function parameters
        parameters: Vec<String>,
    },
    /// Type cast operation (e.g., "as(Patient)")
    TypeCast {
        /// Target type for casting
        target_type: String,
    },
    /// Type filter operation (e.g., "ofType(Patient)")
    TypeFilter {
        /// Type to filter by
        filter_type: String,
    },
    /// Where clause navigation (e.g., "name.where(use = 'official')")
    WhereClause {
        /// Where condition
        condition: String,
    },
}

/// Path complexity analysis
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PathComplexity {
    /// Number of segments in the path
    pub segment_count: usize,
    /// Depth of navigation
    pub depth: u32,
    /// Number of choice type expansions
    pub choice_expansions: u32,
    /// Number of function calls
    pub function_calls: u32,
    /// Overall complexity score (0.0 = simple, 1.0 = very complex)
    pub complexity_score: f32,
    /// Whether path has potential performance issues
    pub has_performance_concerns: bool,
}

/// Navigation step with complete metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationStep {
    /// Name of this step
    pub step_name: String,
    /// Type information before this step
    pub from_type: TypeReflectionInfo,
    /// Type information after this step
    pub to_type: TypeReflectionInfo,
    /// Type of navigation being performed
    pub navigation_type: NavigationType,
    /// Constraints that apply to this step
    pub constraints: Vec<NavigationConstraint>,
    /// Metadata preserved during this step
    pub metadata: HashMap<String, serde_json::Value>,
    /// Performance information for this step
    pub performance: PerformanceMetadata,
}

/// Types of navigation operations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NavigationType {
    /// Simple property access
    PropertyAccess {
        /// Property name
        property_name: String,
        /// Whether property is required
        is_required: bool,
        /// Whether property allows multiple values
        is_multiple: bool,
    },
    /// Choice type expansion with type resolution
    ChoiceTypeExpansion {
        /// Base property name
        base_property: String,
        /// Expanded type name
        expanded_type: String,
        /// Confidence in resolution
        confidence: f64,
    },
    /// Collection navigation with indexing
    CollectionNavigation {
        /// Optional collection index
        index: Option<usize>,
        /// Collection type
        collection_type: String,
        /// Element type
        element_type: String,
    },
    /// FHIRPath function application
    FunctionApplication {
        /// Function name
        function_name: String,
        /// Function parameters
        parameters: Vec<FunctionParameter>,
        /// Function return type
        return_type: String,
    },
    /// Type operation (is, as, ofType)
    TypeOperation {
        /// Type of operation
        operation: TypeOperationType,
        /// Operand type
        operand_type: String,
    },
}

/// Types of FHIRPath type operations
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeOperationType {
    /// Type checking (is)
    Check,
    /// Type casting (as)
    Cast,
    /// Type filtering (ofType)
    Filter,
}

/// Function parameter for navigation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionParameter {
    /// Parameter name
    pub name: String,
    /// Parameter value (as FHIRPath expression)
    pub value: String,
    /// Expected parameter type
    pub expected_type: String,
}

/// Navigation constraint
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationConstraint {
    /// Constraint identifier
    pub constraint_id: String,
    /// Constraint type
    pub constraint_type: ConstraintType,
    /// Constraint expression (FHIRPath)
    pub expression: String,
    /// Error message if constraint violated
    pub error_message: String,
    /// Severity of constraint violation
    pub severity: ConstraintSeverity,
}

/// Types of navigation constraints
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConstraintType {
    /// Type safety constraint
    TypeSafety,
    /// Cardinality constraint  
    Cardinality,
    /// Business rule constraint
    BusinessRule,
    /// Performance constraint
    Performance,
}

/// Constraint severity levels
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

/// Comprehensive navigation result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationResult {
    /// Final result type after navigation
    pub result_type: TypeReflectionInfo,
    /// Collection information if result is a collection
    pub collection_info: CollectionInfo,
    /// Navigation metadata preserved during traversal
    pub navigation_metadata: NavigationMetadata,
    /// Validation results from navigation
    pub validation_results: Vec<ValidationResult>,
    /// Performance hints for optimization
    pub performance_hints: Vec<OptimizationHint>,
    /// Whether navigation succeeded without errors
    pub is_success: bool,
    /// Any errors encountered during navigation
    pub errors: Vec<NavigationError>,
}

/// Navigation metadata preserved during traversal
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationMetadata {
    /// Original navigation path
    pub original_path: String,
    /// Resolved path with type expansions
    pub resolved_path: String,
    /// All intermediate types encountered
    pub intermediate_types: Vec<String>,
    /// Choice types that were resolved
    pub choice_resolutions: Vec<ChoiceResolution>,
    /// Function calls that were made
    pub function_calls: Vec<FunctionCall>,
    /// Type operations that were performed
    pub type_operations: Vec<TypeOperation>,
}

/// Choice type resolution information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChoiceResolution {
    /// Original choice property name
    pub choice_property: String,
    /// Resolved specific type
    pub resolved_type: String,
    /// Confidence in this resolution
    pub confidence: f64,
    /// Method used for resolution
    pub resolution_method: String,
}

/// Function call information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionCall {
    /// Function name
    pub function_name: String,
    /// Function parameters
    pub parameters: Vec<FunctionParameter>,
    /// Return type
    pub return_type: String,
    /// Whether function call succeeded
    pub succeeded: bool,
}

/// Type operation information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeOperation {
    /// Operation type
    pub operation_type: TypeOperationType,
    /// Operand type
    pub operand_type: String,
    /// Operation result
    pub result: bool,
}

/// Validation result for navigation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    /// Validation rule that was applied
    pub rule_id: String,
    /// Validation message
    pub message: String,
    /// Severity of validation result
    pub severity: ConstraintSeverity,
    /// Location where validation was applied
    pub location: String,
}

/// Navigation error
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationError {
    /// Error type
    pub error_type: NavigationErrorType,
    /// Error message
    pub message: String,
    /// Location where error occurred
    pub location: String,
    /// Suggested fixes
    pub suggestions: Vec<String>,
}

/// Types of navigation errors
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NavigationErrorType {
    /// Property does not exist
    PropertyNotFound,
    /// Type mismatch
    TypeMismatch,
    /// Cardinality violation
    CardinalityViolation,
    /// Function not supported
    UnsupportedFunction,
    /// Invalid function parameters
    InvalidParameters,
    /// Collection index out of bounds
    IndexOutOfBounds,
    /// Choice type resolution failed
    ChoiceResolutionFailed,
}

/// Path validation for compile-time checking
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PathValidation {
    /// Path being validated
    pub path: String,
    /// Whether path is valid
    pub is_valid: bool,
    /// Validation errors found
    pub validation_errors: Vec<ValidationError>,
    /// Validation warnings
    pub validation_warnings: Vec<ValidationWarning>,
    /// Suggested corrections
    pub suggested_corrections: Vec<String>,
    /// Type safety analysis
    pub type_safety_analysis: TypeSafetyAnalysis,
    /// Performance analysis
    pub performance_analysis: PerformanceAnalysis,
}

/// Validation error
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationError {
    /// Error code
    pub error_code: String,
    /// Error message
    pub message: String,
    /// Location in path where error occurred
    pub location: PathLocation,
    /// Severity of error
    pub severity: ConstraintSeverity,
}

/// Validation warning
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationWarning {
    /// Warning code
    pub warning_code: String,
    /// Warning message
    pub message: String,
    /// Location in path where warning applies
    pub location: PathLocation,
    /// Recommendation to address warning
    pub recommendation: Option<String>,
}

/// Location within a path
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PathLocation {
    /// Segment index in path
    pub segment_index: usize,
    /// Character position in full path string
    pub character_position: usize,
    /// Segment name
    pub segment_name: String,
}

/// Type safety analysis for paths
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeSafetyAnalysis {
    /// Type violations found
    pub type_violations: Vec<TypeViolation>,
    /// Unsafe operations detected
    pub unsafe_operations: Vec<UnsafeOperation>,
    /// Recommended type casts
    pub recommended_casts: Vec<RecommendedCast>,
    /// Overall safety score (0.0 = unsafe, 1.0 = completely safe)
    pub safety_score: f32,
}

/// Type violation in path
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeViolation {
    /// Location of violation
    pub location: PathLocation,
    /// Expected type
    pub expected_type: String,
    /// Actual type
    pub actual_type: String,
    /// Violation message
    pub message: String,
}

/// Unsafe operation in path
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnsafeOperation {
    /// Location of unsafe operation
    pub location: PathLocation,
    /// Operation that is unsafe
    pub operation: String,
    /// Why it's unsafe
    pub reason: String,
    /// How to make it safe
    pub mitigation: String,
}

/// Recommended type cast
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RecommendedCast {
    /// Location where cast should be applied
    pub location: PathLocation,
    /// Source type
    pub from_type: String,
    /// Target type
    pub to_type: String,
    /// Cast expression to use
    pub cast_expression: String,
}

/// Performance analysis for paths
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceAnalysis {
    /// Overall performance score (0.0 = slow, 1.0 = fast)
    pub performance_score: f32,
    /// Estimated execution time in milliseconds
    pub estimated_time_ms: f64,
    /// Memory usage estimate in bytes
    pub estimated_memory_bytes: usize,
    /// Performance bottlenecks identified
    pub bottlenecks: Vec<PerformanceBottleneck>,
    /// Optimization suggestions
    pub optimizations: Vec<OptimizationHint>,
}

/// Performance bottleneck
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceBottleneck {
    /// Location of bottleneck
    pub location: PathLocation,
    /// Type of bottleneck
    pub bottleneck_type: BottleneckType,
    /// Description of the issue
    pub description: String,
    /// Impact level
    pub impact: ImpactLevel,
}

/// Types of performance bottlenecks
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BottleneckType {
    /// Expensive function call
    ExpensiveFunction,
    /// Large collection traversal
    CollectionTraversal,
    /// Complex type resolution
    TypeResolution,
    /// Inefficient pattern matching
    PatternMatching,
}

/// Impact level of issues
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ImpactLevel {
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
    /// Critical impact
    Critical,
}

// Implementation methods for key types

impl NavigationPath {
    /// Create a new navigation path
    pub fn new(full_path: String) -> Self {
        Self {
            segments: Vec::new(),
            full_path,
            is_type_safe: false,
            validation_errors: Vec::new(),
            optimization_hints: Vec::new(),
            complexity: PathComplexity::default(),
        }
    }

    /// Add a segment to the path
    pub fn add_segment(&mut self, segment: NavigationSegment) {
        self.segments.push(segment);
        self.update_complexity();
    }

    /// Check if path is empty
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Get the final target type of this path
    pub fn target_type(&self) -> Option<&str> {
        self.segments.last().map(|s| s.target_type.as_str())
    }

    /// Calculate total cost of path traversal
    pub fn total_cost(&self) -> f32 {
        self.segments.iter().map(|s| s.cost).sum()
    }

    /// Update complexity analysis
    fn update_complexity(&mut self) {
        let segment_count = self.segments.len();
        let choice_expansions = self
            .segments
            .iter()
            .filter(|s| matches!(s.segment_type, SegmentType::ChoiceExpansion { .. }))
            .count() as u32;
        let function_calls = self
            .segments
            .iter()
            .filter(|s| matches!(s.segment_type, SegmentType::Function { .. }))
            .count() as u32;

        let complexity_score = (segment_count as f32 * 0.1)
            + (choice_expansions as f32 * 0.3)
            + (function_calls as f32 * 0.2);

        self.complexity = PathComplexity {
            segment_count,
            depth: segment_count as u32,
            choice_expansions,
            function_calls,
            complexity_score: complexity_score.min(1.0),
            has_performance_concerns: complexity_score > 0.7,
        };
    }
}

impl NavigationSegment {
    /// Create a simple property segment
    pub fn property(
        name: String,
        source_type: String,
        target_type: String,
        cardinality: Cardinality,
    ) -> Self {
        Self {
            name,
            segment_type: SegmentType::Property,
            source_type,
            target_type,
            cardinality,
            can_fail: false,
            cost: 0.1, // Low cost for simple property access
        }
    }

    /// Create a choice expansion segment
    pub fn choice_expansion(
        name: String,
        base_property: String,
        expanded_type: String,
        source_type: String,
        target_type: String,
        cardinality: Cardinality,
    ) -> Self {
        Self {
            name,
            segment_type: SegmentType::ChoiceExpansion {
                base_property,
                expanded_type,
            },
            source_type,
            target_type,
            cardinality,
            can_fail: true, // Choice expansion can fail
            cost: 0.3,      // Higher cost for choice resolution
        }
    }

    /// Create a function call segment
    pub fn function(
        name: String,
        function_name: String,
        parameters: Vec<String>,
        source_type: String,
        target_type: String,
        cardinality: Cardinality,
    ) -> Self {
        let cost = match function_name.as_str() {
            "exists" | "empty" | "first" | "last" => 0.2, // Simple functions
            "where" | "select" | "all" | "any" => 0.5,    // Complex functions
            _ => 0.4,                                     // Default function cost
        };

        Self {
            name,
            segment_type: SegmentType::Function {
                function_name,
                parameters,
            },
            source_type,
            target_type,
            cardinality,
            can_fail: true, // Functions can fail
            cost,
        }
    }
}

impl Default for PathComplexity {
    fn default() -> Self {
        Self {
            segment_count: 0,
            depth: 0,
            choice_expansions: 0,
            function_calls: 0,
            complexity_score: 0.0,
            has_performance_concerns: false,
        }
    }
}

impl PathComplexity {
    /// Check if complexity is high
    pub fn is_high_complexity(&self) -> bool {
        self.complexity_score > 0.7
    }

    /// Check if path has many segments
    pub fn is_deep_path(&self) -> bool {
        self.depth > 5
    }
}

impl NavigationResult {
    /// Create successful navigation result
    pub fn success(result_type: TypeReflectionInfo) -> Self {
        Self {
            result_type,
            collection_info: CollectionInfo::default(),
            navigation_metadata: NavigationMetadata::default(),
            validation_results: Vec::new(),
            performance_hints: Vec::new(),
            is_success: true,
            errors: Vec::new(),
        }
    }

    /// Create failed navigation result
    pub fn failure(errors: Vec<NavigationError>) -> Self {
        Self {
            result_type: TypeReflectionInfo::simple_type("System", "Unknown"),
            collection_info: CollectionInfo::default(),
            navigation_metadata: NavigationMetadata::default(),
            validation_results: Vec::new(),
            performance_hints: Vec::new(),
            is_success: false,
            errors,
        }
    }
}

impl NavigationMetadata {}

impl PathValidation {
    /// Create a new path validation
    pub fn new(path: String) -> Self {
        Self {
            path,
            is_valid: false,
            validation_errors: Vec::new(),
            validation_warnings: Vec::new(),
            suggested_corrections: Vec::new(),
            type_safety_analysis: TypeSafetyAnalysis::default(),
            performance_analysis: PerformanceAnalysis::default(),
        }
    }

    /// Check if validation passed
    pub fn passed(&self) -> bool {
        self.is_valid && self.validation_errors.is_empty()
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.validation_errors.len()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.validation_warnings.len()
    }
}

impl Default for TypeSafetyAnalysis {
    fn default() -> Self {
        Self {
            type_violations: Vec::new(),
            unsafe_operations: Vec::new(),
            recommended_casts: Vec::new(),
            safety_score: 1.0, // Start with perfect safety
        }
    }
}

impl TypeSafetyAnalysis {
    /// Check if analysis found issues
    pub fn has_issues(&self) -> bool {
        !self.type_violations.is_empty() || !self.unsafe_operations.is_empty()
    }
}

impl Default for PerformanceAnalysis {
    fn default() -> Self {
        Self {
            performance_score: 1.0,       // Start with perfect performance
            estimated_time_ms: 1.0,       // 1ms baseline
            estimated_memory_bytes: 1024, // 1KB baseline
            bottlenecks: Vec::new(),
            optimizations: Vec::new(),
        }
    }
}

impl PerformanceAnalysis {
    /// Check if performance is good
    pub fn is_performant(&self) -> bool {
        self.performance_score > 0.7
    }
}

impl PathValidation {
    /// Create successful path validation result
    pub fn success(path: String) -> Self {
        Self {
            path,
            is_valid: true,
            validation_errors: Vec::new(),
            validation_warnings: Vec::new(),
            suggested_corrections: Vec::new(),
            type_safety_analysis: TypeSafetyAnalysis::default(),
            performance_analysis: PerformanceAnalysis::default(),
        }
    }
}
