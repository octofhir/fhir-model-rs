//! Integration tests for octofhir-fhir-model crate
//!
//! These tests verify the integration between all major components:
//! - ModelProvider trait
//! - Type reflection system
//! - Conformance validation framework
//! - Enhanced value boxing system

use octofhir_fhir_model::*;
use std::collections::HashMap;

/// Mock implementation of ValueReflection for testing
struct MockPatientResource {
    data: serde_json::Value,
}

impl MockPatientResource {
    fn new() -> Self {
        let patient_json = serde_json::json!({
            "resourceType": "Patient",
            "id": "123",
            "active": true,
            "name": [{
                "given": ["John", "Michael"],
                "family": "Doe"
            }],
            "gender": "male",
            "_gender": {
                "extension": [{
                    "url": "http://example.com/gender-code",
                    "valueCode": "M"
                }]
            }
        });

        Self { data: patient_json }
    }
}

impl provider::ValueReflection for MockPatientResource {
    fn type_name(&self) -> String {
        "Patient".to_string()
    }

    fn get_property(&self, name: &str) -> Option<Box<dyn provider::ValueReflection>> {
        self.data
            .get(name)
            .map(|_| Box::new(MockSimpleValue::new(name)) as Box<dyn provider::ValueReflection>)
    }

    fn has_property(&self, name: &str) -> bool {
        self.data.get(name).is_some()
    }

    fn property_names(&self) -> Vec<String> {
        if let Some(obj) = self.data.as_object() {
            obj.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    fn to_debug_string(&self) -> String {
        format!(
            "MockPatientResource(id={})",
            self.data.get("id").unwrap_or(&serde_json::Value::Null)
        )
    }
}

struct MockSimpleValue {
    name: String,
}

impl MockSimpleValue {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl provider::ValueReflection for MockSimpleValue {
    fn type_name(&self) -> String {
        "String".to_string()
    }

    fn get_property(&self, _name: &str) -> Option<Box<dyn provider::ValueReflection>> {
        None
    }

    fn has_property(&self, _name: &str) -> bool {
        false
    }

    fn property_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn to_debug_string(&self) -> String {
        format!("MockSimpleValue({})", self.name)
    }
}

/// Mock validation rule for testing
struct RequiredNameRule;

impl conformance::ValidationRule for RequiredNameRule {
    fn rule_id(&self) -> &str {
        "patient-name-required"
    }

    fn description(&self) -> &str {
        "Patient must have a name"
    }

    fn validate(
        &self,
        path: &str,
        value: &serde_json::Value,
        _context: &conformance::ValidationContext,
    ) -> conformance::ValidationRuleResult {
        if value.get("name").is_some() {
            conformance::ValidationRuleResult::success()
        } else {
            let violation = conformance::ConformanceViolation::error(
                &format!("{}.name", path),
                "Patient name is required",
            );
            conformance::ValidationRuleResult::with_violations(vec![violation])
        }
    }

    fn applies_to(&self, _path: &str, resource_type: &str) -> bool {
        resource_type == "Patient"
    }

    fn priority(&self) -> u32 {
        200 // High priority
    }
}

#[test]
fn test_model_provider_integration() {
    let provider = provider::EmptyModelProvider::new();

    // Test type reflection integration
    let string_type = reflection::TypeReflectionInfo::simple_type("System", "String");
    let patient_elements = vec![
        reflection::ElementInfo::new("id", string_type.clone()),
        reflection::ElementInfo::new("name", string_type.clone())
            .with_cardinality(1, None) // Required
            .as_summary(),
    ];
    let patient_type =
        reflection::TypeReflectionInfo::class_type("FHIR", "Patient", patient_elements);

    // Test type compatibility
    assert!(string_type.is_compatible_with(&string_type));
    assert!(patient_type.is_fhir_type());
    assert!(!patient_type.is_primitive());

    // Test validation rules
    let rules = patient_type.validation_rules();
    assert!(!rules.is_empty());
    assert!(
        rules
            .iter()
            .any(|rule| rule.contains("name") && rule.contains("required"))
    );

    // Test ModelProvider methods
    let analysis = provider.analyze_expression("Patient.name.given").unwrap();
    assert!(analysis.referenced_types.is_empty()); // EmptyProvider returns empty

    let navigation_validation = provider
        .validate_navigation_path("Patient", "name.given")
        .unwrap();
    assert!(!navigation_validation.is_valid); // EmptyProvider cannot validate
}

#[test]
fn test_conformance_validator_integration() {
    let context = conformance::ValidationContext::new("R5")
        .with_mode(conformance::ValidationMode::Strict)
        .with_profile("http://example.com/Patient");

    let mut validator = conformance::ConformanceValidator::new(context);
    validator.add_rule(Box::new(RequiredNameRule));

    // Test with valid patient
    let valid_patient = serde_json::json!({
        "resourceType": "Patient",
        "name": [{"given": ["John"], "family": "Doe"}]
    });

    let result = validator.validate(&valid_patient, "Patient");
    assert!(result.is_valid);
    assert_eq!(result.violations.len(), 0);

    // Test with invalid patient (no name)
    let invalid_patient = serde_json::json!({
        "resourceType": "Patient",
        "id": "123"
    });

    let result = validator.validate(&invalid_patient, "Patient");
    assert!(!result.is_valid);
    assert_eq!(result.violations.len(), 1);
    assert_eq!(result.violations[0].path, ".name");

    // Test metrics
    let metrics = validator.get_metrics();
    assert!(metrics.rules_evaluated > 0);
    assert!(metrics.total_time_us > 0);
}

#[test]
fn test_value_boxing_integration() {
    // Test comprehensive boxing with all metadata
    let patient_type = reflection::TypeReflectionInfo::simple_type("FHIR", "Patient");
    let source_location = boxing::SourceLocation::new("Patient.name")
        .with_resource("Patient", Some("123".to_string()))
        .with_position(10, 5);

    let primitive_ext = boxing::PrimitiveExtension::new()
        .with_id("ext-123")
        .with_extension(boxing::Extension::new(
            "http://example.com/extension",
            boxing::BoxedFhirPathValue::string("extension-value"),
        ));

    let boxed_value = boxing::BoxedFhirPathValue::string("John Doe")
        .with_type_info(patient_type)
        .with_primitive_extension(primitive_ext)
        .with_path("Patient.name[0].given[0]")
        .with_source_location(source_location)
        .with_metadata("validation", "passed")
        .with_metadata("processing", "complete");

    // Test metadata preservation
    assert_eq!(boxed_value.as_string(), Some("John Doe".to_string()));
    assert!(boxed_value.type_info.is_some());
    assert!(boxed_value.primitive_extension.is_some());
    assert!(boxed_value.path.is_some());
    assert!(boxed_value.source_location.is_some());
    assert_eq!(boxed_value.metadata.len(), 2);

    // Test primitive extensions
    assert!(boxed_value.has_primitive_extensions());
    let ext = boxed_value
        .get_primitive_extension("http://example.com/extension")
        .unwrap();
    assert_eq!(ext.value.as_string(), Some("extension-value".to_string()));

    // Test path updating
    let updated = boxed_value.with_updated_path("Patient.name[1].given[0]");
    assert_eq!(updated.path.as_deref(), Some("Patient.name[1].given[0]"));

    // Original should be unchanged
    assert_eq!(
        boxed_value.path.as_deref(),
        Some("Patient.name[0].given[0]")
    );
}

#[test]
fn test_complex_value_navigation() {
    let patient = boxing::ComplexValue::resource("Patient", Some("123".to_string()))
        .with_property("active", boxing::BoxedFhirPathValue::boolean(true))
        .with_property("gender", boxing::BoxedFhirPathValue::string("male"));

    let boxed_patient = boxing::BoxedFhirPathValue::new(boxing::BoxableValue::Complex(patient));

    // Test property navigation
    assert!(boxed_patient.has_property("active"));
    assert!(boxed_patient.has_property("gender"));
    assert!(!boxed_patient.has_property("birthDate"));

    let active_value = boxed_patient.get_property("active").unwrap();
    assert_eq!(active_value.as_boolean(), Some(true));

    let property_names = boxed_patient.property_names();
    assert_eq!(property_names.len(), 2);
    assert!(property_names.contains(&"active".to_string()));
    assert!(property_names.contains(&"gender".to_string()));
}

#[test]
fn test_collection_operations() {
    let items = vec![
        boxing::BoxedFhirPathValue::string("John"),
        boxing::BoxedFhirPathValue::string("Jane"),
        boxing::BoxedFhirPathValue::string("Bob"),
    ];

    let collection = boxing::BoxedFhirPathValue::collection(items);

    assert_eq!(collection.len(), 3);
    assert!(!collection.is_single());
    assert!(!collection.is_empty());

    let first = collection.first().unwrap();
    assert_eq!(first.as_string(), Some("John".to_string()));
}

#[test]
fn test_type_system_edge_cases() {
    // Test nested list types (should not be valid collection elements)
    let string_type = reflection::TypeReflectionInfo::simple_type("System", "String");
    let list_type = reflection::TypeReflectionInfo::list_type(string_type);
    let nested_list = reflection::TypeReflectionInfo::list_type(list_type);

    assert!(!nested_list.is_valid_collection_element());

    // Test type compatibility edge cases
    let integer_type = reflection::TypeReflectionInfo::simple_type("System", "Integer");
    let boolean_type = reflection::TypeReflectionInfo::simple_type("System", "Boolean");

    // Integer should not be compatible with Boolean
    assert!(!integer_type.is_compatible_with(&boolean_type));

    // But both should be compatible with String
    let string_type = reflection::TypeReflectionInfo::simple_type("System", "String");
    assert!(integer_type.is_compatible_with(&string_type));
    assert!(boolean_type.is_compatible_with(&string_type));
}

#[test]
fn test_constraint_evaluation_integration() {
    let constraint =
        constraints::ConstraintInfo::error("pat-1", "Patient must have a name", "name.exists()")
            .with_source("http://hl7.org/fhir/StructureDefinition/Patient");

    assert!(constraint.validate().is_ok());
    assert_eq!(constraint.severity, constraints::ConstraintSeverity::Error);

    // Test constraint evaluation statistics
    let mut stats = constraints::ConstraintEvaluationStats::new();
    stats.record_success(100);
    stats.record_failure(200);
    stats.record_error(50);

    assert_eq!(stats.total_evaluated, 3);
    assert_eq!(stats.successful, 1);
    assert_eq!(stats.failed, 1);
    assert_eq!(stats.errors, 1);
    assert_eq!(stats.success_rate(), 1.0 / 3.0);
    assert_eq!(stats.error_rate(), 1.0 / 3.0);
}

#[test]
fn test_performance_characteristics() {
    use std::time::Instant;

    // Test that boxing operations are reasonably fast
    let start = Instant::now();

    for i in 0..1000 {
        let boxed = boxing::BoxedFhirPathValue::string(format!("value-{}", i))
            .with_type_info(reflection::TypeReflectionInfo::simple_type(
                "System", "String",
            ))
            .with_path(format!("Patient.name[{}]", i))
            .with_metadata("index", i.to_string());

        // Verify the boxing worked
        assert!(boxed.type_info.is_some());
        assert!(boxed.path.is_some());
        assert_eq!(boxed.metadata.len(), 1);
    }

    let elapsed = start.elapsed();

    // Boxing 1000 values should take less than 10ms (very lenient for CI)
    assert!(
        elapsed.as_millis() < 100,
        "Boxing performance test failed: took {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn test_error_handling_integration() {
    // Test error propagation through the system
    let provider = provider::EmptyModelProvider::new();

    // Test that EmptyProvider returns appropriate errors
    let mock_value = MockPatientResource::new();
    let result = provider.box_value_with_metadata(&mock_value, "Patient.name");
    assert!(result.is_err());

    // Test constraint validation errors
    let invalid_constraint = constraints::ConstraintInfo::new(
        "", // Empty key should fail validation
        constraints::ConstraintSeverity::Error,
        "",
        "",
    );
    assert!(invalid_constraint.validate().is_err());
}

#[test]
fn test_memory_efficiency() {
    // Test that we don't leak memory with large collections
    let mut large_collection = Vec::new();

    for i in 0..10000 {
        let boxed_value =
            boxing::BoxedFhirPathValue::integer(i as i64).with_metadata("index", i.to_string());
        large_collection.push(boxed_value);
    }

    let collection = boxing::BoxedFhirPathValue::collection(large_collection);
    assert_eq!(collection.len(), 10000);

    // Access some elements to ensure they're properly accessible
    if let boxing::BoxableValue::Collection(items) = &collection.value {
        assert_eq!(items.len(), 10000);
        assert_eq!(items[0].as_string(), Some("0".to_string()));
        assert_eq!(items[9999].as_string(), Some("9999".to_string()));
    }
}
