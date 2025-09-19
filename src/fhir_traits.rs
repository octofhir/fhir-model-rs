//! FHIR-specific traits for enhanced FHIRPath support
//!
//! This module provides traits for FHIR choice elements and resource metadata
//! to enable polymorphic access in FHIRPath expressions.

/// Trait for FHIR choice element types.
///
/// This trait is implemented by generated enum types that represent FHIR choice elements
/// (fields with [x] in the FHIR specification). It provides metadata about the choice
/// element that enables proper polymorphic access in FHIRPath expressions.
///
/// # Example
///
/// For a FHIR field like `Observation.value[x]`, the generated enum would implement:
/// ```rust,ignore
/// impl ChoiceElement for ObservationValue {
///     fn base_name() -> &'static str {
///         "value"
///     }
///
///     fn possible_field_names() -> Vec<&'static str> {
///         vec!["valueQuantity", "valueCodeableConcept", "valueString", ...]
///     }
/// }
/// ```
pub trait ChoiceElement {
    /// Returns the base name of the choice element without the [x] suffix.
    ///
    /// For example, for `value[x]`, this returns "value".
    fn base_name() -> &'static str;

    /// Returns all possible field names that this choice element can manifest as.
    ///
    /// For example, for `value[x]`, this might return:
    /// ["valueQuantity", "valueCodeableConcept", "valueString", ...]
    fn possible_field_names() -> Vec<&'static str>;
}

/// Trait for FHIR resource metadata.
///
/// This trait is implemented by generated FHIR resource structs to provide
/// metadata about the resource's structure, particularly which fields are
/// choice elements. This enables accurate polymorphic field access in FHIRPath.
///
/// # Example
///
/// ```rust,ignore
/// impl FhirResourceMetadata for Observation {
///     fn choice_elements() -> &'static [&'static str] {
///         &["value", "effective", "component.value"]
///     }
/// }
/// ```
pub trait FhirResourceMetadata {
    /// Returns the names of all choice element fields in this resource.
    ///
    /// The returned slice contains the base names (without [x]) of fields
    /// that are choice elements in the FHIR specification.
    fn choice_elements() -> &'static [&'static str];

    /// Get the resource type name for this resource
    fn resource_type() -> &'static str;

    /// Check if a field is a choice element
    fn is_choice_element(field_name: &str) -> bool {
        Self::choice_elements().contains(&field_name)
    }

    /// Get the possible field names for a choice element
    fn get_choice_field_names(base_name: &str) -> Vec<String> {
        if Self::is_choice_element(base_name) {
            // Default implementation - concrete types should override with actual field names
            vec![
                format!("{}String", base_name),
                format!("{}Boolean", base_name),
            ]
        } else {
            vec![]
        }
    }
}

/// Trait for types that can be converted to JSON for FHIRPath processing
pub trait ToFhirJson {
    /// Convert this type to a JSON representation suitable for FHIRPath evaluation
    fn to_fhir_json(&self) -> serde_json::Value;
}

/// Trait for FHIR primitive types that may have extensions
pub trait FhirPrimitive {
    /// Get the primitive value without extensions
    fn primitive_value(&self) -> Option<serde_json::Value>;

    /// Check if this primitive has extensions
    fn has_extensions(&self) -> bool;

    /// Get extensions if any
    fn get_extensions(&self) -> Option<Vec<serde_json::Value>>;
}

/// Trait for FHIR reference types
pub trait FhirReference {
    /// Get the reference string (e.g., "Patient/123")
    fn reference(&self) -> Option<&str>;

    /// Get the referenced resource type
    fn referenced_type(&self) -> Option<&str> {
        self.reference()?.split('/').next()
    }

    /// Get the referenced resource ID
    fn referenced_id(&self) -> Option<&str> {
        self.reference()?.split('/').nth(1)
    }

    /// Get the display name if available
    fn display(&self) -> Option<&str>;
}

/// Trait for FHIR backbone elements
pub trait BackboneElement {
    /// Get the element ID
    fn element_id(&self) -> Option<&str>;

    /// Get modifierExtensions if any
    fn modifier_extensions(&self) -> Option<Vec<serde_json::Value>>;

    /// Check if this element has modifier extensions
    fn has_modifier_extensions(&self) -> bool {
        self.modifier_extensions()
            .is_some_and(|ext| !ext.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example implementation for testing
    struct TestObservationValue;

    impl ChoiceElement for TestObservationValue {
        fn base_name() -> &'static str {
            "value"
        }

        fn possible_field_names() -> Vec<&'static str> {
            vec!["valueString", "valueQuantity", "valueBoolean"]
        }
    }

    struct TestObservation;

    impl FhirResourceMetadata for TestObservation {
        fn choice_elements() -> &'static [&'static str] {
            &["value", "effective"]
        }

        fn resource_type() -> &'static str {
            "Observation"
        }
    }

    #[test]
    fn test_choice_element() {
        assert_eq!(TestObservationValue::base_name(), "value");
        assert_eq!(
            TestObservationValue::possible_field_names(),
            vec!["valueString", "valueQuantity", "valueBoolean"]
        );
    }

    #[test]
    fn test_resource_metadata() {
        assert_eq!(TestObservation::resource_type(), "Observation");
        assert!(TestObservation::is_choice_element("value"));
        assert!(!TestObservation::is_choice_element("status"));
    }
}
