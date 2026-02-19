//! FHIRPath evaluator trait for dependency injection
//!
//! This module provides the abstract evaluator interface that enables
//! fhirschema and other libraries to use FHIRPath evaluation with enhanced
//! validation capabilities through ModelProvider injection.

use async_trait::async_trait;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;

use crate::error::Result;
use crate::evaluation::EvaluationResult;
use crate::provider::ModelProvider;

/// Variables for FHIRPath evaluation context (Arc-wrapped JSON values to avoid deep cloning)
pub type JsonVariables = HashMap<String, Arc<JsonValue>>;

/// Compiled FHIRPath expression for reuse
#[derive(Debug, Clone)]
pub struct CompiledExpression {
    /// The original expression string
    pub expression: String,
    /// Internal representation (implementation-specific)
    pub compiled_form: String,
    /// Whether the expression is valid
    pub is_valid: bool,
}

impl CompiledExpression {
    /// Create a new compiled expression
    pub fn new(expression: String, compiled_form: String, is_valid: bool) -> Self {
        Self {
            expression,
            compiled_form,
            is_valid,
        }
    }

    /// Create an invalid expression
    pub fn invalid(expression: String, error: String) -> Self {
        Self {
            expression,
            compiled_form: error,
            is_valid: false,
        }
    }
}

/// Validation result for FHIRPath expressions and constraints
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    /// Validation errors if any
    pub errors: Vec<ValidationError>,
    /// Warnings that don't prevent validation
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create a failed validation result with errors
    pub fn with_errors(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Add a warning to the validation result
    pub fn with_warning(mut self, warning: ValidationWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add an error to the validation result
    pub fn with_error(mut self, error: ValidationError) -> Self {
        self.is_valid = false;
        self.errors.push(error);
        self
    }
}

/// Validation error details
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Error message
    pub message: String,
    /// Error code if available
    pub code: Option<String>,
    /// Location in the expression or resource
    pub location: Option<String>,
    /// Severity level
    pub severity: ErrorSeverity,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(message: String) -> Self {
        Self {
            message,
            code: None,
            location: None,
            severity: ErrorSeverity::Error,
        }
    }

    /// Create with code
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// Create with location
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
}

/// Validation warning details
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    /// Warning message
    pub message: String,
    /// Warning code if available
    pub code: Option<String>,
    /// Location in the expression or resource
    pub location: Option<String>,
}

impl ValidationWarning {
    /// Create a new validation warning
    pub fn new(message: String) -> Self {
        Self {
            message,
            code: None,
            location: None,
        }
    }

    /// Create with code
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// Create with location
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Fatal error - stops processing
    Fatal,
    /// Error - validation fails
    Error,
    /// Warning - validation continues
    Warning,
    /// Information - no impact on validation
    Information,
}

/// FHIRPath constraint for validation
#[derive(Debug, Clone)]
pub struct FhirPathConstraint {
    /// Constraint identifier
    pub key: String,
    /// Human-readable description
    pub description: String,
    /// FHIRPath expression to evaluate
    pub expression: String,
    /// Severity if constraint fails
    pub severity: ErrorSeverity,
    /// Whether this constraint is required
    pub required: bool,
}

impl FhirPathConstraint {
    /// Create a new constraint
    pub fn new(key: String, description: String, expression: String) -> Self {
        Self {
            key,
            description,
            expression,
            severity: ErrorSeverity::Error,
            required: true,
        }
    }

    /// Set severity level
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Set as optional constraint
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

/// Abstract FHIRPath evaluator interface
///
/// This trait enables dependency injection of FHIRPath evaluators into
/// libraries like fhirschema, allowing them to use enhanced validation
/// capabilities without circular dependencies.
#[async_trait]
pub trait FhirPathEvaluator: Send + Sync {
    /// Core evaluation method
    ///
    /// Evaluates a FHIRPath expression against the given context.
    ///
    /// # Arguments
    /// * `expression` - The FHIRPath expression to evaluate
    /// * `context` - The JSON context for evaluation (usually a FHIR resource)
    ///
    /// # Returns
    /// The evaluation result or an error
    async fn evaluate(&self, expression: &str, context: Arc<JsonValue>)
    -> Result<EvaluationResult>;

    /// Evaluate with variables
    ///
    /// Evaluates a FHIRPath expression with additional variable context.
    ///
    /// # Arguments
    /// * `expression` - The FHIRPath expression to evaluate
    /// * `context` - The JSON context for evaluation
    /// * `variables` - Additional variables available during evaluation
    ///
    /// # Returns
    /// The evaluation result or an error
    async fn evaluate_with_variables(
        &self,
        expression: &str,
        context: Arc<JsonValue>,
        variables: &JsonVariables,
    ) -> Result<EvaluationResult>;

    /// Compile an expression for reuse
    ///
    /// Pre-compiles a FHIRPath expression for efficient repeated evaluation.
    ///
    /// # Arguments
    /// * `expression` - The FHIRPath expression to compile
    ///
    /// # Returns
    /// A compiled expression that can be reused
    async fn compile(&self, expression: &str) -> Result<CompiledExpression>;

    /// Validate expression syntax
    ///
    /// Checks if a FHIRPath expression is syntactically valid.
    ///
    /// # Arguments
    /// * `expression` - The FHIRPath expression to validate
    ///
    /// # Returns
    /// Validation result with any syntax errors
    async fn validate_expression(&self, expression: &str) -> Result<ValidationResult>;

    /// Get the ModelProvider for this evaluator
    ///
    /// Provides access to the injected ModelProvider for type information
    /// and validation capabilities.
    ///
    /// # Returns
    /// Reference to the ModelProvider
    fn model_provider(&self) -> &dyn ModelProvider;

    /// Get the ValidationProvider for this evaluator (if available)
    ///
    /// Provides access to enhanced validation capabilities for profile
    /// validation and conformance checking. Returns None if the evaluator
    /// was created without validation provider support.
    ///
    /// # Returns
    /// Optional reference to the ValidationProvider
    fn validation_provider(&self) -> Option<&dyn ValidationProvider> {
        // Default implementation returns None - override in concrete evaluators
        None
    }

    /// Evaluate a FHIRPath constraint expression and return whether it is satisfied.
    ///
    /// This is an optimized path for constraint validation that avoids the
    /// expensive conversion of the internal evaluation result to the external
    /// `EvaluationResult` type. Per FHIR spec, a constraint is satisfied when:
    /// - The result is empty (constraint not applicable)
    /// - The result is `Boolean(true)`
    /// - The result is any non-boolean value (truthy)
    ///
    /// Only `Boolean(false)` means the constraint is violated.
    ///
    /// The default implementation delegates to `evaluate_with_variables` and
    /// checks `is_constraint_satisfied()` on the result. Concrete evaluators
    /// can override this to skip the result conversion entirely.
    async fn evaluate_constraint_with_variables(
        &self,
        expression: &str,
        context: Arc<JsonValue>,
        variables: &JsonVariables,
    ) -> Result<bool> {
        let result = self
            .evaluate_with_variables(expression, context, variables)
            .await?;
        Ok(result.is_constraint_satisfied())
    }

    /// Validate FHIR constraints
    ///
    /// Evaluates multiple FHIRPath constraints against a resource for
    /// comprehensive validation.
    ///
    /// # Arguments
    /// * `resource` - The FHIR resource to validate
    /// * `constraints` - The FHIRPath constraints to evaluate
    ///
    /// # Returns
    /// Validation result with any constraint violations
    async fn validate_constraints(
        &self,
        resource: Arc<JsonValue>,
        constraints: &[FhirPathConstraint],
    ) -> Result<ValidationResult>;

    /// Evaluate compiled expression
    ///
    /// Evaluates a pre-compiled expression for better performance.
    ///
    /// # Arguments
    /// * `compiled` - The compiled expression
    /// * `context` - The JSON context for evaluation
    ///
    /// # Returns
    /// The evaluation result or an error
    async fn evaluate_compiled(
        &self,
        compiled: &CompiledExpression,
        context: Arc<JsonValue>,
    ) -> Result<EvaluationResult> {
        // Default implementation falls back to regular evaluation
        self.evaluate(&compiled.expression, context).await
    }

    /// Evaluate compiled expression with variables
    ///
    /// Evaluates a pre-compiled expression with variables for better performance.
    ///
    /// # Arguments
    /// * `compiled` - The compiled expression
    /// * `context` - The JSON context for evaluation
    /// * `variables` - Additional variables available during evaluation
    ///
    /// # Returns
    /// The evaluation result or an error
    async fn evaluate_compiled_with_variables(
        &self,
        compiled: &CompiledExpression,
        context: Arc<JsonValue>,
        variables: &JsonVariables,
    ) -> Result<EvaluationResult> {
        // Default implementation falls back to regular evaluation
        self.evaluate_with_variables(&compiled.expression, context, variables)
            .await
    }

    /// Check if the evaluator supports a specific feature
    ///
    /// Allows callers to check for optional features before using them.
    ///
    /// # Arguments
    /// * `feature` - The feature name to check
    ///
    /// # Returns
    /// True if the feature is supported
    fn supports_feature(&self, feature: &str) -> bool {
        // Default implementation - override in concrete evaluators
        matches!(feature, "compilation" | "variables" | "constraints")
    }
}

/// Factory trait for creating FHIRPath evaluators
///
/// Enables creation of evaluators with dependency injection.
#[async_trait]
pub trait FhirPathEvaluatorFactory {
    /// Create a new evaluator with the given ModelProvider
    ///
    /// # Arguments
    /// * `model_provider` - The ModelProvider to inject for type information
    ///
    /// # Returns
    /// A new evaluator instance
    async fn create_evaluator(
        &self,
        model_provider: Arc<dyn ModelProvider>,
    ) -> Result<Arc<dyn FhirPathEvaluator>>;
}

/// Minimal validation interface - wraps existing ModelProvider
///
/// This trait provides a simple validation interface without creating
/// circular dependencies between ModelProvider and FhirPathEvaluator.
/// It leverages existing ModelProvider functionality for maximum reuse.
#[async_trait]
pub trait ValidationProvider: Send + Sync {
    /// Check if a resource conforms to a profile
    ///
    /// This method checks if a resource conforms to the given profile URL
    /// by validating constraints and structural requirements.
    ///
    /// # Arguments
    /// * `resource` - The FHIR resource to validate
    /// * `profile_url` - The canonical URL of the profile to validate against
    ///
    /// # Returns
    /// True if the resource conforms to the profile
    async fn validate(&self, resource: &JsonValue, profile_url: &str) -> Result<bool>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiled_expression_creation() {
        let expr = CompiledExpression::new(
            "Patient.name".to_string(),
            "compiled_form".to_string(),
            true,
        );
        assert!(expr.is_valid);
        assert_eq!(expr.expression, "Patient.name");
    }

    #[test]
    fn test_validation_result() {
        let result = ValidationResult::success();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());

        let error = ValidationError::new("Test error".to_string());
        let result = ValidationResult::with_errors(vec![error]);
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_constraint_creation() {
        let constraint = FhirPathConstraint::new(
            "test-1".to_string(),
            "Test constraint".to_string(),
            "Patient.name.exists()".to_string(),
        );
        assert!(constraint.required);
        assert_eq!(constraint.severity, ErrorSeverity::Error);

        let optional_constraint = constraint.optional().with_severity(ErrorSeverity::Warning);
        assert!(!optional_constraint.required);
        assert_eq!(optional_constraint.severity, ErrorSeverity::Warning);
    }
}
