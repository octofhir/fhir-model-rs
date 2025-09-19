//! ModelProvider trait for FHIR model access
//!
//! This module provides the core ModelProvider trait for FHIRPath evaluation.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use async_trait::async_trait;

use crate::error::Result;
use crate::evaluation::{EvaluationResult, IntoEvaluationResult, TypeInfoResult};

/// Core trait for accessing FHIR model information
///
/// Core trait for accessing FHIR model information during FHIRPath evaluation
#[async_trait]
pub trait ModelProvider: Send + Sync + std::fmt::Debug {
    /// Core type lookup
    async fn get_type(&self, type_name: &str) -> Result<Option<TypeInfo>>;

    /// Get element type from complex type
    async fn get_element_type(
        &self,
        parent_type: &TypeInfo,
        property_name: &str,
    ) -> Result<Option<TypeInfo>>;

    /// Get type from union type
    fn of_type(&self, type_info: &TypeInfo, target_type: &str) -> Option<TypeInfo>;

    /// Get element names from complex type
    fn get_element_names(&self, parent_type: &TypeInfo) -> Vec<String>;

    /// Returns a union type of all possible child element types
    async fn get_children_type(&self, parent_type: &TypeInfo) -> Result<Option<TypeInfo>>;

    /// Get detailed information about elements of a type for completion suggestions
    async fn get_elements(&self, type_name: &str) -> Result<Vec<ElementInfo>>;

    /// Get list of all resource types
    async fn get_resource_types(&self) -> Result<Vec<String>>;

    /// Get list of all complex types
    async fn get_complex_types(&self) -> Result<Vec<String>>;

    /// Get list of all primitive types
    async fn get_primitive_types(&self) -> Result<Vec<String>>;

    /// Check if a resource type exists
    async fn resource_type_exists(&self, resource_type: &str) -> Result<bool> {
        let resource_types = self.get_resource_types().await?;
        Ok(resource_types.contains(&resource_type.to_string()))
    }

    /// Get the FHIR version supported by this provider
    async fn get_fhir_version(&self) -> Result<FhirVersion> {
        // Default implementation returns R4
        Ok(FhirVersion::R4)
    }

    /// Check if one type is derived from another using schema hierarchy
    /// Default implementation - override in concrete providers with actual schema data
    fn is_type_derived_from(&self, derived_type: &str, base_type: &str) -> bool {
        // Default implementation for base trait - just direct equality
        derived_type == base_type
    }

    /// Get choice type metadata for a property (valueX patterns)
    async fn get_choice_types(
        &self,
        parent_type: &str,
        property_name: &str,
    ) -> Result<Option<Vec<ChoiceTypeInfo>>> {
        let _ = (parent_type, property_name);
        Ok(None)
    }

    /// Get union type information for a type
    async fn get_union_types(&self, type_info: &TypeInfo) -> Result<Option<Vec<TypeInfo>>> {
        let _ = type_info;
        Ok(None)
    }

    /// Check if a type is a union type
    fn is_union_type(&self, type_info: &TypeInfo) -> bool {
        let _ = type_info;
        false
    }
}

/// Type information structure for FHIR elements
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeInfo {
    /// FHIRPath type name ('Any', 'Boolean', 'String', 'Integer', etc.)
    pub type_name: String,
    /// Single value vs collection (optional, defaults to true)
    pub singleton: Option<bool>,
    /// Indicates this is definitely an empty collection
    pub is_empty: Option<bool>,
    /// Model type namespace ('FHIR', 'System', etc.)
    pub namespace: Option<String>,
    /// Model type name (Patient, string, etc.)
    pub name: Option<String>,
}

impl TypeInfo {
    /// Create a system type
    pub fn system_type(type_name: String, singleton: bool) -> Self {
        Self {
            type_name: type_name.clone(),
            singleton: Some(singleton),
            is_empty: Some(false),
            namespace: Some("System".to_string()),
            name: Some(type_name),
        }
    }
}

/// Element information for completion suggestions
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElementInfo {
    /// Element name
    pub name: String,
    /// Element type
    pub element_type: String,
    /// Documentation/description
    pub documentation: Option<String>,
}

/// Choice type information for valueX properties
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChoiceTypeInfo {
    /// The suffix for the property (e.g., "String" for valueString)
    pub suffix: String,
    /// The FHIR type name (e.g., "string")
    pub type_name: String,
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

/// Empty implementation of ModelProvider for testing and default behavior
#[derive(Debug, Clone, Default)]
pub struct EmptyModelProvider;

#[async_trait]
impl ModelProvider for EmptyModelProvider {
    async fn get_type(&self, type_name: &str) -> Result<Option<TypeInfo>> {
        match type_name {
            "Patient" | "Observation" | "Practitioner" | "Organization" => Ok(Some(TypeInfo {
                type_name: "Any".to_string(),
                singleton: Some(true),
                is_empty: Some(false),
                namespace: Some("FHIR".to_string()),
                name: Some(type_name.to_string()),
            })),
            "Boolean" => Ok(Some(TypeInfo {
                type_name: "Boolean".to_string(),
                singleton: Some(true),
                is_empty: Some(false),
                namespace: Some("System".to_string()),
                name: Some("Boolean".to_string()),
            })),
            "String" => Ok(Some(TypeInfo {
                type_name: "String".to_string(),
                singleton: Some(true),
                is_empty: Some(false),
                namespace: Some("System".to_string()),
                name: Some("String".to_string()),
            })),
            "Integer" => Ok(Some(TypeInfo {
                type_name: "Integer".to_string(),
                singleton: Some(true),
                is_empty: Some(false),
                namespace: Some("System".to_string()),
                name: Some("Integer".to_string()),
            })),
            "Decimal" => Ok(Some(TypeInfo {
                type_name: "Decimal".to_string(),
                singleton: Some(true),
                is_empty: Some(false),
                namespace: Some("System".to_string()),
                name: Some("Decimal".to_string()),
            })),
            _ => Ok(None),
        }
    }

    async fn get_element_type(
        &self,
        parent_type: &TypeInfo,
        property_name: &str,
    ) -> Result<Option<TypeInfo>> {
        match (
            parent_type
                .name
                .as_deref()
                .unwrap_or(&parent_type.type_name),
            property_name,
        ) {
            ("Patient", "name") => Ok(Some(TypeInfo {
                type_name: "Any".to_string(),
                singleton: Some(false),
                is_empty: Some(false),
                namespace: Some("FHIR".to_string()),
                name: Some("HumanName".to_string()),
            })),
            ("HumanName", "given") => Ok(Some(TypeInfo {
                type_name: "String".to_string(),
                singleton: Some(false),
                is_empty: Some(false),
                namespace: Some("System".to_string()),
                name: Some("String".to_string()),
            })),
            _ => Ok(None),
        }
    }

    fn of_type(&self, type_info: &TypeInfo, target_type: &str) -> Option<TypeInfo> {
        // Direct type match
        if type_info.type_name == target_type {
            return Some(type_info.clone());
        }

        // Name match
        if let Some(ref name) = type_info.name {
            if name == target_type {
                return Some(type_info.clone());
            }
            // Check type hierarchy using is_type_derived_from
            if self.is_type_derived_from(name, target_type) {
                return Some(type_info.clone());
            }
        }

        // Check if type_name derives from target_type
        if self.is_type_derived_from(&type_info.type_name, target_type) {
            return Some(type_info.clone());
        }

        None
    }

    fn is_type_derived_from(&self, derived_type: &str, base_type: &str) -> bool {
        if derived_type == base_type {
            return true;
        }

        // Minimal type hierarchy for testing - in real implementation this comes from schemas
        matches!(
            (derived_type, base_type),
            ("code" | "id" | "uri", "string")
                | ("Patient", "DomainResource")
                | ("DomainResource", "Resource")
        )
    }

    fn get_element_names(&self, parent_type: &TypeInfo) -> Vec<String> {
        match parent_type
            .name
            .as_deref()
            .unwrap_or(&parent_type.type_name)
        {
            "Patient" => vec![
                "id".to_string(),
                "name".to_string(),
                "gender".to_string(),
                "birthDate".to_string(),
            ],
            "HumanName" => vec!["given".to_string(), "family".to_string(), "use".to_string()],
            "Observation" => vec![
                "id".to_string(),
                "status".to_string(),
                "code".to_string(),
                "value".to_string(),
                "subject".to_string(),
            ],
            _ => Vec::new(),
        }
    }

    async fn get_children_type(&self, parent_type: &TypeInfo) -> Result<Option<TypeInfo>> {
        if parent_type.singleton.unwrap_or(true) {
            Ok(None)
        } else {
            Ok(Some(TypeInfo {
                type_name: parent_type.type_name.clone(),
                singleton: Some(true),
                is_empty: Some(false),
                namespace: parent_type.namespace.clone(),
                name: parent_type.name.clone(),
            }))
        }
    }

    async fn get_elements(&self, type_name: &str) -> Result<Vec<ElementInfo>> {
        match type_name {
            "Patient" => Ok(vec![
                ElementInfo {
                    name: "id".to_string(),
                    element_type: "id".to_string(),
                    documentation: Some("Logical id of this artifact".to_string()),
                },
                ElementInfo {
                    name: "name".to_string(),
                    element_type: "HumanName[]".to_string(),
                    documentation: Some("A name associated with the patient".to_string()),
                },
            ]),
            _ => Ok(Vec::new()),
        }
    }

    async fn get_resource_types(&self) -> Result<Vec<String>> {
        Ok(vec![
            "Patient".to_string(),
            "Observation".to_string(),
            "Practitioner".to_string(),
            "Organization".to_string(),
        ])
    }

    async fn get_complex_types(&self) -> Result<Vec<String>> {
        Ok(vec![
            "HumanName".to_string(),
            "Address".to_string(),
            "ContactPoint".to_string(),
            "CodeableConcept".to_string(),
            "Quantity".to_string(),
        ])
    }

    async fn get_primitive_types(&self) -> Result<Vec<String>> {
        Ok(vec![
            "Boolean".to_string(),
            "String".to_string(),
            "Integer".to_string(),
            "Decimal".to_string(),
            "Date".to_string(),
            "DateTime".to_string(),
            "Time".to_string(),
        ])
    }

    async fn get_choice_types(
        &self,
        parent_type: &str,
        property_name: &str,
    ) -> Result<Option<Vec<ChoiceTypeInfo>>> {
        match (parent_type, property_name) {
            ("Observation", "value") => Ok(Some(vec![
                ChoiceTypeInfo {
                    suffix: "String".to_string(),
                    type_name: "string".to_string(),
                },
                ChoiceTypeInfo {
                    suffix: "Integer".to_string(),
                    type_name: "integer".to_string(),
                },
                ChoiceTypeInfo {
                    suffix: "Boolean".to_string(),
                    type_name: "boolean".to_string(),
                },
                ChoiceTypeInfo {
                    suffix: "Quantity".to_string(),
                    type_name: "Quantity".to_string(),
                },
                ChoiceTypeInfo {
                    suffix: "CodeableConcept".to_string(),
                    type_name: "CodeableConcept".to_string(),
                },
            ])),
            _ => Ok(None),
        }
    }

    async fn get_union_types(&self, type_info: &TypeInfo) -> Result<Option<Vec<TypeInfo>>> {
        match type_info.type_name.as_str() {
            "Union" | "Choice" => Ok(Some(vec![
                TypeInfo {
                    type_name: "String".to_string(),
                    singleton: Some(true),
                    is_empty: Some(false),
                    namespace: Some("System".to_string()),
                    name: Some("String".to_string()),
                },
                TypeInfo {
                    type_name: "Integer".to_string(),
                    singleton: Some(true),
                    is_empty: Some(false),
                    namespace: Some("System".to_string()),
                    name: Some("Integer".to_string()),
                },
            ])),
            _ => Ok(None),
        }
    }

    fn is_union_type(&self, type_info: &TypeInfo) -> bool {
        matches!(type_info.type_name.as_str(), "Union" | "Choice")
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

// === IntoEvaluationResult implementations ===

impl IntoEvaluationResult for TypeInfo {
    fn to_evaluation_result(&self) -> EvaluationResult {
        // Convert TypeInfo to an object representation
        let mut map = std::collections::HashMap::new();

        map.insert(
            "type_name".to_string(),
            self.type_name.to_evaluation_result(),
        );

        if let Some(singleton) = self.singleton {
            map.insert("singleton".to_string(), singleton.to_evaluation_result());
        }

        if let Some(is_empty) = self.is_empty {
            map.insert("is_empty".to_string(), is_empty.to_evaluation_result());
        }

        if let Some(ref namespace) = self.namespace {
            map.insert("namespace".to_string(), namespace.to_evaluation_result());
        }

        if let Some(ref name) = self.name {
            map.insert("name".to_string(), name.to_evaluation_result());
        }

        let type_info = if let Some(ref namespace) = self.namespace {
            Some(TypeInfoResult::new(namespace, &self.type_name))
        } else {
            Some(TypeInfoResult::system(&self.type_name))
        };

        EvaluationResult::Object { map, type_info }
    }
}

impl IntoEvaluationResult for ElementInfo {
    fn to_evaluation_result(&self) -> EvaluationResult {
        let mut map = std::collections::HashMap::new();

        map.insert("name".to_string(), self.name.to_evaluation_result());
        map.insert(
            "element_type".to_string(),
            self.element_type.to_evaluation_result(),
        );

        if let Some(ref documentation) = self.documentation {
            map.insert(
                "documentation".to_string(),
                documentation.to_evaluation_result(),
            );
        }

        EvaluationResult::typed_object(map, "FHIR", "ElementInfo")
    }
}

impl IntoEvaluationResult for ChoiceTypeInfo {
    fn to_evaluation_result(&self) -> EvaluationResult {
        let mut map = std::collections::HashMap::new();

        map.insert("suffix".to_string(), self.suffix.to_evaluation_result());
        map.insert(
            "type_name".to_string(),
            self.type_name.to_evaluation_result(),
        );

        EvaluationResult::typed_object(map, "FHIR", "ChoiceTypeInfo")
    }
}

impl IntoEvaluationResult for FhirVersion {
    fn to_evaluation_result(&self) -> EvaluationResult {
        EvaluationResult::string(self.to_string())
    }
}

/// Lightweight ModelProvider wrapper that delegates to a full provider
/// but excludes validation-related functionality to break circular dependencies.
///
/// This provider is used in scenarios where we need basic type information
/// without profile validation capabilities, preventing circular dependencies
/// between ModelProvider and FhirPathEvaluator.
#[derive(Debug, Clone)]
pub struct LiteModelProvider {
    /// The underlying full model provider
    inner: std::sync::Arc<dyn ModelProvider>,
}

impl LiteModelProvider {
    /// Create a new lite provider wrapping a full provider
    pub fn new(inner: std::sync::Arc<dyn ModelProvider>) -> Self {
        Self { inner }
    }

    /// Get reference to the underlying provider
    pub fn inner(&self) -> &dyn ModelProvider {
        self.inner.as_ref()
    }

    /// Unwrap to get the underlying provider
    pub fn into_inner(self) -> std::sync::Arc<dyn ModelProvider> {
        self.inner
    }

    /// Check if this provider supports enhanced validation
    /// (always returns false for lite provider)
    pub fn supports_validation(&self) -> bool {
        false
    }
}

#[async_trait]
impl ModelProvider for LiteModelProvider {
    async fn get_type(&self, type_name: &str) -> Result<Option<TypeInfo>> {
        self.inner.get_type(type_name).await
    }

    async fn get_element_type(
        &self,
        parent_type: &TypeInfo,
        property_name: &str,
    ) -> Result<Option<TypeInfo>> {
        self.inner
            .get_element_type(parent_type, property_name)
            .await
    }

    fn of_type(&self, type_info: &TypeInfo, target_type: &str) -> Option<TypeInfo> {
        self.inner.of_type(type_info, target_type)
    }

    fn get_element_names(&self, parent_type: &TypeInfo) -> Vec<String> {
        self.inner.get_element_names(parent_type)
    }

    async fn get_children_type(&self, parent_type: &TypeInfo) -> Result<Option<TypeInfo>> {
        self.inner.get_children_type(parent_type).await
    }

    async fn get_elements(&self, type_name: &str) -> Result<Vec<ElementInfo>> {
        self.inner.get_elements(type_name).await
    }

    async fn get_resource_types(&self) -> Result<Vec<String>> {
        self.inner.get_resource_types().await
    }

    async fn get_complex_types(&self) -> Result<Vec<String>> {
        self.inner.get_complex_types().await
    }

    async fn get_primitive_types(&self) -> Result<Vec<String>> {
        self.inner.get_primitive_types().await
    }

    async fn resource_type_exists(&self, resource_type: &str) -> Result<bool> {
        self.inner.resource_type_exists(resource_type).await
    }

    async fn get_fhir_version(&self) -> Result<FhirVersion> {
        self.inner.get_fhir_version().await
    }

    fn is_type_derived_from(&self, derived_type: &str, base_type: &str) -> bool {
        self.inner.is_type_derived_from(derived_type, base_type)
    }

    async fn get_choice_types(
        &self,
        parent_type: &str,
        property_name: &str,
    ) -> Result<Option<Vec<ChoiceTypeInfo>>> {
        self.inner
            .get_choice_types(parent_type, property_name)
            .await
    }

    async fn get_union_types(&self, type_info: &TypeInfo) -> Result<Option<Vec<TypeInfo>>> {
        self.inner.get_union_types(type_info).await
    }

    fn is_union_type(&self, type_info: &TypeInfo) -> bool {
        self.inner.is_union_type(type_info)
    }
}
