//! ModelProvider trait and core abstractions for FHIR model access

use crate::boxing::{BoxedFhirPathValue, PrimitiveExtension};
use crate::conformance::ConformanceResult;
use crate::constraints::ConstraintInfo;
use crate::error::Result;
use crate::reflection::TypeReflectionInfo;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// FHIR version enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FhirVersion {
    /// FHIR R4 (4.0.x)
    R4,
    /// FHIR R4B (4.3.x)
    R4B,
    /// FHIR R5 (5.0.x)
    R5,
}

impl FhirVersion {
    /// Get the version string
    pub fn as_str(&self) -> &'static str {
        match self {
            FhirVersion::R4 => "R4",
            FhirVersion::R4B => "R4B",
            FhirVersion::R5 => "R5",
        }
    }

    /// Get the full version string
    pub fn full_version(&self) -> &'static str {
        match self {
            FhirVersion::R4 => "4.0.1",
            FhirVersion::R4B => "4.3.0",
            FhirVersion::R5 => "5.0.0",
        }
    }
}

/// Search parameter definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SearchParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type (string, token, reference, etc.)
    pub param_type: String,
    /// FHIRPath expression for the parameter
    pub expression: String,
    /// Description of the parameter
    pub description: Option<String>,
}

/// Polymorphic type information for choice[x] properties
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PolymorphicTypeInfo {
    /// The concrete type name (e.g., "string", "CodeableConcept")
    pub type_name: String,
    /// The suffix used in property names (e.g., "String", "CodeableConcept")
    pub type_suffix: String,
    /// The full property name (e.g., "valueString", "valueCodeableConcept")
    pub property_name: String,
    /// Type reflection information
    pub type_reflection: TypeReflectionInfo,
    /// Whether this type represents a resource
    pub is_resource: bool,
}

/// Context for reference resolution operations
pub trait ResolutionContext: Send + Sync {
    /// Resolve a local reference within the current bundle/context
    fn resolve_local_reference(
        &self,
        resource_type: &str,
        id: &str,
    ) -> Option<Box<dyn ValueReflection>>;

    /// Resolve an external reference via HTTP or other means
    fn resolve_external_reference(
        &self,
        base_url: &str,
        resource_type: &str,
        id: &str,
    ) -> Option<Box<dyn ValueReflection>>;

    /// Resolve a contained resource reference
    fn resolve_contained_reference(&self, fragment_id: &str) -> Option<Box<dyn ValueReflection>>;

    /// Get the current context path for debugging
    fn current_path(&self) -> String;
}

/// Abstraction for reflecting on FHIR values
pub trait ValueReflection: Send + Sync {
    /// Get the type name of this value
    fn type_name(&self) -> String;

    /// Get a property value by name
    fn get_property(&self, name: &str) -> Option<Box<dyn ValueReflection>>;

    /// Check if a property exists
    fn has_property(&self, name: &str) -> bool;

    /// Get all property names
    fn property_names(&self) -> Vec<String>;

    /// Convert to a JSON-like representation for debugging
    fn to_debug_string(&self) -> String;
}

/// FHIR Structure Definition abstraction
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StructureDefinition {
    /// Structure definition URL
    pub url: String,
    /// Version of the structure definition
    pub version: Option<String>,
    /// Name of the structure definition
    pub name: String,
    /// Base definition URL if this is a profile
    pub base_definition: Option<String>,
    /// Kind of structure (resource, complex-type, primitive-type)
    pub kind: String,
    /// Whether this is an abstract definition
    pub is_abstract: bool,
    /// Element definitions
    pub elements: Vec<ElementDefinition>,
}

/// Element definition within a structure definition
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElementDefinition {
    /// Element path
    pub path: String,
    /// Minimum cardinality
    pub min: u32,
    /// Maximum cardinality (None for unbounded)
    pub max: Option<u32>,
    /// Element types
    pub types: Vec<ElementType>,
    /// Constraints on this element
    pub constraints: Vec<ConstraintInfo>,
}

/// Type information for an element
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElementType {
    /// Type code
    pub code: String,
    /// Target profiles for references
    pub target_profiles: Vec<String>,
}

/// Main trait for providing FHIR model information
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait ModelProvider: Send + Sync + std::fmt::Debug {
    /// Get type reflection information for a type name
    fn get_type_reflection(&self, type_name: &str) -> Option<TypeReflectionInfo>;

    /// Get detailed element reflection information
    fn get_element_reflection(
        &self,
        parent_type: &str,
        element: &str,
    ) -> Option<TypeReflectionInfo>;

    /// Get property type for a given parent type and property name
    fn get_property_type(&self, parent_type: &str, property: &str) -> Option<TypeReflectionInfo>;

    /// Get structure definition by URL
    fn get_structure_definition(&self, url: &str) -> Option<StructureDefinition>;

    /// Validate conformance against a structure definition
    fn validate_conformance(
        &self,
        value: &dyn ValueReflection,
        profile_url: &str,
    ) -> Result<ConformanceResult>;

    /// Get constraints for a type
    fn get_constraints(&self, type_name: &str) -> Vec<ConstraintInfo>;

    /// Resolve a reference to another resource
    fn resolve_reference(
        &self,
        reference: &str,
        context: &dyn ResolutionContext,
    ) -> Option<Box<dyn ValueReflection>>;

    /// Analyze a FHIRPath expression for type safety and optimization opportunities
    fn analyze_expression(&self, expression: &str) -> Result<ExpressionAnalysis>;

    /// **NAVIGATION-DRIVEN BOXING** - Support for primitive extension preservation
    /// Box a value with full metadata preservation during navigation
    fn box_value_with_metadata(
        &self,
        value: &dyn ValueReflection,
        navigation_path: &str,
    ) -> Result<BoxedValueWithMetadata>;

    /// Extract primitive extensions during navigation (e.g., Patient.gender._gender)
    fn extract_primitive_extensions(
        &self,
        parent_value: &dyn ValueReflection,
        property_name: &str,
    ) -> Option<PrimitiveExtensionData>;

    /// Check if a property is polymorphic (e.g., value[x])
    fn is_polymorphic(&self, property: &str) -> bool {
        property.ends_with("[x]")
    }

    /// Get all possible types for a polymorphic property
    fn get_polymorphic_types(&self, property: &str) -> Vec<String> {
        if !self.is_polymorphic(property) {
            return vec![];
        }

        // Standard FHIR polymorphic types
        vec![
            "Boolean".to_string(),
            "Integer".to_string(),
            "String".to_string(),
            "Decimal".to_string(),
            "Uri".to_string(),
            "Url".to_string(),
            "Canonical".to_string(),
            "Base64Binary".to_string(),
            "Instant".to_string(),
            "Date".to_string(),
            "DateTime".to_string(),
            "Time".to_string(),
            "Code".to_string(),
            "Oid".to_string(),
            "Id".to_string(),
            "Markdown".to_string(),
            "UnsignedInt".to_string(),
            "PositiveInt".to_string(),
            "Uuid".to_string(),
            "Quantity".to_string(),
            "Age".to_string(),
            "Distance".to_string(),
            "Duration".to_string(),
            "Count".to_string(),
            "Money".to_string(),
            "Range".to_string(),
            "Period".to_string(),
            "Ratio".to_string(),
            "RatioRange".to_string(),
            "SampledData".to_string(),
            "Signature".to_string(),
            "HumanName".to_string(),
            "Address".to_string(),
            "ContactPoint".to_string(),
            "Timing".to_string(),
            "Reference".to_string(),
            "Annotation".to_string(),
            "Attachment".to_string(),
            "CodeableConcept".to_string(),
            "Identifier".to_string(),
            "Coding".to_string(),
            "Meta".to_string(),
        ]
    }

    /// Get detailed polymorphic type information
    fn get_polymorphic_types_detailed(&self, property: &str) -> Vec<PolymorphicTypeInfo> {
        if !self.is_polymorphic(property) {
            return vec![];
        }

        let base_name = &property[..property.len() - 3]; // Remove "[x]"
        let mut result = Vec::new();

        for type_name in self.get_polymorphic_types(property) {
            if let Some(type_reflection) = self.get_type_reflection(&type_name) {
                result.push(PolymorphicTypeInfo {
                    type_name: type_name.clone(),
                    type_suffix: type_name.clone(),
                    property_name: format!("{base_name}{type_name}"),
                    type_reflection,
                    is_resource: self.is_resource_type(&type_name),
                });
            }
        }

        result
    }

    /// Get search parameters for a resource type
    fn get_search_params(&self, resource_type: &str) -> Vec<SearchParameter>;

    /// Check if a type is a resource type
    fn is_resource_type(&self, type_name: &str) -> bool;

    /// Check if a type is a primitive type
    fn is_primitive_type(&self, type_name: &str) -> bool {
        matches!(
            type_name,
            "boolean"
                | "integer"
                | "string"
                | "decimal"
                | "uri"
                | "url"
                | "canonical"
                | "base64Binary"
                | "instant"
                | "date"
                | "dateTime"
                | "time"
                | "code"
                | "oid"
                | "id"
                | "markdown"
                | "unsignedInt"
                | "positiveInt"
                | "uuid"
        )
    }

    /// Check if a type is a complex type
    fn is_complex_type(&self, type_name: &str) -> bool {
        !self.is_primitive_type(type_name) && !self.is_resource_type(type_name)
    }

    /// Get the FHIR version
    fn fhir_version(&self) -> FhirVersion;

    /// Check if a type is a subtype of another
    fn is_subtype_of(&self, child_type: &str, parent_type: &str) -> bool;

    /// Get all properties for a type
    fn get_properties(&self, type_name: &str) -> Vec<(String, TypeReflectionInfo)>;

    /// Get the base type for a given type
    fn get_base_type(&self, type_name: &str) -> Option<String>;

    /// Clear any internal caches
    fn clear_cache(&self) {}

    /// Get performance metrics if available
    fn get_metrics(&self) -> Option<ProviderMetrics> {
        None
    }

    /// Pre-analyze a FHIRPath expression to determine analysis requirements
    fn preanalyze_fhirpath(&self, _expression: &str) -> Result<FhirPathAnalysisResult> {
        // Default implementation returns basic analysis
        Ok(FhirPathAnalysisResult {
            requires_model_info: false,
            referenced_types: Vec::new(),
            navigation_paths: Vec::new(),
            constraint_dependencies: Vec::new(),
            optimization_hints: Vec::new(),
        })
    }

    /// Validate that navigation operations are type-safe at compile time
    fn validate_navigation_path(&self, base_type: &str, path: &str)
    -> Result<NavigationValidation>;

    /// Enhanced conformance checking with detailed violation reporting
    fn validate_conformance_detailed(
        &self,
        value: &dyn ValueReflection,
        profile_url: &str,
    ) -> Result<DetailedConformanceResult> {
        // Default delegates to basic conformance validation
        self.validate_conformance(value, profile_url)
            .map(|result| DetailedConformanceResult {
                basic_result: result,
                violations: Vec::new(),
                warnings: Vec::new(),
                information: Vec::new(),
            })
    }
}

/// Expression analysis result for mandatory analysis pipeline (ADR-008)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpressionAnalysis {
    /// Types referenced in the expression
    pub referenced_types: Vec<String>,
    /// Navigation paths used in the expression
    pub navigation_paths: Vec<String>,
    /// Whether the expression requires runtime type information
    pub requires_runtime_types: bool,
    /// Optimization opportunities identified
    pub optimization_hints: Vec<OptimizationHint>,
    /// Potential type safety issues
    pub type_safety_warnings: Vec<String>,
}

/// Boxed value with comprehensive metadata for navigation-driven boxing
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoxedValueWithMetadata {
    /// The boxed FHIRPath value
    pub boxed_value: BoxedFhirPathValue,
    /// Navigation context information
    pub navigation_context: NavigationContext,
    /// Type validation results
    pub type_validation: Option<TypeValidationResult>,
}

/// Primitive extension data extracted during navigation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrimitiveExtensionData {
    /// The primitive extension object
    pub extension: PrimitiveExtension,
    /// Element ID from the parent element
    pub element_id: Option<String>,
    /// Extension metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Navigation context for boxed values
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationContext {
    /// Full navigation path
    pub path: String,
    /// Source type information
    pub source_type: String,
    /// Target type information
    pub target_type: Option<String>,
    /// Whether this navigation crossed a reference boundary
    pub crossed_reference: bool,
}

/// Type validation result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeValidationResult {
    /// Whether the type validation passed
    pub is_valid: bool,
    /// Validation messages
    pub messages: Vec<String>,
    /// Suggested fixes
    pub suggestions: Vec<String>,
}

/// Optimization hint for expression analysis
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OptimizationHint {
    /// Type of optimization (e.g., \"index_lookup\", \"cache_opportunity\")
    pub hint_type: String,
    /// Description of the optimization
    pub description: String,
    /// Estimated performance impact (0.0 to 1.0)
    pub impact: f64,
}

/// Result of FHIRPath expression pre-analysis
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FhirPathAnalysisResult {
    /// Whether the expression requires model information
    pub requires_model_info: bool,
    /// Types referenced in the expression
    pub referenced_types: Vec<String>,
    /// Navigation paths identified
    pub navigation_paths: Vec<String>,
    /// Constraint dependencies
    pub constraint_dependencies: Vec<String>,
    /// Optimization hints
    pub optimization_hints: Vec<OptimizationHint>,
}

/// Navigation path validation result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NavigationValidation {
    /// Whether the navigation path is valid
    pub is_valid: bool,
    /// Validation messages
    pub messages: Vec<String>,
    /// Intermediate types in the navigation path
    pub intermediate_types: Vec<String>,
    /// Final result type
    pub result_type: Option<TypeReflectionInfo>,
}

/// Detailed conformance result with violation breakdown
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DetailedConformanceResult {
    /// Basic conformance result
    pub basic_result: ConformanceResult,
    /// Constraint violations (errors)
    pub violations: Vec<ConstraintViolation>,
    /// Warnings
    pub warnings: Vec<ConstraintViolation>,
    /// Informational messages
    pub information: Vec<ConstraintViolation>,
}

/// Constraint violation details
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstraintViolation {
    /// Constraint key that was violated
    pub constraint_key: String,
    /// Human-readable message
    pub message: String,
    /// Path where violation occurred
    pub path: String,
    /// Severity of the violation
    pub severity: ViolationSeverity,
    /// Expected value (if applicable)
    pub expected: Option<String>,
    /// Actual value (if applicable)
    pub actual: Option<String>,
}

/// Severity levels for constraint violations
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViolationSeverity {
    /// Fatal error - processing cannot continue
    Fatal,
    /// Error - constraint violation
    Error,
    /// Warning - should be addressed
    Warning,
    /// Information - for reference
    Information,
}

impl std::fmt::Display for ViolationSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViolationSeverity::Fatal => write!(f, "Fatal"),
            ViolationSeverity::Error => write!(f, "Error"),
            ViolationSeverity::Warning => write!(f, "Warning"),
            ViolationSeverity::Information => write!(f, "Information"),
        }
    }
}

/// Performance metrics for model provider operations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProviderMetrics {
    /// Number of type lookups performed
    pub type_lookups: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Average lookup time in microseconds
    pub avg_lookup_time_us: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
}

impl ProviderMetrics {
    /// Calculate cache hit rate
    pub fn cache_hit_rate(&self) -> f64 {
        if self.cache_hits + self.cache_misses == 0 {
            0.0
        } else {
            self.cache_hits as f64 / (self.cache_hits + self.cache_misses) as f64
        }
    }
}

/// Empty model provider for testing and fallback scenarios
#[derive(Debug, Clone)]
pub struct EmptyModelProvider {
    version: FhirVersion,
}

impl EmptyModelProvider {
    /// Create a new empty model provider
    pub fn new() -> Self {
        Self {
            version: FhirVersion::R5,
        }
    }

    /// Create with specific FHIR version
    pub fn with_version(version: FhirVersion) -> Self {
        Self { version }
    }
}

impl ModelProvider for EmptyModelProvider {
    fn get_type_reflection(&self, _type_name: &str) -> Option<TypeReflectionInfo> {
        None
    }

    fn get_element_reflection(
        &self,
        _parent_type: &str,
        _element: &str,
    ) -> Option<TypeReflectionInfo> {
        None
    }

    fn get_property_type(&self, _parent_type: &str, _property: &str) -> Option<TypeReflectionInfo> {
        None
    }

    fn get_structure_definition(&self, _url: &str) -> Option<StructureDefinition> {
        None
    }

    fn validate_conformance(
        &self,
        _value: &dyn ValueReflection,
        _profile_url: &str,
    ) -> Result<ConformanceResult> {
        Ok(ConformanceResult::empty())
    }

    fn get_constraints(&self, _type_name: &str) -> Vec<ConstraintInfo> {
        Vec::new()
    }

    fn resolve_reference(
        &self,
        _reference: &str,
        _context: &dyn ResolutionContext,
    ) -> Option<Box<dyn ValueReflection>> {
        None
    }

    fn get_search_params(&self, _resource_type: &str) -> Vec<SearchParameter> {
        Vec::new()
    }

    fn is_resource_type(&self, _type_name: &str) -> bool {
        false
    }

    fn fhir_version(&self) -> FhirVersion {
        self.version
    }

    fn is_subtype_of(&self, _child_type: &str, _parent_type: &str) -> bool {
        false
    }

    fn get_properties(&self, _type_name: &str) -> Vec<(String, TypeReflectionInfo)> {
        Vec::new()
    }

    fn get_base_type(&self, _type_name: &str) -> Option<String> {
        None
    }

    fn analyze_expression(&self, _expression: &str) -> Result<ExpressionAnalysis> {
        Ok(ExpressionAnalysis {
            referenced_types: Vec::new(),
            navigation_paths: Vec::new(),
            requires_runtime_types: false,
            optimization_hints: Vec::new(),
            type_safety_warnings: Vec::new(),
        })
    }

    fn box_value_with_metadata(
        &self,
        _value: &dyn ValueReflection,
        _navigation_path: &str,
    ) -> Result<BoxedValueWithMetadata> {
        use crate::error::ModelError;
        Err(ModelError::generic(
            "EmptyModelProvider does not support value boxing",
        ))
    }

    fn extract_primitive_extensions(
        &self,
        _parent_value: &dyn ValueReflection,
        _property_name: &str,
    ) -> Option<PrimitiveExtensionData> {
        None
    }

    fn validate_navigation_path(
        &self,
        _base_type: &str,
        _path: &str,
    ) -> Result<NavigationValidation> {
        Ok(NavigationValidation {
            is_valid: false,
            messages: vec!["EmptyModelProvider cannot validate navigation paths".to_string()],
            intermediate_types: Vec::new(),
            result_type: None,
        })
    }
}

impl Default for EmptyModelProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fhir_version() {
        assert_eq!(FhirVersion::R4.as_str(), "R4");
        assert_eq!(FhirVersion::R4B.as_str(), "R4B");
        assert_eq!(FhirVersion::R5.as_str(), "R5");

        assert_eq!(FhirVersion::R5.full_version(), "5.0.0");
    }

    #[test]
    fn test_empty_provider() {
        let provider = EmptyModelProvider::new();
        assert_eq!(provider.fhir_version(), FhirVersion::R5);
        assert!(provider.get_type_reflection("Patient").is_none());
        assert!(!provider.is_resource_type("Patient"));
        assert!(!provider.is_primitive_type("Patient"));
        assert!(provider.is_primitive_type("string"));
    }

    #[test]
    fn test_polymorphic_detection() {
        let provider = EmptyModelProvider::new();
        assert!(provider.is_polymorphic("value[x]"));
        assert!(!provider.is_polymorphic("value"));

        let types = provider.get_polymorphic_types("value[x]");
        assert!(types.contains(&"String".to_string()));
        assert!(types.contains(&"Boolean".to_string()));
    }

    #[test]
    fn test_provider_metrics() {
        let metrics = ProviderMetrics {
            type_lookups: 100,
            cache_hits: 80,
            cache_misses: 20,
            avg_lookup_time_us: 1.5,
            memory_usage_bytes: 1024,
        };

        assert_eq!(metrics.cache_hit_rate(), 0.8);
    }

    #[test]
    fn test_enhanced_provider_methods() {
        let provider = EmptyModelProvider::new();

        // Test expression analysis
        let analysis = provider.analyze_expression("Patient.name.given").unwrap();
        assert!(analysis.referenced_types.is_empty());
        assert!(analysis.navigation_paths.is_empty());
        assert!(!analysis.requires_runtime_types);

        // Test navigation validation
        let validation = provider
            .validate_navigation_path("Patient", "name.given")
            .unwrap();
        assert!(!validation.is_valid);
        assert!(!validation.messages.is_empty());

        // Test primitive extension extraction
        assert!(
            provider
                .extract_primitive_extensions(&MockValueReflection, "gender")
                .is_none()
        );
    }

    #[test]
    fn test_fhirpath_analysis_result() {
        let result = FhirPathAnalysisResult {
            requires_model_info: true,
            referenced_types: vec!["Patient".to_string(), "HumanName".to_string()],
            navigation_paths: vec!["Patient.name".to_string(), "name.given".to_string()],
            constraint_dependencies: vec!["pat-1".to_string()],
            optimization_hints: vec![OptimizationHint {
                hint_type: "index_lookup".to_string(),
                description: "Consider indexing Patient.name for faster access".to_string(),
                impact: 0.8,
            }],
        };

        assert!(result.requires_model_info);
        assert_eq!(result.referenced_types.len(), 2);
        assert_eq!(result.optimization_hints.len(), 1);
        assert_eq!(result.optimization_hints[0].impact, 0.8);
    }

    #[test]
    fn test_violation_severity() {
        assert_eq!(ViolationSeverity::Fatal.to_string(), "Fatal");
        assert_eq!(ViolationSeverity::Error.to_string(), "Error");
        assert_eq!(ViolationSeverity::Warning.to_string(), "Warning");
        assert_eq!(ViolationSeverity::Information.to_string(), "Information");
    }

    // Mock implementation for testing
    struct MockValueReflection;

    impl ValueReflection for MockValueReflection {
        fn type_name(&self) -> String {
            "Patient".to_string()
        }

        fn get_property(&self, _name: &str) -> Option<Box<dyn ValueReflection>> {
            None
        }

        fn has_property(&self, _name: &str) -> bool {
            false
        }

        fn property_names(&self) -> Vec<String> {
            Vec::new()
        }

        fn to_debug_string(&self) -> String {
            "MockValueReflection".to_string()
        }
    }
}
