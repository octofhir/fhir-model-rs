//! FHIRPath integration types for ModelProvider support
//!
//! This module provides comprehensive types and abstractions for supporting
//! type-aware FHIRPath operations including resolve, conforms, and type checking.

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::navigation::{NavigationPath, OptimizationHint};
use crate::reflection::TypeReflectionInfo;

/// Comprehensive analysis of a FHIRPath expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpressionTypeAnalysis {
    /// The FHIRPath expression being analyzed
    pub expression: String,
    /// All types referenced in the expression
    pub referenced_types: Vec<TypeReference>,
    /// Type operations performed in the expression
    pub type_operations: Vec<TypeOperation>,
    /// Navigation paths extracted from the expression
    pub navigation_paths: Vec<NavigationPath>,
    /// Potential issues identified during analysis
    pub potential_issues: Vec<TypeIssue>,
    /// Optimization opportunities for performance
    pub optimization_opportunities: Vec<OptimizationHint>,
    /// Overall complexity score of the expression
    pub complexity_score: f64,
    /// Expected result type after evaluation
    pub result_type: Option<TypeReflectionInfo>,
}

/// Reference to a type within an expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeReference {
    /// Name of the referenced type
    pub type_name: String,
    /// Context in which the type is used
    pub usage_context: String,
    /// Whether the type reference is explicit or inferred
    pub is_explicit: bool,
    /// Confidence in the type reference (0.0-1.0)
    pub confidence: f64,
    /// Location within the expression
    pub location: ExpressionLocation,
    /// Additional metadata about the reference
    pub metadata: HashMap<String, String>,
}

/// Location within a FHIRPath expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpressionLocation {
    /// Start position in expression
    pub start: usize,
    /// End position in expression
    pub end: usize,
    /// Line number (for multiline expressions)
    pub line: Option<usize>,
    /// Column number
    pub column: Option<usize>,
}

/// Type operation within an expression
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeOperation {
    /// Type of operation being performed
    pub operation_type: TypeOperationType,
    /// Source type for the operation
    pub source_type: String,
    /// Target type (if applicable)
    pub target_type: Option<String>,
    /// Parameters for the operation
    pub parameters: Vec<TypeOperationParameter>,
    /// Location within the expression
    pub location: ExpressionLocation,
    /// Whether the operation is type-safe
    pub is_type_safe: bool,
}

/// Types of type operations in FHIRPath
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeOperationType {
    /// Type casting (as)
    TypeCast,
    /// Type checking (is)
    TypeCheck,
    /// Type filtering (ofType)
    TypeFilter,
    /// Reference resolution (resolve)
    ReferenceResolution,
    /// Conformance checking (conforms)
    ConformanceCheck,
    /// Extension access
    ExtensionAccess,
    /// Choice type access
    ChoiceTypeAccess,
    /// Function invocation
    FunctionInvocation,
}

/// Parameter for a type operation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeOperationParameter {
    /// Parameter name
    pub name: String,
    /// Parameter value
    pub value: String,
    /// Expected type for the parameter
    pub expected_type: String,
    /// Actual inferred type
    pub actual_type: Option<String>,
}

/// Issue identified during type analysis
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeIssue {
    /// Severity of the issue
    pub severity: IssueSeverity,
    /// Category of the issue
    pub category: IssueCategory,
    /// Human-readable description
    pub description: String,
    /// Location within the expression
    pub location: ExpressionLocation,
    /// Suggested resolution
    pub suggested_resolution: Option<String>,
    /// Impact on performance or correctness
    pub impact: ImpactLevel,
}

/// Severity levels for type issues
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IssueSeverity {
    /// Critical error - expression will fail
    Error,
    /// Warning - potential issue
    Warning,
    /// Information - for optimization
    Info,
}

/// Categories of type issues
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IssueCategory {
    /// Type mismatch
    TypeMismatch,
    /// Missing type information
    MissingType,
    /// Ambiguous type resolution
    AmbiguousType,
    /// Performance concern
    Performance,
    /// Deprecated usage
    Deprecation,
    /// Security concern
    Security,
}

/// Impact levels for issues
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ImpactLevel {
    /// High impact - significant performance or correctness issue
    High,
    /// Medium impact - moderate issue
    Medium,
    /// Low impact - minor issue
    Low,
}

/// Type dependency tracking for complex expressions
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeDependency {
    /// Source type in the dependency
    pub source_type: String,
    /// Target type that depends on source
    pub target_type: String,
    /// Kind of dependency relationship
    pub dependency_kind: DependencyKind,
    /// Strength of the dependency (0.0-1.0)
    pub dependency_strength: f64,
    /// Context in which dependency occurs
    pub context: String,
    /// Whether dependency is required or optional
    pub is_required: bool,
}

/// Types of dependency relationships
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DependencyKind {
    /// Type inheritance relationship
    Inheritance,
    /// Composition relationship
    Composition,
    /// Reference relationship
    Reference,
    /// Extension relationship
    Extension,
    /// Choice type relationship
    ChoiceType,
    /// Function parameter relationship
    FunctionParameter,
    /// Constraint relationship
    Constraint,
}

/// Dependency graph for tracking type relationships
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DependencyGraph {
    /// All dependencies in the graph
    pub dependencies: Vec<TypeDependency>,
    /// Types involved in the graph
    pub involved_types: Vec<String>,
    /// Circular dependencies detected
    pub circular_dependencies: Vec<CircularDependency>,
    /// Optimized resolution order
    pub resolution_order: Vec<String>,
    /// Graph complexity metrics
    pub complexity_metrics: GraphComplexityMetrics,
}

/// Circular dependency in type graph
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CircularDependency {
    /// Types involved in the cycle
    pub cycle_types: Vec<String>,
    /// Cycle length
    pub cycle_length: usize,
    /// Severity of the circular dependency
    pub severity: IssueSeverity,
    /// Suggested resolution strategy
    pub resolution_strategy: String,
}

/// Metrics for dependency graph complexity
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraphComplexityMetrics {
    /// Total number of nodes
    pub node_count: usize,
    /// Total number of edges
    pub edge_count: usize,
    /// Maximum depth of dependencies
    pub max_depth: usize,
    /// Average fan-out per node
    pub average_fan_out: f64,
    /// Graph density (0.0-1.0)
    pub density: f64,
}

/// Comprehensive type checking result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeCheckResult {
    /// Whether the expression passed type checking
    pub is_valid: bool,
    /// Type errors found during checking
    pub type_errors: Vec<TypeError>,
    /// Type warnings for potential issues
    pub type_warnings: Vec<TypeWarning>,
    /// Suggested fixes for issues
    pub suggested_fixes: Vec<TypeFix>,
    /// Performance impact analysis
    pub performance_impact: Option<PerformanceImpact>,
    /// Overall confidence in the result (0.0-1.0)
    pub confidence: f64,
    /// Type checking statistics
    pub statistics: TypeCheckStatistics,
}

/// Type error during checking
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeError {
    /// Kind of type error
    pub error_type: TypeErrorKind,
    /// Human-readable error message
    pub message: String,
    /// Location within the expression
    pub location: ExpressionLocation,
    /// Expected type
    pub expected_type: String,
    /// Actual type encountered
    pub actual_type: String,
    /// Additional context information
    pub context: HashMap<String, String>,
}

/// Categories of type errors
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeErrorKind {
    /// Type mismatch between expected and actual
    TypeMismatch,
    /// Unknown or undefined type
    UnknownType,
    /// Invalid operation for type
    InvalidOperation,
    /// Missing required type information
    MissingTypeInfo,
    /// Circular type reference
    CircularReference,
    /// Constraint violation
    ConstraintViolation,
    /// Incompatible choice type usage
    IncompatibleChoiceType,
}

/// Type warning for potential issues
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeWarning {
    /// Kind of warning
    pub warning_type: TypeWarningKind,
    /// Warning message
    pub message: String,
    /// Location within the expression
    pub location: ExpressionLocation,
    /// Recommendation for addressing the warning
    pub recommendation: Option<String>,
    /// Potential impact if ignored
    pub potential_impact: ImpactLevel,
}

/// Categories of type warnings
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeWarningKind {
    /// Potentially inefficient operation
    Performance,
    /// Deprecated type usage
    Deprecation,
    /// Ambiguous type resolution
    Ambiguity,
    /// Unsafe type operation
    Safety,
    /// Missing best practice
    BestPractice,
}

/// Suggested fix for a type issue
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeFix {
    /// Description of the fix
    pub description: String,
    /// Type of fix being suggested
    pub fix_type: TypeFixKind,
    /// Location where fix should be applied
    pub location: ExpressionLocation,
    /// Replacement text for the fix
    pub replacement_text: String,
    /// Confidence in the fix (0.0-1.0)
    pub confidence: f64,
    /// Whether the fix can be applied automatically
    pub is_automatic: bool,
}

/// Categories of type fixes
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeFixKind {
    /// Add explicit type cast
    AddTypeCast,
    /// Change operation type
    ChangeOperation,
    /// Add type constraint
    AddConstraint,
    /// Replace with equivalent expression
    ReplaceExpression,
    /// Add missing type information
    AddTypeInfo,
    /// Optimize for performance
    PerformanceOptimization,
}

/// Performance impact analysis
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceImpact {
    /// Overall performance score (0.0-1.0, higher is better)
    pub performance_score: f64,
    /// Identified performance bottlenecks
    pub bottlenecks: Vec<PerformanceBottleneck>,
    /// Estimated execution cost
    pub estimated_cost: ExecutionCost,
    /// Optimization recommendations
    pub optimization_recommendations: Vec<OptimizationHint>,
}

/// Performance bottleneck identification
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceBottleneck {
    /// Type of bottleneck
    pub bottleneck_type: BottleneckType,
    /// Description of the issue
    pub description: String,
    /// Location within the expression
    pub location: ExpressionLocation,
    /// Severity of the bottleneck
    pub severity: ImpactLevel,
    /// Suggested mitigation
    pub mitigation: String,
}

/// Types of performance bottlenecks
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BottleneckType {
    /// Expensive type operation
    ExpensiveOperation,
    /// Inefficient navigation pattern
    InefficientNavigation,
    /// Redundant computation
    RedundantComputation,
    /// Memory intensive operation
    MemoryIntensive,
    /// Network-dependent operation
    NetworkDependent,
}

/// Execution cost estimation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecutionCost {
    /// CPU cost estimate (relative units)
    pub cpu_cost: f64,
    /// Memory cost estimate (relative units)
    pub memory_cost: f64,
    /// I/O cost estimate (relative units)
    pub io_cost: f64,
    /// Network cost estimate (relative units)
    pub network_cost: f64,
    /// Overall cost score
    pub total_cost: f64,
}

/// Statistics from type checking
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeCheckStatistics {
    /// Number of types analyzed
    pub types_analyzed: usize,
    /// Number of operations checked
    pub operations_checked: usize,
    /// Number of paths validated
    pub paths_validated: usize,
    /// Time taken for type checking (milliseconds)
    pub checking_time_ms: f64,
    /// Memory used during checking (bytes)
    pub memory_used_bytes: usize,
}

impl ExpressionTypeAnalysis {
    /// Create a new expression analysis
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
            referenced_types: Vec::new(),
            type_operations: Vec::new(),
            navigation_paths: Vec::new(),
            potential_issues: Vec::new(),
            optimization_opportunities: Vec::new(),
            complexity_score: 0.0,
            result_type: None,
        }
    }

    /// Add a type reference to the analysis
    pub fn add_type_reference(mut self, reference: TypeReference) -> Self {
        self.referenced_types.push(reference);
        self
    }

    /// Add a type operation to the analysis
    pub fn add_type_operation(mut self, operation: TypeOperation) -> Self {
        self.type_operations.push(operation);
        self
    }

    /// Calculate complexity score based on analysis
    pub fn calculate_complexity(&mut self) {
        let type_count_factor = self.referenced_types.len() as f64 * 0.1;
        let operation_count_factor = self.type_operations.len() as f64 * 0.2;
        let path_count_factor = self.navigation_paths.len() as f64 * 0.15;
        let issue_count_factor = self.potential_issues.len() as f64 * 0.3;

        self.complexity_score =
            (type_count_factor + operation_count_factor + path_count_factor + issue_count_factor)
                .min(1.0);
    }

    /// Check if analysis indicates high complexity
    pub fn is_high_complexity(&self) -> bool {
        self.complexity_score > 0.7
    }

    /// Get all error-level issues
    pub fn get_error_issues(&self) -> Vec<&TypeIssue> {
        self.potential_issues
            .iter()
            .filter(|issue| matches!(issue.severity, IssueSeverity::Error))
            .collect()
    }

    /// Get all warning-level issues
    pub fn get_warning_issues(&self) -> Vec<&TypeIssue> {
        self.potential_issues
            .iter()
            .filter(|issue| matches!(issue.severity, IssueSeverity::Warning))
            .collect()
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    /// Create a new dependency graph
    pub fn new() -> Self {
        Self {
            dependencies: Vec::new(),
            involved_types: Vec::new(),
            circular_dependencies: Vec::new(),
            resolution_order: Vec::new(),
            complexity_metrics: GraphComplexityMetrics {
                node_count: 0,
                edge_count: 0,
                max_depth: 0,
                average_fan_out: 0.0,
                density: 0.0,
            },
        }
    }

    /// Add a dependency to the graph
    pub fn add_dependency(mut self, dependency: TypeDependency) -> Self {
        // Add types to involved types if not present
        if !self.involved_types.contains(&dependency.source_type) {
            self.involved_types.push(dependency.source_type.clone());
        }
        if !self.involved_types.contains(&dependency.target_type) {
            self.involved_types.push(dependency.target_type.clone());
        }

        self.dependencies.push(dependency);
        self.update_metrics();
        self
    }

    /// Detect circular dependencies in the graph
    pub fn detect_cycles(&mut self) {
        // Simple cycle detection algorithm
        // In practice, this would use more sophisticated graph algorithms
        self.circular_dependencies.clear();

        for dependency in &self.dependencies {
            if self.has_path(&dependency.target_type, &dependency.source_type) {
                let cycle = CircularDependency {
                    cycle_types: vec![
                        dependency.source_type.clone(),
                        dependency.target_type.clone(),
                    ],
                    cycle_length: 2,
                    severity: IssueSeverity::Warning,
                    resolution_strategy: "Consider breaking the cycle with abstraction".to_string(),
                };
                self.circular_dependencies.push(cycle);
            }
        }
    }

    /// Check if there's a path between two types
    fn has_path(&self, from: &str, to: &str) -> bool {
        // Simplified path detection
        self.dependencies
            .iter()
            .any(|dep| dep.source_type == from && dep.target_type == to)
    }

    /// Compute optimal resolution order
    pub fn compute_resolution_order(&mut self) {
        // Topological sort for dependency resolution
        // This is a simplified version - real implementation would be more robust
        self.resolution_order = self.involved_types.clone();
        self.resolution_order.sort();
    }

    /// Update complexity metrics
    fn update_metrics(&mut self) {
        self.complexity_metrics.node_count = self.involved_types.len();
        self.complexity_metrics.edge_count = self.dependencies.len();

        if self.complexity_metrics.node_count > 0 {
            self.complexity_metrics.average_fan_out = self.complexity_metrics.edge_count as f64
                / self.complexity_metrics.node_count as f64;

            let max_possible_edges =
                self.complexity_metrics.node_count * (self.complexity_metrics.node_count - 1);
            if max_possible_edges > 0 {
                self.complexity_metrics.density =
                    self.complexity_metrics.edge_count as f64 / max_possible_edges as f64;
            }
        }
    }
}

impl TypeCheckResult {
    /// Create a successful type check result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            type_errors: Vec::new(),
            type_warnings: Vec::new(),
            suggested_fixes: Vec::new(),
            performance_impact: None,
            confidence: 1.0,
            statistics: TypeCheckStatistics::default(),
        }
    }

    /// Create a failed type check result
    pub fn failure(errors: Vec<TypeError>) -> Self {
        Self {
            is_valid: false,
            type_errors: errors,
            type_warnings: Vec::new(),
            suggested_fixes: Vec::new(),
            performance_impact: None,
            confidence: 0.0,
            statistics: TypeCheckStatistics::default(),
        }
    }

    /// Add a type error
    pub fn add_error(mut self, error: TypeError) -> Self {
        self.type_errors.push(error);
        self.is_valid = false;
        self
    }

    /// Add a type warning
    pub fn add_warning(mut self, warning: TypeWarning) -> Self {
        self.type_warnings.push(warning);
        self
    }

    /// Check if result has any issues
    pub fn has_issues(&self) -> bool {
        !self.type_errors.is_empty() || !self.type_warnings.is_empty()
    }

    /// Get severity of the worst issue
    pub fn worst_issue_severity(&self) -> Option<IssueSeverity> {
        if !self.type_errors.is_empty() {
            Some(IssueSeverity::Error)
        } else if !self.type_warnings.is_empty() {
            Some(IssueSeverity::Warning)
        } else {
            None
        }
    }
}

impl Default for TypeCheckStatistics {
    fn default() -> Self {
        Self {
            types_analyzed: 0,
            operations_checked: 0,
            paths_validated: 0,
            checking_time_ms: 0.0,
            memory_used_bytes: 0,
        }
    }
}

impl Default for ExecutionCost {
    fn default() -> Self {
        Self {
            cpu_cost: 0.0,
            memory_cost: 0.0,
            io_cost: 0.0,
            network_cost: 0.0,
            total_cost: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_type_analysis() {
        let mut analysis = ExpressionTypeAnalysis::new("Patient.name.given");

        analysis = analysis.add_type_reference(TypeReference {
            type_name: "Patient".to_string(),
            usage_context: "root".to_string(),
            is_explicit: true,
            confidence: 1.0,
            location: ExpressionLocation {
                start: 0,
                end: 7,
                line: None,
                column: None,
            },
            metadata: HashMap::new(),
        });

        analysis.calculate_complexity();

        assert_eq!(analysis.expression, "Patient.name.given");
        assert_eq!(analysis.referenced_types.len(), 1);
        assert!(analysis.complexity_score > 0.0);
        assert!(!analysis.is_high_complexity());
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();

        graph = graph.add_dependency(TypeDependency {
            source_type: "Patient".to_string(),
            target_type: "HumanName".to_string(),
            dependency_kind: DependencyKind::Composition,
            dependency_strength: 0.9,
            context: "name property".to_string(),
            is_required: true,
        });

        assert_eq!(graph.involved_types.len(), 2);
        assert_eq!(graph.dependencies.len(), 1);
        assert_eq!(graph.complexity_metrics.node_count, 2);
        assert_eq!(graph.complexity_metrics.edge_count, 1);
    }

    #[test]
    fn test_type_check_result() {
        let success_result = TypeCheckResult::success();
        assert!(success_result.is_valid);
        assert!(!success_result.has_issues());
        assert_eq!(success_result.confidence, 1.0);

        let error = TypeError {
            error_type: TypeErrorKind::TypeMismatch,
            message: "Expected string, got boolean".to_string(),
            location: ExpressionLocation {
                start: 0,
                end: 10,
                line: None,
                column: None,
            },
            expected_type: "string".to_string(),
            actual_type: "boolean".to_string(),
            context: HashMap::new(),
        };

        let failure_result = TypeCheckResult::failure(vec![error]);
        assert!(!failure_result.is_valid);
        assert!(failure_result.has_issues());
        assert_eq!(failure_result.confidence, 0.0);
        assert_eq!(failure_result.type_errors.len(), 1);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut graph = DependencyGraph::new();

        graph = graph
            .add_dependency(TypeDependency {
                source_type: "A".to_string(),
                target_type: "B".to_string(),
                dependency_kind: DependencyKind::Reference,
                dependency_strength: 1.0,
                context: "test".to_string(),
                is_required: true,
            })
            .add_dependency(TypeDependency {
                source_type: "B".to_string(),
                target_type: "A".to_string(),
                dependency_kind: DependencyKind::Reference,
                dependency_strength: 1.0,
                context: "test".to_string(),
                is_required: true,
            });

        graph.detect_cycles();
        assert!(!graph.circular_dependencies.is_empty());
    }

    #[test]
    fn test_performance_impact() {
        let bottleneck = PerformanceBottleneck {
            bottleneck_type: BottleneckType::ExpensiveOperation,
            description: "Complex type resolution".to_string(),
            location: ExpressionLocation {
                start: 0,
                end: 20,
                line: None,
                column: None,
            },
            severity: ImpactLevel::High,
            mitigation: "Cache type resolution results".to_string(),
        };

        let impact = PerformanceImpact {
            performance_score: 0.3,
            bottlenecks: vec![bottleneck],
            estimated_cost: ExecutionCost::default(),
            optimization_recommendations: Vec::new(),
        };

        assert_eq!(impact.performance_score, 0.3);
        assert_eq!(impact.bottlenecks.len(), 1);
        assert!(matches!(
            impact.bottlenecks[0].bottleneck_type,
            BottleneckType::ExpensiveOperation
        ));
    }
}
