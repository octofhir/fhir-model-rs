//! Enhanced value boxing system for metadata preservation
//!

use std::collections::HashMap;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::reflection::TypeReflectionInfo;

/// Enhanced FHIRPath value with comprehensive metadata preservation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoxedFhirPathValue {
    /// The actual value being boxed
    pub value: BoxableValue,
    /// Type information from ModelProvider
    pub type_info: Option<TypeReflectionInfo>,
    /// Primitive extension metadata
    pub primitive_extension: Option<PrimitiveExtension>,
    /// Navigation path for debugging and error reporting
    pub path: Option<String>,
    /// Source location in original resource
    pub source_location: Option<SourceLocation>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Value types that can be boxed
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BoxableValue {
    /// Boolean value
    Boolean(bool),
    /// Integer value (64-bit signed)
    Integer(i64),
    /// Decimal value with arbitrary precision
    Decimal(f64),
    /// String value
    String(String),
    /// Date value (ISO 8601 format)
    Date(String),
    /// DateTime value (ISO 8601 format with timezone)
    DateTime(String),
    /// Time value (ISO 8601 format)
    Time(String),
    /// Quantity value with unit
    Quantity {
        /// Numeric value of the quantity
        value: f64,
        /// Optional unit of measurement
        unit: Option<String>,
    },
    /// Collection of boxed values
    Collection(Vec<BoxedFhirPathValue>),
    /// Complex object (e.g., FHIR resource or complex type)
    Complex(ComplexValue),
    /// Reference to another resource
    Reference(String),
    /// Empty value
    Empty,
}

/// Complex value representation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComplexValue {
    /// Type name of the complex value
    pub type_name: String,
    /// Properties of the complex value
    pub properties: HashMap<String, BoxedFhirPathValue>,
    /// Resource type (if this is a FHIR resource)
    pub resource_type: Option<String>,
    /// Resource ID (if applicable)
    pub id: Option<String>,
}

/// Primitive extension information as per FHIR specification
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrimitiveExtension {
    /// Element ID
    pub id: Option<String>,
    /// Extensions on the primitive element
    pub extensions: Vec<Extension>,
}

/// FHIR Extension representation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Extension {
    /// Extension URL
    pub url: String,
    /// Extension value
    pub value: BoxedFhirPathValue,
}

/// Source location information for debugging and error reporting
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceLocation {
    /// Resource ID where this value originated
    pub resource_id: Option<String>,
    /// Resource type where this value originated
    pub resource_type: Option<String>,
    /// Element path within the resource
    pub element_path: String,
    /// Line number in source document (if available)
    pub line: Option<u32>,
    /// Column number in source document (if available)
    pub column: Option<u32>,
}

impl BoxedFhirPathValue {
    /// Create a new boxed value
    pub fn new(value: BoxableValue) -> Self {
        Self {
            value,
            type_info: None,
            primitive_extension: None,
            path: None,
            source_location: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a boxed boolean
    pub fn boolean(value: bool) -> Self {
        Self::new(BoxableValue::Boolean(value))
    }

    /// Create a boxed integer
    pub fn integer(value: i64) -> Self {
        Self::new(BoxableValue::Integer(value))
    }

    /// Create a boxed decimal
    pub fn decimal(value: f64) -> Self {
        Self::new(BoxableValue::Decimal(value))
    }

    /// Create a boxed string
    pub fn string(value: impl Into<String>) -> Self {
        Self::new(BoxableValue::String(value.into()))
    }

    /// Create a boxed collection
    pub fn collection(values: Vec<BoxedFhirPathValue>) -> Self {
        Self::new(BoxableValue::Collection(values))
    }

    /// Create an empty boxed value
    pub fn empty() -> Self {
        Self::new(BoxableValue::Empty)
    }

    /// Set type information
    pub fn with_type_info(mut self, type_info: TypeReflectionInfo) -> Self {
        self.type_info = Some(type_info);
        self
    }

    /// Set primitive extension
    pub fn with_primitive_extension(mut self, extension: PrimitiveExtension) -> Self {
        self.primitive_extension = Some(extension);
        self
    }

    /// Set navigation path
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Set source location
    pub fn with_source_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Check if the value is empty
    pub fn is_empty(&self) -> bool {
        match &self.value {
            BoxableValue::Empty => true,
            BoxableValue::Collection(items) => items.is_empty(),
            _ => false,
        }
    }

    /// Check if the value is a single item (not a collection)
    pub fn is_single(&self) -> bool {
        match &self.value {
            BoxableValue::Collection(items) => items.len() == 1,
            BoxableValue::Empty => false,
            _ => true,
        }
    }

    /// Get the length of a collection, or 1 for single values, 0 for empty
    pub fn len(&self) -> usize {
        match &self.value {
            BoxableValue::Collection(items) => items.len(),
            BoxableValue::Empty => 0,
            _ => 1,
        }
    }

    /// Convert to boolean if possible
    pub fn as_boolean(&self) -> Option<bool> {
        match &self.value {
            BoxableValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Convert to string if possible
    pub fn as_string(&self) -> Option<String> {
        match &self.value {
            BoxableValue::String(s) => Some(s.clone()),
            BoxableValue::Boolean(b) => Some(b.to_string()),
            BoxableValue::Integer(i) => Some(i.to_string()),
            BoxableValue::Decimal(d) => Some(d.to_string()),
            BoxableValue::Date(d) => Some(d.clone()),
            BoxableValue::DateTime(dt) => Some(dt.clone()),
            BoxableValue::Time(t) => Some(t.clone()),
            _ => None,
        }
    }

    /// Get the first item from a collection, or the value itself if single
    pub fn first(&self) -> Option<&BoxedFhirPathValue> {
        match &self.value {
            BoxableValue::Collection(items) => items.first(),
            BoxableValue::Empty => None,
            _ => Some(self),
        }
    }

    /// Navigate to a property (for complex values)
    pub fn get_property(&self, name: &str) -> Option<&BoxedFhirPathValue> {
        match &self.value {
            BoxableValue::Complex(complex) => complex.properties.get(name),
            _ => None,
        }
    }

    /// Check if a property exists (for complex values)
    pub fn has_property(&self, name: &str) -> bool {
        match &self.value {
            BoxableValue::Complex(complex) => complex.properties.contains_key(name),
            _ => false,
        }
    }

    /// Get all property names (for complex values)
    pub fn property_names(&self) -> Vec<String> {
        match &self.value {
            BoxableValue::Complex(complex) => complex.properties.keys().cloned().collect(),
            _ => Vec::new(),
        }
    }

    /// Check if this value has primitive extensions
    pub fn has_primitive_extensions(&self) -> bool {
        self.primitive_extension
            .as_ref()
            .map(|ext| !ext.extensions.is_empty())
            .unwrap_or(false)
    }

    /// Get primitive extension by URL
    pub fn get_primitive_extension(&self, url: &str) -> Option<&Extension> {
        self.primitive_extension
            .as_ref()?
            .extensions
            .iter()
            .find(|ext| ext.url == url)
    }

    /// Clone with updated path
    pub fn with_updated_path(&self, new_path: impl Into<String>) -> Self {
        let mut cloned = self.clone();
        cloned.path = Some(new_path.into());
        cloned
    }
}

impl ComplexValue {
    /// Create a new complex value
    pub fn new(type_name: impl Into<String>) -> Self {
        Self {
            type_name: type_name.into(),
            properties: HashMap::new(),
            resource_type: None,
            id: None,
        }
    }

    /// Create a FHIR resource value
    pub fn resource(resource_type: impl Into<String>, id: Option<String>) -> Self {
        let resource_type_str = resource_type.into();
        Self {
            type_name: resource_type_str.clone(),
            properties: HashMap::new(),
            resource_type: Some(resource_type_str),
            id,
        }
    }

    /// Add a property
    pub fn with_property(mut self, name: impl Into<String>, value: BoxedFhirPathValue) -> Self {
        self.properties.insert(name.into(), value);
        self
    }

    /// Set the ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl PrimitiveExtension {
    /// Create a new primitive extension
    pub fn new() -> Self {
        Self {
            id: None,
            extensions: Vec::new(),
        }
    }

    /// Set the ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add an extension
    pub fn with_extension(mut self, extension: Extension) -> Self {
        self.extensions.push(extension);
        self
    }

    /// Create from a list of extensions
    pub fn from_extensions(extensions: Vec<Extension>) -> Self {
        Self {
            id: None,
            extensions,
        }
    }
}

impl Extension {
    /// Create a new extension
    pub fn new(url: impl Into<String>, value: BoxedFhirPathValue) -> Self {
        Self {
            url: url.into(),
            value,
        }
    }
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(element_path: impl Into<String>) -> Self {
        Self {
            resource_id: None,
            resource_type: None,
            element_path: element_path.into(),
            line: None,
            column: None,
        }
    }

    /// Set resource information
    pub fn with_resource(
        mut self,
        resource_type: impl Into<String>,
        resource_id: Option<String>,
    ) -> Self {
        self.resource_type = Some(resource_type.into());
        self.resource_id = resource_id;
        self
    }

    /// Set line and column information
    pub fn with_position(mut self, line: u32, column: u32) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }
}

impl Default for PrimitiveExtension {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BoxedFhirPathValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            BoxableValue::Boolean(b) => write!(f, "{b}"),
            BoxableValue::Integer(i) => write!(f, "{i}"),
            BoxableValue::Decimal(d) => write!(f, "{d}"),
            BoxableValue::String(s) => write!(f, "'{s}'"),
            BoxableValue::Date(d) => write!(f, "@{d}"),
            BoxableValue::DateTime(dt) => write!(f, "@{dt}"),
            BoxableValue::Time(t) => write!(f, "@T{t}"),
            BoxableValue::Quantity { value, unit } => match unit {
                Some(u) => write!(f, "{value} '{u}'"),
                None => write!(f, "{value}"),
            },
            BoxableValue::Collection(items) => {
                let item_strings: Vec<String> = items.iter().map(|item| item.to_string()).collect();
                write!(f, "{{{}}}", item_strings.join(", "))
            }
            BoxableValue::Complex(complex) => {
                write!(f, "{}({})", complex.type_name, complex.properties.len())
            }
            BoxableValue::Reference(ref_str) => write!(f, "Reference({ref_str})"),
            BoxableValue::Empty => write!(f, "{{}}"),
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.element_path)?;
        if let Some(resource_type) = &self.resource_type {
            write!(f, " in {resource_type}")?;
            if let Some(resource_id) = &self.resource_id {
                write!(f, "/{resource_id}")?;
            }
        }
        if let (Some(line), Some(column)) = (self.line, self.column) {
            write!(f, " at {line}:{column}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reflection::TypeReflectionInfo;

    #[test]
    fn test_basic_boxing() {
        let boxed_bool = BoxedFhirPathValue::boolean(true);
        assert_eq!(boxed_bool.as_boolean(), Some(true));
        assert!(boxed_bool.is_single());
        assert!(!boxed_bool.is_empty());

        let boxed_string = BoxedFhirPathValue::string("hello");
        assert_eq!(boxed_string.as_string(), Some("hello".to_string()));

        let boxed_empty = BoxedFhirPathValue::empty();
        assert!(boxed_empty.is_empty());
        assert_eq!(boxed_empty.len(), 0);
    }

    #[test]
    fn test_collection_boxing() {
        let items = vec![
            BoxedFhirPathValue::boolean(true),
            BoxedFhirPathValue::string("test"),
        ];
        let collection = BoxedFhirPathValue::collection(items);

        assert_eq!(collection.len(), 2);
        assert!(!collection.is_single());
        assert!(!collection.is_empty());

        let first = collection.first().unwrap();
        assert_eq!(first.as_boolean(), Some(true));
    }

    #[test]
    fn test_complex_value() {
        let patient = ComplexValue::resource("Patient", Some("123".to_string()))
            .with_property("active", BoxedFhirPathValue::boolean(true))
            .with_property("name", BoxedFhirPathValue::string("John Doe"));

        assert_eq!(patient.resource_type.as_deref(), Some("Patient"));
        assert_eq!(patient.id.as_deref(), Some("123"));
        assert_eq!(patient.properties.len(), 2);

        let boxed_patient = BoxedFhirPathValue::new(BoxableValue::Complex(patient));
        assert!(boxed_patient.has_property("active"));
        assert!(boxed_patient.has_property("name"));
        assert!(!boxed_patient.has_property("gender"));

        let active_value = boxed_patient.get_property("active").unwrap();
        assert_eq!(active_value.as_boolean(), Some(true));
    }

    #[test]
    fn test_primitive_extensions() {
        let extension = Extension::new(
            "http://example.com/extension",
            BoxedFhirPathValue::string("extension value"),
        );

        let primitive_ext = PrimitiveExtension::new()
            .with_id("id123")
            .with_extension(extension);

        let boxed_value =
            BoxedFhirPathValue::string("main value").with_primitive_extension(primitive_ext);

        assert!(boxed_value.has_primitive_extensions());
        let ext = boxed_value
            .get_primitive_extension("http://example.com/extension")
            .unwrap();
        assert_eq!(ext.value.as_string(), Some("extension value".to_string()));
    }

    #[test]
    fn test_source_location() {
        let location = SourceLocation::new("Patient.name[0].given[0]")
            .with_resource("Patient", Some("123".to_string()))
            .with_position(15, 10);

        assert_eq!(location.element_path, "Patient.name[0].given[0]");
        assert_eq!(location.resource_type.as_deref(), Some("Patient"));
        assert_eq!(location.line, Some(15));
        assert_eq!(location.column, Some(10));
    }

    #[test]
    fn test_metadata_preservation() {
        let type_info = TypeReflectionInfo::simple_type("FHIR", "string");
        let location = SourceLocation::new("Patient.name");

        let boxed_value = BoxedFhirPathValue::string("John")
            .with_type_info(type_info)
            .with_path("Patient.name[0].given[0]")
            .with_source_location(location)
            .with_metadata("validation", "passed");

        assert!(boxed_value.type_info.is_some());
        assert!(boxed_value.path.is_some());
        assert!(boxed_value.source_location.is_some());
        assert_eq!(boxed_value.metadata.get("validation").unwrap(), "passed");

        // Test path updating
        let updated = boxed_value.with_updated_path("Patient.name[1].given[0]");
        assert_eq!(updated.path.as_deref(), Some("Patient.name[1].given[0]"));
    }

    #[test]
    fn test_display_formatting() {
        assert_eq!(BoxedFhirPathValue::boolean(true).to_string(), "true");
        assert_eq!(BoxedFhirPathValue::integer(42).to_string(), "42");
        assert_eq!(BoxedFhirPathValue::string("hello").to_string(), "'hello'");
        assert_eq!(BoxedFhirPathValue::empty().to_string(), "{}");

        let collection = BoxedFhirPathValue::collection(vec![
            BoxedFhirPathValue::boolean(true),
            BoxedFhirPathValue::integer(42),
        ]);
        assert_eq!(collection.to_string(), "{true, 42}");
    }
}
