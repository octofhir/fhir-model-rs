//! Error types for FHIR model operations

/// Result type for FHIR model operations
pub type Result<T> = std::result::Result<T, ModelError>;

/// Error types for FHIR model operations
#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    /// Type not found in model
    #[error("Type not found: {type_name}")]
    TypeNotFound { type_name: String },

    /// Property not found on type
    #[error("Property '{property}' not found on type '{type_name}'")]
    PropertyNotFound { type_name: String, property: String },

    /// Schema loading error
    #[error("Schema loading error: {message}")]
    SchemaLoadError { message: String },

    /// Validation error
    #[error("Validation error: {message}")]
    ValidationError { message: String },

    /// Constraint evaluation error
    #[error("Constraint evaluation error: {constraint_key}: {message}")]
    ConstraintError {
        constraint_key: String,
        message: String,
    },

    /// Reference resolution error
    #[error("Reference resolution error: {reference}: {message}")]
    ReferenceError { reference: String, message: String },

    /// Type incompatibility error
    #[error("Type incompatibility: expected {expected}, got {actual}")]
    TypeIncompatibility { expected: String, actual: String },

    /// Boxing/unboxing error
    #[error("Boxing error: {message}")]
    BoxingError { message: String },

    /// Network or I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON parsing error
    #[cfg(feature = "serde")]
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Generic error with message
    #[error("Model error: {message}")]
    Generic { message: String },
}

impl ModelError {
    /// Create a type not found error
    pub fn type_not_found(type_name: impl Into<String>) -> Self {
        Self::TypeNotFound {
            type_name: type_name.into(),
        }
    }

    /// Create a property not found error
    pub fn property_not_found(type_name: impl Into<String>, property: impl Into<String>) -> Self {
        Self::PropertyNotFound {
            type_name: type_name.into(),
            property: property.into(),
        }
    }

    /// Create a schema loading error
    pub fn schema_load_error(message: impl Into<String>) -> Self {
        Self::SchemaLoadError {
            message: message.into(),
        }
    }

    /// Create a validation error
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
        }
    }

    /// Create a constraint error
    pub fn constraint_error(constraint_key: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ConstraintError {
            constraint_key: constraint_key.into(),
            message: message.into(),
        }
    }

    /// Create a reference error
    pub fn reference_error(reference: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ReferenceError {
            reference: reference.into(),
            message: message.into(),
        }
    }

    /// Create a type incompatibility error
    pub fn type_incompatibility(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::TypeIncompatibility {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Create a boxing error
    pub fn boxing_error(message: impl Into<String>) -> Self {
        Self::BoxingError {
            message: message.into(),
        }
    }

    /// Create a generic error
    pub fn generic(message: impl Into<String>) -> Self {
        Self::Generic {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = ModelError::type_not_found("Patient");
        assert!(matches!(error, ModelError::TypeNotFound { .. }));
        assert_eq!(error.to_string(), "Type not found: Patient");

        let error = ModelError::property_not_found("Patient", "name");
        assert!(matches!(error, ModelError::PropertyNotFound { .. }));
        assert_eq!(
            error.to_string(),
            "Property 'name' not found on type 'Patient'"
        );
    }

    #[test]
    fn test_error_chain() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let model_error = ModelError::from(io_error);
        assert!(matches!(model_error, ModelError::IoError(_)));
    }
}
