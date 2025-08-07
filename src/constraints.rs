//! Constraint definitions and evaluation framework

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Constraint information for FHIR elements
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstraintInfo {
    /// Constraint key (unique identifier)
    pub key: String,
    /// Severity level
    pub severity: ConstraintSeverity,
    /// Human-readable description
    pub human: String,
    /// FHIRPath expression for the constraint
    pub expression: String,
    /// XPath expression (legacy, optional)
    pub xpath: Option<String>,
    /// Source of the constraint (e.g., structure definition URL)
    pub source: Option<String>,
    /// Additional constraint metadata
    pub metadata: HashMap<String, String>,
}

/// Severity levels for constraints
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConstraintSeverity {
    /// Error - constraint must be satisfied
    Error,
    /// Warning - constraint should be satisfied
    Warning,
    /// Information - informational constraint
    Information,
}

/// Result of constraint evaluation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstraintResult {
    /// Whether the constraint was satisfied
    pub success: bool,
    /// Path where the constraint was evaluated
    pub evaluation_path: String,
    /// Result value from the FHIRPath expression (if available)
    pub result_value: Option<ConstraintValue>,
    /// Error message if evaluation failed
    pub error: Option<String>,
    /// Execution time in microseconds
    pub execution_time_us: Option<u64>,
}

/// Value resulting from constraint evaluation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConstraintValue {
    /// Boolean result
    Boolean(bool),
    /// String result
    String(String),
    /// Integer result
    Integer(i64),
    /// Decimal result
    Decimal(f64),
    /// Collection of values
    Collection(Vec<ConstraintValue>),
    /// Empty result
    Empty,
}

/// Violation resulting from failed constraint evaluation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstraintViolation {
    /// Constraint that was violated
    pub constraint_key: String,
    /// Human-readable message
    pub message: String,
    /// Severity of the violation
    pub severity: ConstraintSeverity,
    /// Path where the violation occurred
    pub path: String,
    /// Expected result
    pub expected: Option<String>,
    /// Actual result
    pub actual: Option<String>,
}

/// Statistics for constraint evaluation
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstraintEvaluationStats {
    /// Total constraints evaluated
    pub total_evaluated: u64,
    /// Number of successful evaluations
    pub successful: u64,
    /// Number of failed evaluations
    pub failed: u64,
    /// Number of evaluation errors
    pub errors: u64,
    /// Total execution time in microseconds
    pub total_execution_time_us: u64,
    /// Average execution time per constraint
    pub avg_execution_time_us: f64,
}

impl ConstraintInfo {
    /// Create a new constraint
    pub fn new(
        key: impl Into<String>,
        severity: ConstraintSeverity,
        human: impl Into<String>,
        expression: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            severity,
            human: human.into(),
            expression: expression.into(),
            xpath: None,
            source: None,
            metadata: HashMap::new(),
        }
    }

    /// Create an error constraint
    pub fn error(
        key: impl Into<String>,
        human: impl Into<String>,
        expression: impl Into<String>,
    ) -> Self {
        Self::new(key, ConstraintSeverity::Error, human, expression)
    }

    /// Create a warning constraint
    pub fn warning(
        key: impl Into<String>,
        human: impl Into<String>,
        expression: impl Into<String>,
    ) -> Self {
        Self::new(key, ConstraintSeverity::Warning, human, expression)
    }

    /// Create an informational constraint
    pub fn info(
        key: impl Into<String>,
        human: impl Into<String>,
        expression: impl Into<String>,
    ) -> Self {
        Self::new(key, ConstraintSeverity::Information, human, expression)
    }

    /// Set XPath expression
    pub fn with_xpath(mut self, xpath: impl Into<String>) -> Self {
        self.xpath = Some(xpath.into());
        self
    }

    /// Set source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Validate the constraint definition
    pub fn validate(&self) -> Result<(), String> {
        if self.key.is_empty() {
            return Err("Constraint key cannot be empty".to_string());
        }

        if self.expression.is_empty() {
            return Err("Constraint expression cannot be empty".to_string());
        }

        if self.human.is_empty() {
            return Err("Constraint human description cannot be empty".to_string());
        }

        Ok(())
    }
}

impl ConstraintResult {
    /// Create a successful result
    pub fn success(path: impl Into<String>) -> Self {
        Self {
            success: true,
            evaluation_path: path.into(),
            result_value: Some(ConstraintValue::Boolean(true)),
            error: None,
            execution_time_us: None,
        }
    }

    /// Create a failed result
    pub fn failure(path: impl Into<String>) -> Self {
        Self {
            success: false,
            evaluation_path: path.into(),
            result_value: Some(ConstraintValue::Boolean(false)),
            error: None,
            execution_time_us: None,
        }
    }

    /// Create an error result
    pub fn error(path: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            success: false,
            evaluation_path: path.into(),
            result_value: None,
            error: Some(error.into()),
            execution_time_us: None,
        }
    }

    /// Set result value
    pub fn with_value(mut self, value: ConstraintValue) -> Self {
        self.result_value = Some(value);
        self
    }

    /// Set execution time
    pub fn with_execution_time(mut self, time_us: u64) -> Self {
        self.execution_time_us = Some(time_us);
        self
    }

    /// Check if the result indicates success
    pub fn is_success(&self) -> bool {
        self.success && self.error.is_none()
    }

    /// Get boolean result value
    pub fn as_boolean(&self) -> Option<bool> {
        match &self.result_value {
            Some(ConstraintValue::Boolean(b)) => Some(*b),
            _ => None,
        }
    }
}

impl ConstraintValue {
    /// Check if the value is truthy (non-empty and true-like)
    pub fn is_truthy(&self) -> bool {
        match self {
            ConstraintValue::Boolean(b) => *b,
            ConstraintValue::String(s) => !s.is_empty(),
            ConstraintValue::Integer(i) => *i != 0,
            ConstraintValue::Decimal(d) => *d != 0.0,
            ConstraintValue::Collection(c) => !c.is_empty(),
            ConstraintValue::Empty => false,
        }
    }

    /// Convert to boolean
    pub fn to_boolean(&self) -> bool {
        self.is_truthy()
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            ConstraintValue::Boolean(b) => b.to_string(),
            ConstraintValue::String(s) => s.clone(),
            ConstraintValue::Integer(i) => i.to_string(),
            ConstraintValue::Decimal(d) => d.to_string(),
            ConstraintValue::Collection(c) => {
                let strings: Vec<String> = c.iter().map(|v| v.to_string()).collect();
                format!("[{}]", strings.join(", "))
            }
            ConstraintValue::Empty => "{}".to_string(),
        }
    }
}

impl ConstraintViolation {
    /// Create a new violation
    pub fn new(
        constraint_key: impl Into<String>,
        message: impl Into<String>,
        severity: ConstraintSeverity,
        path: impl Into<String>,
    ) -> Self {
        Self {
            constraint_key: constraint_key.into(),
            message: message.into(),
            severity,
            path: path.into(),
            expected: None,
            actual: None,
        }
    }

    /// Create an error violation
    pub fn error(
        constraint_key: impl Into<String>,
        message: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self::new(constraint_key, message, ConstraintSeverity::Error, path)
    }

    /// Set expected and actual values
    pub fn with_values(mut self, expected: impl Into<String>, actual: impl Into<String>) -> Self {
        self.expected = Some(expected.into());
        self.actual = Some(actual.into());
        self
    }
}

impl ConstraintEvaluationStats {
    /// Create new stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a successful evaluation
    pub fn record_success(&mut self, execution_time_us: u64) {
        self.total_evaluated += 1;
        self.successful += 1;
        self.total_execution_time_us += execution_time_us;
        self.update_average();
    }

    /// Record a failed evaluation
    pub fn record_failure(&mut self, execution_time_us: u64) {
        self.total_evaluated += 1;
        self.failed += 1;
        self.total_execution_time_us += execution_time_us;
        self.update_average();
    }

    /// Record an evaluation error
    pub fn record_error(&mut self, execution_time_us: u64) {
        self.total_evaluated += 1;
        self.errors += 1;
        self.total_execution_time_us += execution_time_us;
        self.update_average();
    }

    /// Update average execution time
    fn update_average(&mut self) {
        if self.total_evaluated > 0 {
            self.avg_execution_time_us =
                self.total_execution_time_us as f64 / self.total_evaluated as f64;
        }
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_evaluated == 0 {
            0.0
        } else {
            self.successful as f64 / self.total_evaluated as f64
        }
    }

    /// Get error rate
    pub fn error_rate(&self) -> f64 {
        if self.total_evaluated == 0 {
            0.0
        } else {
            self.errors as f64 / self.total_evaluated as f64
        }
    }
}

impl std::fmt::Display for ConstraintSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstraintSeverity::Error => write!(f, "error"),
            ConstraintSeverity::Warning => write!(f, "warning"),
            ConstraintSeverity::Information => write!(f, "information"),
        }
    }
}

impl std::fmt::Display for ConstraintViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}: {} ({})",
            self.severity, self.constraint_key, self.message, self.path
        )?;

        if let (Some(expected), Some(actual)) = (&self.expected, &self.actual) {
            write!(f, " (expected: {}, actual: {})", expected, actual)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_info() {
        let constraint = ConstraintInfo::error("pat-1", "Name is required", "name.exists()")
            .with_xpath("//name")
            .with_source("http://example.com/StructureDefinition/Patient")
            .with_metadata("author", "FHIR Team");

        assert_eq!(constraint.key, "pat-1");
        assert_eq!(constraint.severity, ConstraintSeverity::Error);
        assert_eq!(constraint.expression, "name.exists()");
        assert_eq!(constraint.xpath.as_deref(), Some("//name"));
        assert!(constraint.validate().is_ok());
    }

    #[test]
    fn test_constraint_validation() {
        let invalid_constraint = ConstraintInfo::new("", ConstraintSeverity::Error, "", "");
        assert!(invalid_constraint.validate().is_err());

        let valid_constraint = ConstraintInfo::error("test-1", "Test constraint", "true");
        assert!(valid_constraint.validate().is_ok());
    }

    #[test]
    fn test_constraint_result() {
        let success = ConstraintResult::success("Patient.name")
            .with_value(ConstraintValue::Boolean(true))
            .with_execution_time(100);

        assert!(success.is_success());
        assert_eq!(success.as_boolean(), Some(true));
        assert_eq!(success.execution_time_us, Some(100));

        let failure = ConstraintResult::failure("Patient.name");
        assert!(!failure.is_success());
        assert_eq!(failure.as_boolean(), Some(false));

        let error = ConstraintResult::error("Patient.name", "Evaluation failed");
        assert!(!error.is_success());
    }

    #[test]
    fn test_constraint_value() {
        assert!(ConstraintValue::Boolean(true).is_truthy());
        assert!(!ConstraintValue::Boolean(false).is_truthy());
        assert!(ConstraintValue::String("hello".to_string()).is_truthy());
        assert!(!ConstraintValue::String("".to_string()).is_truthy());
        assert!(ConstraintValue::Integer(1).is_truthy());
        assert!(!ConstraintValue::Integer(0).is_truthy());
        assert!(!ConstraintValue::Empty.is_truthy());

        let collection = ConstraintValue::Collection(vec![ConstraintValue::Boolean(true)]);
        assert!(collection.is_truthy());

        let empty_collection = ConstraintValue::Collection(vec![]);
        assert!(!empty_collection.is_truthy());
    }

    #[test]
    fn test_evaluation_stats() {
        let mut stats = ConstraintEvaluationStats::new();

        stats.record_success(100);
        stats.record_failure(200);
        stats.record_error(300);

        assert_eq!(stats.total_evaluated, 3);
        assert_eq!(stats.successful, 1);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.total_execution_time_us, 600);
        assert_eq!(stats.avg_execution_time_us, 200.0);
        assert_eq!(stats.success_rate(), 1.0 / 3.0);
        assert_eq!(stats.error_rate(), 1.0 / 3.0);
    }

    #[test]
    fn test_constraint_violation() {
        let violation = ConstraintViolation::error("pat-1", "Name is required", "Patient.name")
            .with_values("string", "null");

        assert_eq!(violation.constraint_key, "pat-1");
        assert_eq!(violation.severity, ConstraintSeverity::Error);
        assert_eq!(violation.expected.as_deref(), Some("string"));
        assert_eq!(violation.actual.as_deref(), Some("null"));
    }
}
