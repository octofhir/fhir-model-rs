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
//! use octofhir_fhir_model::{TypeReflectionInfo, ChoiceTypeDefinition};
//!
//! // Create type reflection for a FHIR type
//! let type_info = TypeReflectionInfo::simple_type("FHIR", "Patient");
//! println!("Patient type: {:?}", type_info);
//!
//! // Create choice type definition for value[x]
//! let choice_def = ChoiceTypeDefinition::new("value", "value[x]");
//! println!("Choice type: {:?}", choice_def);
//! ```

#![warn(missing_docs)]

pub mod choice_types;
pub mod conformance;
pub mod constraints;
pub mod error;
pub mod fhirpath_types;
pub mod navigation;
pub mod provider;
pub mod reflection;
pub mod type_system;

// Re-export core types
pub use choice_types::{
    ChoiceConstraint, ChoiceExpansion, ChoiceTypeDefinition, ChoiceTypeOption, ExpandedPath,
    InferenceRule, ResolutionStrategy, TypeCandidate, TypeInference, TypeInferenceResult,
};
pub use conformance::{
    ConformanceResult, ConformanceViolation, ConformanceWarning, ViolationSeverity,
};
pub use constraints::{ConstraintInfo, ConstraintResult, ConstraintViolation};
pub use error::{ModelError, Result};
pub use fhirpath_types::{
    DependencyGraph, ExpressionTypeAnalysis, PerformanceImpact, TypeCheckResult, TypeDependency,
    TypeError, TypeFix, TypeOperation, TypeReference, TypeWarning,
};
pub use navigation::{
    NavigationPath, NavigationResult, NavigationSegment, NavigationStep, NavigationType,
    OptimizationHint, PathValidation, SegmentType,
};
pub use provider::{EmptyModelProvider, FhirVersion, ModelProvider};
pub use reflection::{ElementInfo, TupleElementInfo, TypeReflectionInfo, TypeSuggestion};
pub use type_system::{
    Cardinality, CollectionSemantics, NavigationMetadata, PolymorphicContext,
    PolymorphicResolution, SingletonEvaluation, TypeCompatibilityMatrix, TypeHierarchy,
};

/// Version information for this crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
