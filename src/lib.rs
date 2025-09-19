//! Core FHIR model abstractions and ModelProvider trait
//!
//! This crate provides the foundational types and traits for FHIR model integration
//! with FHIRPath engines. It serves as an intermediate dependency to break circular
//! dependencies between FHIRPath implementations and FHIR schema libraries.
//!
//! # Architecture
//!
//! The crate is designed around the `ModelProvider` trait, which abstracts access
//! to FHIR model information including:
//!
//! - Type reflection and metadata
//! - Structure definitions and conformance validation
//! - Constraint definitions and evaluation
//! - Reference resolution capabilities
//!
//! # Example
//!
//! ```rust
//! use octofhir_fhir_model::{TypeInfo, EmptyModelProvider, ModelProvider};
//!
//! // Create type info for a FHIR type
//! let type_info = TypeInfo::system_type("Patient".to_string(), true);
//! println!("Patient type: {:?}", type_info);
//!
//! // Use empty model provider for testing
//! let provider = EmptyModelProvider::default();
//! println!("Provider: {:?}", provider);
//! ```

#![warn(missing_docs)]

pub mod error;
pub mod evaluation;
pub mod evaluator;
pub mod fhir_traits;
pub mod provider;
pub mod terminology;

// Re-export core types
pub use error::{ModelError, Result};
pub use evaluation::{
    EvaluationResult, IntoEvaluationResult, TypeInfoResult, convert_value_to_evaluation_result,
};
pub use evaluator::{
    CompiledExpression, ErrorSeverity, FhirPathConstraint, FhirPathEvaluator,
    FhirPathEvaluatorFactory, ValidationError, ValidationProvider, ValidationResult,
    ValidationWarning, Variables,
};
pub use fhir_traits::{
    BackboneElement, ChoiceElement, FhirPrimitive, FhirReference, FhirResourceMetadata, ToFhirJson,
};
pub use provider::{
    ElementInfo, EmptyModelProvider, FhirVersion, LiteModelProvider, ModelProvider, TypeInfo,
};
pub use terminology::{
    ConnectionStatus, EquivalenceLevel, ExpansionParameter, ExpansionParameters,
    NoOpTerminologyProvider, TerminologyProvider, TranslationResult, TranslationTarget,
    ValueSetConcept, ValueSetExpansion,
};

#[cfg(feature = "http-client")]
pub use terminology::HttpTerminologyProvider;

/// Version information for this crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
