//! Choice type framework for polymorphic type resolution and expansion
//!
//! This module provides comprehensive support for FHIR choice types (e.g., value[x])
//! with type-safe expansion, resolution, and inference capabilities.

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::reflection::TypeReflectionInfo;
use crate::type_system::{Cardinality, PolymorphicContext};

/// Comprehensive choice type definition with metadata
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChoiceTypeDefinition {
    /// Base path of the choice type (e.g., "value")
    pub base_path: String,
    /// Property name with choice suffix (e.g., "value[x]")
    pub choice_property: String,
    /// All possible types for this choice
    pub possible_types: Vec<ChoiceTypeOption>,
    /// Rules for expanding choice types
    pub expansion_rules: Vec<ExpansionRule>,
    /// Metadata for choice resolution
    pub resolution_metadata: ChoiceResolutionMetadata,
    /// Constraints that apply to this choice
    pub constraints: Vec<ChoiceConstraint>,
}

/// Individual option in a choice type
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChoiceTypeOption {
    /// Type name (e.g., "string", "boolean")
    pub type_name: String,
    /// Expanded property name (e.g., "valueString")
    pub expanded_property: String,
    /// Complete type information
    pub type_info: TypeReflectionInfo,
    /// Statistical usage frequency (0.0-1.0)
    pub usage_frequency: f64,
    /// Type-specific compatibility rules
    pub compatibility_rules: Vec<CompatibilityRule>,
}

/// Rule for expanding choice types
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpansionRule {
    /// Rule identifier
    pub rule_id: String,
    /// Source type pattern
    pub source_pattern: String,
    /// Target expansion pattern
    pub target_pattern: String,
    /// Rule priority (higher = more important)
    pub priority: u32,
    /// Context where rule applies
    pub applicable_contexts: Vec<String>,
}

/// Metadata for choice type resolution
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChoiceResolutionMetadata {
    /// Default resolution strategy
    pub default_strategy: ResolutionStrategy,
    /// Confidence threshold for automatic resolution
    pub confidence_threshold: f64,
    /// Whether to allow ambiguous resolution
    pub allow_ambiguous: bool,
    /// Fallback type if resolution fails
    pub fallback_type: Option<String>,
    /// Performance hints for resolution
    pub performance_hints: HashMap<String, String>,
}

/// Strategy for resolving choice types
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ResolutionStrategy {
    /// Use most frequent type
    MostFrequent,
    /// Use first matching type
    FirstMatch,
    /// Use type with highest confidence
    HighestConfidence,
    /// Use context-aware resolution
    ContextAware,
    /// Custom resolution logic
    Custom {
        /// Name of the custom strategy
        strategy_name: String,
    },
}

/// Constraint on choice type usage
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChoiceConstraint {
    /// Constraint identifier
    pub constraint_id: String,
    /// Type of constraint
    pub constraint_type: ChoiceConstraintType,
    /// Constraint expression
    pub expression: String,
    /// Error message if violated
    pub error_message: String,
    /// Contexts where constraint applies
    pub applicable_contexts: Vec<String>,
}

/// Types of choice constraints
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ChoiceConstraintType {
    /// Mutual exclusion constraint
    MutualExclusion,
    /// Required together constraint  
    RequiredTogether,
    /// Type hierarchy constraint
    TypeHierarchy,
    /// Cardinality constraint
    Cardinality,
    /// Context-specific constraint
    ContextSpecific,
}

/// Compatibility rule for type selection
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompatibilityRule {
    /// Rule identifier
    pub rule_id: String,
    /// Source type or pattern
    pub source_type: String,
    /// Compatible target types
    pub compatible_types: Vec<String>,
    /// Compatibility score (0.0-1.0)
    pub compatibility_score: f64,
    /// Conversion requirements
    pub conversion_requirements: Vec<ConversionRequirement>,
}

/// Requirement for type conversion
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConversionRequirement {
    /// Requirement type
    pub requirement_type: RequirementType,
    /// Description of requirement
    pub description: String,
    /// Whether requirement is mandatory
    pub is_mandatory: bool,
}

/// Types of conversion requirements
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RequirementType {
    /// Data format validation
    FormatValidation,
    /// Value range check
    RangeValidation,
    /// Pattern matching
    PatternMatching,
    /// Custom validation
    CustomValidation,
}

/// Choice type expansion with bidirectional mappings
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChoiceExpansion {
    /// Original choice property
    pub choice_property: String,
    /// Expanded property mappings (type -> property)
    pub forward_mappings: HashMap<String, String>,
    /// Reverse mappings (property -> type)
    pub reverse_mappings: HashMap<String, String>,
    /// Expanded paths with constraints
    pub expanded_paths: Vec<ExpandedPath>,
    /// Expansion context
    pub expansion_context: ExpansionContext,
}

/// Expanded path with type-specific constraints
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpandedPath {
    /// Full expanded path
    pub path: String,
    /// Type for this path
    pub target_type: String,
    /// Type reflection information
    pub type_info: TypeReflectionInfo,
    /// Path-specific constraints
    pub path_constraints: Vec<PathConstraint>,
    /// Cardinality for this path
    pub cardinality: Cardinality,
}

/// Context for choice type expansion
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpansionContext {
    /// Resource type context
    pub resource_type: Option<String>,
    /// Profile context
    pub profile: Option<String>,
    /// Extension context
    pub extension_context: Option<String>,
    /// Additional context metadata
    pub metadata: HashMap<String, String>,
}

/// Constraint on expanded path
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PathConstraint {
    /// Constraint identifier
    pub constraint_id: String,
    /// Constraint expression (FHIRPath)
    pub expression: String,
    /// Severity level
    pub severity: ConstraintSeverity,
    /// Human-readable description
    pub description: String,
}

/// Severity levels for constraints
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConstraintSeverity {
    /// Error - must be satisfied
    Error,
    /// Warning - should be satisfied
    Warning,
    /// Information - for guidance only
    Information,
}

/// Type inference system for ambiguous cases
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeInference {
    /// Collection of inference rules
    pub inference_rules: Vec<InferenceRule>,
    /// Minimum confidence threshold for inference
    pub confidence_threshold: f64,
    /// Context for inference analysis
    pub inference_context: InferenceContext,
    /// Statistical model for type prediction
    pub statistical_model: Option<StatisticalModel>,
}

/// Rule for type inference
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InferenceRule {
    /// Unique rule identifier
    pub rule_id: String,
    /// Pattern to match against
    pub pattern: String,
    /// Type to infer if pattern matches
    pub inferred_type: String,
    /// Confidence weight for this rule
    pub confidence_weight: f64,
    /// Contexts where rule is applicable
    pub applicable_contexts: Vec<String>,
    /// Additional rule metadata
    pub metadata: HashMap<String, String>,
}

/// Context for type inference
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InferenceContext {
    /// Current polymorphic context
    pub polymorphic_context: Option<PolymorphicContext>,
    /// Value being analyzed
    pub analyzed_value: Option<String>,
    /// Resource context
    pub resource_context: Option<String>,
    /// Historical type usage
    pub historical_usage: HashMap<String, f64>,
}

/// Statistical model for type prediction
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StatisticalModel {
    /// Model type identifier
    pub model_type: String,
    /// Model parameters
    pub parameters: HashMap<String, f64>,
    /// Training data statistics
    pub training_statistics: TrainingStatistics,
    /// Model performance metrics
    pub performance_metrics: HashMap<String, f64>,
}

/// Statistics from model training
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TrainingStatistics {
    /// Number of training samples
    pub sample_count: usize,
    /// Type frequency distribution
    pub type_frequencies: HashMap<String, f64>,
    /// Pattern success rates
    pub pattern_success_rates: HashMap<String, f64>,
    /// Last training date
    pub last_training_date: Option<String>,
}

impl ChoiceTypeDefinition {
    /// Create a new choice type definition
    pub fn new(base_path: impl Into<String>, choice_property: impl Into<String>) -> Self {
        Self {
            base_path: base_path.into(),
            choice_property: choice_property.into(),
            possible_types: Vec::new(),
            expansion_rules: Vec::new(),
            resolution_metadata: ChoiceResolutionMetadata::default(),
            constraints: Vec::new(),
        }
    }

    /// Add a possible type option
    pub fn add_type_option(mut self, option: ChoiceTypeOption) -> Self {
        self.possible_types.push(option);
        self
    }

    /// Add an expansion rule
    pub fn add_expansion_rule(mut self, rule: ExpansionRule) -> Self {
        self.expansion_rules.push(rule);
        self
    }

    /// Get type option by name
    pub fn get_type_option(&self, type_name: &str) -> Option<&ChoiceTypeOption> {
        self.possible_types
            .iter()
            .find(|opt| opt.type_name == type_name)
    }

    /// Get all expanded property names
    pub fn get_expanded_properties(&self) -> Vec<String> {
        self.possible_types
            .iter()
            .map(|opt| opt.expanded_property.clone())
            .collect()
    }

    /// Check if a property is an expanded form of this choice
    pub fn is_expanded_property(&self, property_name: &str) -> bool {
        self.possible_types
            .iter()
            .any(|opt| opt.expanded_property == property_name)
    }

    /// Get type name from expanded property
    pub fn get_type_from_expanded_property(&self, property_name: &str) -> Option<String> {
        self.possible_types
            .iter()
            .find(|opt| opt.expanded_property == property_name)
            .map(|opt| opt.type_name.clone())
    }
}

impl ChoiceTypeOption {
    /// Create a new choice type option
    pub fn new(
        type_name: impl Into<String>,
        expanded_property: impl Into<String>,
        type_info: TypeReflectionInfo,
    ) -> Self {
        Self {
            type_name: type_name.into(),
            expanded_property: expanded_property.into(),
            type_info,
            usage_frequency: 0.0,
            compatibility_rules: Vec::new(),
        }
    }

    /// Set usage frequency
    pub fn with_usage_frequency(mut self, frequency: f64) -> Self {
        self.usage_frequency = frequency.clamp(0.0, 1.0);
        self
    }

    /// Add compatibility rule
    pub fn add_compatibility_rule(mut self, rule: CompatibilityRule) -> Self {
        self.compatibility_rules.push(rule);
        self
    }
}

impl Default for ChoiceResolutionMetadata {
    fn default() -> Self {
        Self {
            default_strategy: ResolutionStrategy::ContextAware,
            confidence_threshold: 0.7,
            allow_ambiguous: false,
            fallback_type: None,
            performance_hints: HashMap::new(),
        }
    }
}

impl ChoiceExpansion {
    /// Create a new choice expansion
    pub fn new(choice_property: impl Into<String>) -> Self {
        Self {
            choice_property: choice_property.into(),
            forward_mappings: HashMap::new(),
            reverse_mappings: HashMap::new(),
            expanded_paths: Vec::new(),
            expansion_context: ExpansionContext::default(),
        }
    }

    /// Add type mapping
    pub fn add_mapping(
        mut self,
        type_name: impl Into<String>,
        property_name: impl Into<String>,
    ) -> Self {
        let type_str = type_name.into();
        let property_str = property_name.into();

        self.forward_mappings
            .insert(type_str.clone(), property_str.clone());
        self.reverse_mappings.insert(property_str, type_str);
        self
    }

    /// Expand choice type to specific type
    pub fn expand_to_type(&self, type_name: &str) -> Option<String> {
        self.forward_mappings.get(type_name).cloned()
    }

    /// Get base type from expanded property
    pub fn get_base_type(&self, expanded_property: &str) -> Option<String> {
        self.reverse_mappings.get(expanded_property).cloned()
    }
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeInference {
    /// Create a new type inference system
    pub fn new() -> Self {
        Self {
            inference_rules: Vec::new(),
            confidence_threshold: 0.6,
            inference_context: InferenceContext::default(),
            statistical_model: None,
        }
    }

    /// Add inference rule
    pub fn add_rule(mut self, rule: InferenceRule) -> Self {
        self.inference_rules.push(rule);
        self
    }

    /// Infer type from value and context
    pub fn infer_type(&self, value: &str) -> Option<TypeInferenceResult> {
        let mut candidates = Vec::new();

        for rule in &self.inference_rules {
            if self.rule_matches(rule, value) {
                let confidence = self.calculate_confidence(rule, value);
                if confidence >= self.confidence_threshold {
                    candidates.push(TypeCandidate {
                        type_name: rule.inferred_type.clone(),
                        confidence,
                        rule_id: rule.rule_id.clone(),
                    });
                }
            }
        }

        if candidates.is_empty() {
            return None;
        }

        // Sort by confidence (highest first)
        candidates.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        Some(TypeInferenceResult {
            best_match: candidates[0].clone(),
            alternatives: candidates[1..].to_vec(),
            overall_confidence: candidates[0].confidence,
        })
    }

    /// Check if rule matches value
    fn rule_matches(&self, rule: &InferenceRule, value: &str) -> bool {
        // Simple pattern matching - in practice, this would be more sophisticated
        if rule.pattern == "*" {
            return true;
        }

        // Check for basic pattern types
        match rule.pattern.as_str() {
            "numeric" => value.parse::<f64>().is_ok(),
            "boolean" => value.parse::<bool>().is_ok(),
            "date" => self.is_date_format(value),
            _ => value.contains(&rule.pattern),
        }
    }

    /// Calculate confidence for rule match
    fn calculate_confidence(&self, rule: &InferenceRule, _value: &str) -> f64 {
        // Base confidence from rule weight
        let mut confidence = rule.confidence_weight;

        // Adjust based on historical usage
        if let Some(historical) = self
            .inference_context
            .historical_usage
            .get(&rule.inferred_type)
        {
            confidence *= 0.7 + (historical * 0.3);
        }

        confidence.clamp(0.0, 1.0)
    }

    /// Check if value matches date format
    fn is_date_format(&self, value: &str) -> bool {
        // Simple date format check - would be more sophisticated in practice
        value.len() >= 8 && value.contains('-')
    }
}

/// Result of type inference
#[derive(Debug, Clone)]
pub struct TypeInferenceResult {
    /// Best matching type
    pub best_match: TypeCandidate,
    /// Alternative type candidates
    pub alternatives: Vec<TypeCandidate>,
    /// Overall confidence in inference
    pub overall_confidence: f64,
}

/// Type candidate from inference
#[derive(Debug, Clone)]
pub struct TypeCandidate {
    /// Candidate type name
    pub type_name: String,
    /// Confidence score
    pub confidence: f64,
    /// ID of rule that produced this candidate
    pub rule_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reflection::TypeReflectionInfo;

    #[test]
    fn test_choice_type_definition() {
        let type_info = TypeReflectionInfo::simple_type("FHIR", "string");

        let choice_def = ChoiceTypeDefinition::new("value", "value[x]").add_type_option(
            ChoiceTypeOption::new("string", "valueString", type_info.clone())
                .with_usage_frequency(0.8),
        );

        assert_eq!(choice_def.base_path, "value");
        assert_eq!(choice_def.choice_property, "value[x]");
        assert_eq!(choice_def.possible_types.len(), 1);

        let option = &choice_def.possible_types[0];
        assert_eq!(option.type_name, "string");
        assert_eq!(option.expanded_property, "valueString");
        assert_eq!(option.usage_frequency, 0.8);
    }

    #[test]
    fn test_choice_expansion() {
        let expansion = ChoiceExpansion::new("value[x]")
            .add_mapping("string", "valueString")
            .add_mapping("boolean", "valueBoolean");

        assert_eq!(
            expansion.expand_to_type("string"),
            Some("valueString".to_string())
        );
        assert_eq!(
            expansion.expand_to_type("boolean"),
            Some("valueBoolean".to_string())
        );
        assert_eq!(expansion.expand_to_type("integer"), None);

        assert_eq!(
            expansion.get_base_type("valueString"),
            Some("string".to_string())
        );
        assert_eq!(
            expansion.get_base_type("valueBoolean"),
            Some("boolean".to_string())
        );
        assert_eq!(expansion.get_base_type("valueInteger"), None);
    }

    #[test]
    fn test_type_inference() {
        let inference = TypeInference::new()
            .add_rule(InferenceRule {
                rule_id: "numeric".to_string(),
                pattern: "numeric".to_string(),
                inferred_type: "decimal".to_string(),
                confidence_weight: 0.9,
                applicable_contexts: vec!["*".to_string()],
                metadata: HashMap::new(),
            })
            .add_rule(InferenceRule {
                rule_id: "boolean".to_string(),
                pattern: "boolean".to_string(),
                inferred_type: "boolean".to_string(),
                confidence_weight: 0.95,
                applicable_contexts: vec!["*".to_string()],
                metadata: HashMap::new(),
            });

        let result = inference.infer_type("123.45");
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.best_match.type_name, "decimal");
        assert!(result.best_match.confidence >= 0.6);

        let result = inference.infer_type("true");
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.best_match.type_name, "boolean");
        assert!(result.best_match.confidence >= 0.6);
    }

    #[test]
    fn test_choice_type_operations() {
        let type_info = TypeReflectionInfo::simple_type("FHIR", "string");
        let choice_def = ChoiceTypeDefinition::new("value", "value[x]")
            .add_type_option(ChoiceTypeOption::new("string", "valueString", type_info));

        assert!(choice_def.is_expanded_property("valueString"));
        assert!(!choice_def.is_expanded_property("valueInteger"));

        assert_eq!(
            choice_def.get_type_from_expanded_property("valueString"),
            Some("string".to_string())
        );
        assert_eq!(
            choice_def.get_type_from_expanded_property("valueInteger"),
            None
        );

        let properties = choice_def.get_expanded_properties();
        assert_eq!(properties, vec!["valueString"]);
    }
}
