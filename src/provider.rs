//! ModelProvider trait for FHIR model access
//!
//! This module provides the ModelProvider trait using only types that exist in the codebase.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use async_trait::async_trait;
use serde_json::Value as JsonValue;

// Import only the types that actually exist in our codebase
use crate::choice_types::{ChoiceExpansion, ChoiceTypeDefinition, TypeInference};
use crate::conformance::ConformanceResult;
use crate::constraints::ConstraintInfo;
use crate::error::Result;
use crate::fhirpath_types::{ExpressionTypeAnalysis, TypeCheckResult, TypeDependency};
use crate::navigation::{NavigationResult, OptimizationHint, PathValidation};
use crate::reflection::TypeReflectionInfo;
use crate::type_system::{
    CollectionSemantics, NavigationMetadata, PolymorphicContext, PolymorphicResolution,
    TypeCompatibilityMatrix, TypeHierarchy,
};

/// Core trait for accessing FHIR model information
#[async_trait]
pub trait ModelProvider: Send + Sync + std::fmt::Debug {
    // ========================================================================
    // Core Type Operations
    // ========================================================================

    /// Get type hierarchy information for a specific type
    async fn get_type_hierarchy(&self, type_name: &str) -> Result<Option<TypeHierarchy>>;

    /// Check compatibility between two types
    async fn is_type_compatible(&self, from_type: &str, to_type: &str) -> Result<bool>;

    /// Get common supertype for a set of types
    async fn get_common_supertype(&self, types: &[String]) -> Result<Option<String>>;

    /// Get type compatibility matrix
    async fn get_type_compatibility_matrix(&self) -> Result<TypeCompatibilityMatrix>;

    // ========================================================================
    // Navigation Operations
    // ========================================================================

    /// Navigate a typed path and return result
    async fn navigate_typed_path(&self, base_type: &str, path: &str) -> Result<NavigationResult>;

    /// Validate navigation path for type safety
    async fn validate_navigation_safety(
        &self,
        base_type: &str,
        path: &str,
    ) -> Result<PathValidation>;

    /// Get expected result type from navigation
    async fn get_navigation_result_type(
        &self,
        base_type: &str,
        path: &str,
    ) -> Result<Option<TypeReflectionInfo>>;

    /// Get navigation metadata
    async fn get_navigation_metadata(
        &self,
        base_type: &str,
        path: &str,
    ) -> Result<NavigationMetadata>;

    // ========================================================================
    // Choice Type Operations
    // ========================================================================

    /// Resolve choice type in given context
    async fn resolve_choice_type(
        &self,
        base_path: &str,
        context: &PolymorphicContext,
    ) -> Result<PolymorphicResolution>;

    /// Get choice type expansions
    async fn get_choice_expansions(&self, choice_property: &str) -> Result<Vec<ChoiceExpansion>>;

    /// Infer choice type from context
    async fn infer_choice_type(&self, context: &PolymorphicContext) -> Result<TypeInference>;

    /// Get choice type definition
    async fn get_choice_type_definition(
        &self,
        base_path: &str,
    ) -> Result<Option<ChoiceTypeDefinition>>;

    // ========================================================================
    // FHIRPath Functions
    // ========================================================================

    /// Check conformance to profile
    async fn conforms_to_profile(&self, profile_url: &str) -> Result<ConformanceResult>;

    /// Analyze expression for type information
    async fn analyze_expression_types(&self, expression: &str) -> Result<ExpressionTypeAnalysis>;

    /// Validate FHIRPath expression
    async fn validate_fhirpath_expression(
        &self,
        expression: &str,
        base_type: &str,
    ) -> Result<TypeCheckResult>;

    /// Get expression dependencies
    async fn get_expression_dependencies(&self, expression: &str) -> Result<Vec<TypeDependency>>;

    // ========================================================================
    // Advanced Operations
    // ========================================================================

    /// Get collection semantics for a type
    async fn get_collection_semantics(&self, type_name: &str) -> Result<CollectionSemantics>;

    /// Get optimization hints
    async fn get_optimization_hints(&self, expression: &str) -> Result<Vec<OptimizationHint>>;

    /// Clear caches
    async fn clear_caches(&self) -> Result<()>;

    // ========================================================================
    // Core Information Methods
    // ========================================================================

    /// Get type reflection information
    async fn get_type_reflection(&self, type_name: &str) -> Result<Option<TypeReflectionInfo>>;

    /// Get constraint information
    async fn get_constraints(&self, type_name: &str) -> Result<Vec<ConstraintInfo>>;

    /// Get FHIR version
    fn get_fhir_version(&self) -> FhirVersion;

    /// Get supported resource types
    async fn get_supported_resource_types(&self) -> Result<Vec<String>>;

    /// O(1) check if a resource type exists in the converted schemas
    /// IMPORTANT: This should use data extracted from schemas, not hardcoded lists
    fn resource_type_exists(&self, _resource_type: &str) -> Result<bool> {
        // Default implementation for backward compatibility
        // Implementors should override this for O(1) performance
        Ok(false)
    }

    /// Refresh resource types cache from current schema storage
    /// IMPORTANT: This should re-extract data from schemas, ensuring no stale hardcoded data
    async fn refresh_resource_types(&self) -> Result<()> {
        // Default implementation - no-op for backward compatibility
        Ok(())
    }

    // ========================================================================
    // Resource Resolution (FHIRPath resolve())
    // ========================================================================
    /// Resolve a reference string to a resource JSON, if available.
    ///
    /// Implementors may resolve against in-memory caches, Bundles, or external servers.
    /// Default implementation returns Ok(None).
    async fn resolve_reference(
        &self,
        _reference: &str,
        _base_context: Option<&JsonValue>,
    ) -> Result<Option<JsonValue>> {
        Ok(None)
    }
}

/// FHIR version enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FhirVersion {
    /// FHIR R4 (4.0.x)
    R4,
    /// FHIR R4B (4.3.x)
    R4B,
    /// FHIR R5 (5.0.x)
    R5,
    /// FHIR R6 (6.0.x)
    R6,
    /// Custom version
    Custom {
        /// Version identifier
        version: String,
    },
}

// ============================================================================
// Empty ModelProvider Implementation
// ============================================================================

/// Empty implementation of ModelProvider for testing and default behavior
#[derive(Debug, Clone, Default)]
pub struct EmptyModelProvider;

#[async_trait]
impl ModelProvider for EmptyModelProvider {
    async fn get_type_hierarchy(&self, _type_name: &str) -> Result<Option<TypeHierarchy>> {
        Ok(None)
    }

    async fn is_type_compatible(&self, _from_type: &str, _to_type: &str) -> Result<bool> {
        Ok(false)
    }

    async fn get_common_supertype(&self, _types: &[String]) -> Result<Option<String>> {
        Ok(None)
    }

    async fn get_type_compatibility_matrix(&self) -> Result<TypeCompatibilityMatrix> {
        Ok(TypeCompatibilityMatrix::new())
    }

    async fn navigate_typed_path(&self, _base_type: &str, _path: &str) -> Result<NavigationResult> {
        Ok(NavigationResult::success(TypeReflectionInfo::simple_type(
            "FHIR", "Unknown",
        )))
    }

    async fn validate_navigation_safety(
        &self,
        base_type: &str,
        path: &str,
    ) -> Result<PathValidation> {
        Ok(PathValidation::success(format!("{base_type}.{path}")))
    }

    async fn get_navigation_result_type(
        &self,
        _base_type: &str,
        _path: &str,
    ) -> Result<Option<TypeReflectionInfo>> {
        Ok(None)
    }

    async fn get_navigation_metadata(
        &self,
        _base_type: &str,
        _path: &str,
    ) -> Result<NavigationMetadata> {
        Ok(NavigationMetadata::default())
    }

    async fn resolve_choice_type(
        &self,
        _base_path: &str,
        _context: &PolymorphicContext,
    ) -> Result<PolymorphicResolution> {
        Ok(PolymorphicResolution::default())
    }

    async fn get_choice_expansions(&self, _choice_property: &str) -> Result<Vec<ChoiceExpansion>> {
        Ok(Vec::new())
    }

    async fn infer_choice_type(&self, _context: &PolymorphicContext) -> Result<TypeInference> {
        Ok(TypeInference::new())
    }

    async fn get_choice_type_definition(
        &self,
        _base_path: &str,
    ) -> Result<Option<ChoiceTypeDefinition>> {
        Ok(None)
    }

    async fn conforms_to_profile(&self, profile_url: &str) -> Result<ConformanceResult> {
        Ok(ConformanceResult::new(profile_url, "Unknown"))
    }

    async fn analyze_expression_types(&self, expression: &str) -> Result<ExpressionTypeAnalysis> {
        Ok(ExpressionTypeAnalysis::new(expression))
    }

    async fn validate_fhirpath_expression(
        &self,
        _expression: &str,
        _base_type: &str,
    ) -> Result<TypeCheckResult> {
        Ok(TypeCheckResult::success())
    }

    async fn get_expression_dependencies(&self, _expression: &str) -> Result<Vec<TypeDependency>> {
        Ok(Vec::new())
    }

    async fn get_collection_semantics(&self, _type_name: &str) -> Result<CollectionSemantics> {
        Ok(CollectionSemantics::default())
    }

    async fn get_optimization_hints(&self, _expression: &str) -> Result<Vec<OptimizationHint>> {
        Ok(Vec::new())
    }

    async fn clear_caches(&self) -> Result<()> {
        Ok(())
    }

    async fn get_type_reflection(&self, _type_name: &str) -> Result<Option<TypeReflectionInfo>> {
        Ok(None)
    }

    async fn get_constraints(&self, _type_name: &str) -> Result<Vec<ConstraintInfo>> {
        Ok(Vec::new())
    }

    fn get_fhir_version(&self) -> FhirVersion {
        FhirVersion::R4
    }

    async fn get_supported_resource_types(&self) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    fn resource_type_exists(&self, _resource_type: &str) -> Result<bool> {
        Ok(false)
    }

    async fn refresh_resource_types(&self) -> Result<()> {
        Ok(())
    }
}

impl std::fmt::Display for FhirVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FhirVersion::R4 => write!(f, "R4"),
            FhirVersion::R4B => write!(f, "R4B"),
            FhirVersion::R5 => write!(f, "R5"),
            FhirVersion::R6 => write!(f, "R6"),
            FhirVersion::Custom { version } => write!(f, "{version}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_empty_model_provider() {
        let provider = EmptyModelProvider;

        // Test core type operations
        let hierarchy = provider.get_type_hierarchy("Patient").await.unwrap();
        assert!(hierarchy.is_none());

        let compatibility = provider
            .is_type_compatible("string", "boolean")
            .await
            .unwrap();
        assert!(!compatibility);

        // Test navigation operations
        let nav_result = provider
            .navigate_typed_path("Patient", "name")
            .await
            .unwrap();
        assert!(nav_result.is_success);

        // Test choice type operations
        let choice_expansions = provider.get_choice_expansions("value[x]").await.unwrap();
        assert!(choice_expansions.is_empty());

        // Test FHIRPath functions
        let analysis = provider
            .analyze_expression_types("Patient.name")
            .await
            .unwrap();
        assert_eq!(analysis.expression, "Patient.name");

        // Test FHIR version
        assert_eq!(provider.get_fhir_version(), FhirVersion::R4);
    }

    #[test]
    fn test_fhir_version_display() {
        assert_eq!(format!("{}", FhirVersion::R4), "R4");
        assert_eq!(format!("{}", FhirVersion::R4B), "R4B");
        assert_eq!(format!("{}", FhirVersion::R5), "R5");
        assert_eq!(format!("{}", FhirVersion::R6), "R6");
        assert_eq!(
            format!(
                "{}",
                FhirVersion::Custom {
                    version: "6.0.0".to_string()
                }
            ),
            "6.0.0"
        );
    }
}
