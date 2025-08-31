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
//! use octofhir_fhir_model::{ModelProvider, TypeReflectionInfo, FhirVersion};
//!
//! // ModelProvider implementations provide type information
//! fn example_usage(provider: &dyn ModelProvider) {
//!     if let Some(type_info) = provider.get_type_reflection("Patient") {
//!         println!("Patient type: {:?}", type_info);
//!     }
//! }
//! ```

#![warn(missing_docs)]

pub mod boxing;
pub mod conformance;
pub mod constraints;
pub mod error;
pub mod fhirpath_engine;
pub mod provider;
pub mod reflection;

// Re-export core types
pub use boxing::{BoxedFhirPathValue, Extension, PrimitiveExtension, SourceLocation};
pub use conformance::{
    ConformanceResult, ConformanceViolation, ConformanceWarning, ViolationSeverity,
};
pub use constraints::{ConstraintInfo, ConstraintResult, ConstraintViolation};
pub use error::{ModelError, Result};
pub use fhirpath_engine::{
    BatchConstraintResult, BatchEvaluationMetrics, FhirPathEngine, FhirPathEngineCapabilities,
    FhirPathEngineFactory, FhirPathEvaluationConfig, FhirPathEvaluationContext,
};
pub use provider::{
    FhirVersion, ModelProvider, PolymorphicTypeInfo, ResolutionContext, SearchParameter,
    StructureDefinition, ValueReflection,
};
pub use reflection::{
    ElementInfo, TupleElementInfo, TypeHierarchy, TypeReflectionInfo, TypeSuggestion,
};

/// Version information for this crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
