//! Conformance validation framework for FHIR resources

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Import serde_json for JSON validation support

/// Result of conformance validation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConformanceResult {
    /// Whether the resource conforms to the profile
    pub is_valid: bool,
    /// List of validation violations
    pub violations: Vec<ConformanceViolation>,
    /// List of validation warnings
    pub warnings: Vec<ConformanceWarning>,
    /// Profile URL that was validated against
    pub profile_url: String,
    /// Resource type that was validated
    pub resource_type: String,
    /// Validation metadata
    pub metadata: ConformanceMetadata,
}

/// A conformance validation violation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConformanceViolation {
    /// Path to the element that violated the rule
    pub path: String,
    /// Human-readable description of the violation
    pub message: String,
    /// Severity of the violation
    pub severity: ViolationSeverity,
    /// Constraint key that was violated (if applicable)
    pub constraint_key: Option<String>,
    /// Expected value or constraint
    pub expected: Option<String>,
    /// Actual value found
    pub actual: Option<String>,
    /// Location in the source document
    pub location: Option<SourceLocation>,
}

/// A conformance validation warning
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConformanceWarning {
    /// Path to the element
    pub path: String,
    /// Warning message
    pub message: String,
    /// Warning code
    pub code: Option<String>,
    /// Location in the source document
    pub location: Option<SourceLocation>,
}

/// Severity levels for validation violations
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViolationSeverity {
    /// Fatal error - validation cannot continue
    Fatal,
    /// Error - resource does not conform
    Error,
    /// Warning - potential issue but not a strict violation
    Warning,
    /// Information - helpful information for developers
    Information,
}

/// Metadata about conformance validation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConformanceMetadata {
    /// Validation engine version
    pub validator_version: String,
    /// Time taken for validation in milliseconds
    pub validation_time_ms: u64,
    /// Number of elements validated
    pub elements_validated: usize,
    /// Number of constraints evaluated
    pub constraints_evaluated: usize,
    /// Additional metadata
    pub additional: HashMap<String, String>,
}

/// Source location information for validation issues
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceLocation {
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub column: u32,
    /// Character offset in the source
    pub char_offset: Option<usize>,
    /// Length of the problematic content
    pub length: Option<usize>,
}

impl ConformanceResult {
    /// Create an empty conformance result
    pub fn empty() -> Self {
        Self {
            is_valid: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            profile_url: String::new(),
            resource_type: String::new(),
            metadata: ConformanceMetadata::default(),
        }
    }

    /// Create a new conformance result
    pub fn new(profile_url: impl Into<String>, resource_type: impl Into<String>) -> Self {
        Self {
            is_valid: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            profile_url: profile_url.into(),
            resource_type: resource_type.into(),
            metadata: ConformanceMetadata::default(),
        }
    }

    /// Add a violation
    pub fn add_violation(&mut self, violation: ConformanceViolation) {
        if matches!(
            violation.severity,
            ViolationSeverity::Error | ViolationSeverity::Fatal
        ) {
            self.is_valid = false;
        }
        self.violations.push(violation);
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: ConformanceWarning) {
        self.warnings.push(warning);
    }

    /// Get all error-level violations
    pub fn errors(&self) -> Vec<&ConformanceViolation> {
        self.violations
            .iter()
            .filter(|v| {
                matches!(
                    v.severity,
                    ViolationSeverity::Error | ViolationSeverity::Fatal
                )
            })
            .collect()
    }

    /// Get all warning-level violations
    pub fn warning_violations(&self) -> Vec<&ConformanceViolation> {
        self.violations
            .iter()
            .filter(|v| matches!(v.severity, ViolationSeverity::Warning))
            .collect()
    }

    /// Get total number of issues (violations + warnings)
    pub fn total_issues(&self) -> usize {
        self.violations.len() + self.warnings.len()
    }

    /// Check if there are any fatal errors
    pub fn has_fatal_errors(&self) -> bool {
        self.violations
            .iter()
            .any(|v| matches!(v.severity, ViolationSeverity::Fatal))
    }

    /// Merge another conformance result into this one
    pub fn merge(&mut self, other: ConformanceResult) {
        if !other.is_valid {
            self.is_valid = false;
        }
        self.violations.extend(other.violations);
        self.warnings.extend(other.warnings);
        self.metadata.elements_validated += other.metadata.elements_validated;
        self.metadata.constraints_evaluated += other.metadata.constraints_evaluated;
    }
}

impl ConformanceViolation {
    /// Create a new violation
    pub fn new(
        path: impl Into<String>,
        message: impl Into<String>,
        severity: ViolationSeverity,
    ) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            severity,
            constraint_key: None,
            expected: None,
            actual: None,
            location: None,
        }
    }

    /// Create an error violation
    pub fn error(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(path, message, ViolationSeverity::Error)
    }

    /// Create a warning violation
    pub fn warning(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(path, message, ViolationSeverity::Warning)
    }

    /// Create a fatal violation
    pub fn fatal(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(path, message, ViolationSeverity::Fatal)
    }

    /// Set the constraint key
    pub fn with_constraint_key(mut self, key: impl Into<String>) -> Self {
        self.constraint_key = Some(key.into());
        self
    }

    /// Set expected and actual values
    pub fn with_values(mut self, expected: impl Into<String>, actual: impl Into<String>) -> Self {
        self.expected = Some(expected.into());
        self.actual = Some(actual.into());
        self
    }

    /// Set source location
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl ConformanceWarning {
    /// Create a new warning
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
            code: None,
            location: None,
        }
    }

    /// Set warning code
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Set source location
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(line: u32, column: u32) -> Self {
        Self {
            line,
            column,
            char_offset: None,
            length: None,
        }
    }

    /// Set character offset
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.char_offset = Some(offset);
        self
    }

    /// Set length
    pub fn with_length(mut self, length: usize) -> Self {
        self.length = Some(length);
        self
    }
}

impl Default for ConformanceMetadata {
    fn default() -> Self {
        Self {
            validator_version: "1.0.0".to_string(),
            validation_time_ms: 0,
            elements_validated: 0,
            constraints_evaluated: 0,
            additional: HashMap::new(),
        }
    }
}

impl ConformanceMetadata {
    /// Create new metadata
    pub fn new() -> Self {
        Self::default()
    }

    /// Set validator version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.validator_version = version.into();
        self
    }

    /// Set validation time
    pub fn with_time(mut self, time_ms: u64) -> Self {
        self.validation_time_ms = time_ms;
        self
    }

    /// Add additional metadata
    pub fn with_additional(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional.insert(key.into(), value.into());
        self
    }
}

impl std::fmt::Display for ViolationSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViolationSeverity::Fatal => write!(f, "FATAL"),
            ViolationSeverity::Error => write!(f, "ERROR"),
            ViolationSeverity::Warning => write!(f, "WARNING"),
            ViolationSeverity::Information => write!(f, "INFO"),
        }
    }
}

impl std::fmt::Display for ConformanceViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.path, self.message)?;

        if let Some(constraint_key) = &self.constraint_key {
            write!(f, " (constraint: {})", constraint_key)?;
        }

        if let (Some(expected), Some(actual)) = (&self.expected, &self.actual) {
            write!(f, " (expected: {}, actual: {})", expected, actual)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ConformanceWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[WARNING] {}: {}", self.path, self.message)?;

        if let Some(code) = &self.code {
            write!(f, " ({})", code)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conformance_result() {
        let mut result = ConformanceResult::new("http://example.com/profile", "Patient");
        assert!(result.is_valid);
        assert_eq!(result.total_issues(), 0);

        let violation = ConformanceViolation::error("Patient.name", "Missing required name");
        result.add_violation(violation);

        assert!(!result.is_valid);
        assert_eq!(result.total_issues(), 1);
        assert_eq!(result.errors().len(), 1);

        let warning = ConformanceWarning::new("Patient.id", "ID should be present");
        result.add_warning(warning);

        assert_eq!(result.total_issues(), 2);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_violation_creation() {
        let violation = ConformanceViolation::error("Patient.name", "Missing name")
            .with_constraint_key("pat-1")
            .with_values("string", "null")
            .with_location(SourceLocation::new(10, 15));

        assert_eq!(violation.path, "Patient.name");
        assert_eq!(violation.severity, ViolationSeverity::Error);
        assert_eq!(violation.constraint_key.as_deref(), Some("pat-1"));
        assert!(violation.location.is_some());
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(ViolationSeverity::Fatal.to_string(), "FATAL");
        assert_eq!(ViolationSeverity::Error.to_string(), "ERROR");
        assert_eq!(ViolationSeverity::Warning.to_string(), "WARNING");
        assert_eq!(ViolationSeverity::Information.to_string(), "INFO");
    }

    #[test]
    fn test_result_merge() {
        let mut result1 = ConformanceResult::new("profile1", "Patient");
        result1.add_violation(ConformanceViolation::error("path1", "error1"));

        let mut result2 = ConformanceResult::new("profile2", "Patient");
        result2.add_violation(ConformanceViolation::warning("path2", "warning1"));

        result1.merge(result2);

        assert!(!result1.is_valid); // Has error
        assert_eq!(result1.violations.len(), 2);
        assert_eq!(result1.errors().len(), 1);
        assert_eq!(result1.warning_violations().len(), 1);
    }

    #[test]
    fn test_source_location() {
        let location = SourceLocation::new(10, 5).with_offset(100).with_length(10);

        assert_eq!(location.line, 10);
        assert_eq!(location.column, 5);
        assert_eq!(location.char_offset, Some(100));
        assert_eq!(location.length, Some(10));
    }
}

/// Enhanced conformance validation framework with extensibility
pub struct ConformanceValidator {
    /// Custom validation rules
    pub custom_rules: Vec<Box<dyn ValidationRule>>,
    /// Validation context
    pub context: ValidationContext,
    /// Performance metrics
    pub metrics: ValidationMetrics,
}

/// Validation context for conformance checking
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationContext {
    /// FHIR version being validated against
    pub fhir_version: String,
    /// Validation mode (strict, lenient, etc.)
    pub validation_mode: ValidationMode,
    /// Profile URLs to validate against
    pub target_profiles: Vec<String>,
    /// Additional context parameters
    pub parameters: HashMap<String, String>,
    /// Validation scope
    pub scope: ValidationScope,
}

/// Validation mode enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ValidationMode {
    /// Strict validation - all constraints must be satisfied
    Strict,
    /// Lenient validation - warnings for non-critical violations
    Lenient,
    /// Profile-only validation - only validate against specific profiles
    ProfileOnly,
    /// Custom validation with specified rule sets
    Custom(Vec<String>),
}

/// Validation scope enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ValidationScope {
    /// Validate entire resource tree
    Full,
    /// Validate only specified paths
    PathsOnly(Vec<String>),
    /// Validate only elements matching criteria
    Conditional(ValidationCondition),
}

/// Validation condition for conditional scoping
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationCondition {
    /// FHIRPath expression for condition
    pub expression: String,
    /// Whether to include or exclude matching elements
    pub include: bool,
}

/// Trait for custom validation rules
pub trait ValidationRule: Send + Sync {
    /// Rule identifier
    fn rule_id(&self) -> &str;

    /// Human-readable description
    fn description(&self) -> &str;

    /// Validate a resource element
    fn validate(
        &self,
        path: &str,
        value: &serde_json::Value,
        context: &ValidationContext,
    ) -> ValidationRuleResult;

    /// Check if this rule applies to the given path
    fn applies_to(&self, path: &str, resource_type: &str) -> bool;

    /// Rule priority (higher numbers are evaluated first)
    fn priority(&self) -> u32 {
        100
    }
}

/// Result of a validation rule execution
#[derive(Debug, Clone)]
pub struct ValidationRuleResult {
    /// Whether the rule passed
    pub passed: bool,
    /// Violations found by this rule
    pub violations: Vec<ConformanceViolation>,
    /// Warnings generated by this rule
    pub warnings: Vec<ConformanceWarning>,
    /// Rule execution metadata
    pub metadata: HashMap<String, String>,
}

/// Validation metrics for performance monitoring
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationMetrics {
    /// Total validation time in microseconds
    pub total_time_us: u64,
    /// Number of rules evaluated
    pub rules_evaluated: u32,
    /// Number of violations found
    pub violations_found: u32,
    /// Number of warnings generated
    pub warnings_generated: u32,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Cache hit/miss statistics
    pub cache_stats: CacheStatistics,
}

/// Cache statistics for validation performance
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CacheStatistics {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Cache size in entries
    pub size: usize,
}

/// Validation profile information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationProfile {
    /// Profile URL
    pub url: String,
    /// Profile version
    pub version: String,
    /// Profile name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Base profile URL (if this is a derived profile)
    pub base_profile: Option<String>,
    /// Validation rules specific to this profile
    pub rules: Vec<ProfileRule>,
    /// Dependencies on other profiles
    pub dependencies: Vec<String>,
}

/// Profile-specific validation rule
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProfileRule {
    /// Rule identifier within the profile
    pub id: String,
    /// FHIRPath expression for the rule
    pub expression: String,
    /// Human-readable description
    pub description: String,
    /// Rule severity
    pub severity: ViolationSeverity,
    /// Rule category
    pub category: RuleCategory,
}

/// Categories of validation rules
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RuleCategory {
    /// Structural rules (cardinality, data types)
    Structural,
    /// Terminology rules (code validation)
    Terminology,
    /// Business rules (invariants)
    Business,
    /// Reference integrity rules
    References,
    /// Custom rules
    Custom(String),
}

impl ConformanceValidator {
    /// Create a new conformance validator
    pub fn new(context: ValidationContext) -> Self {
        Self {
            custom_rules: Vec::new(),
            context,
            metrics: ValidationMetrics::default(),
        }
    }

    /// Add a custom validation rule
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.custom_rules.push(rule);
        // Sort by priority (highest first)
        self.custom_rules
            .sort_by(|a, b| b.priority().cmp(&a.priority()));
    }

    /// Validate a resource using all applicable rules
    pub fn validate(
        &mut self,
        resource: &serde_json::Value,
        resource_type: &str,
    ) -> ConformanceResult {
        let start_time = std::time::Instant::now();

        let mut result = ConformanceResult::new("", resource_type);

        // Apply all custom rules
        for rule in &self.custom_rules {
            if rule.applies_to("", resource_type) {
                let rule_result = rule.validate("", resource, &self.context);
                self.merge_rule_result(&mut result, &rule_result);
                self.metrics.rules_evaluated += 1;
            }
        }

        // Update metrics
        self.metrics.total_time_us = start_time.elapsed().as_micros() as u64;
        self.metrics.violations_found = result.violations.len() as u32;
        self.metrics.warnings_generated = result.warnings.len() as u32;

        result
    }

    /// Merge rule result into overall result
    fn merge_rule_result(
        &self,
        result: &mut ConformanceResult,
        rule_result: &ValidationRuleResult,
    ) {
        result
            .violations
            .extend(rule_result.violations.iter().cloned());
        result.warnings.extend(rule_result.warnings.iter().cloned());

        if !rule_result.violations.is_empty() {
            result.is_valid = false;
        }
    }

    /// Get validation metrics
    pub fn get_metrics(&self) -> &ValidationMetrics {
        &self.metrics
    }

    /// Reset metrics
    pub fn reset_metrics(&mut self) {
        self.metrics = ValidationMetrics::default();
    }
}

impl ValidationContext {
    /// Create a new validation context
    pub fn new(fhir_version: impl Into<String>) -> Self {
        Self {
            fhir_version: fhir_version.into(),
            validation_mode: ValidationMode::Strict,
            target_profiles: Vec::new(),
            parameters: HashMap::new(),
            scope: ValidationScope::Full,
        }
    }

    /// Set validation mode
    pub fn with_mode(mut self, mode: ValidationMode) -> Self {
        self.validation_mode = mode;
        self
    }

    /// Add target profile
    pub fn with_profile(mut self, profile_url: impl Into<String>) -> Self {
        self.target_profiles.push(profile_url.into());
        self
    }

    /// Add parameter
    pub fn with_parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }

    /// Set validation scope
    pub fn with_scope(mut self, scope: ValidationScope) -> Self {
        self.scope = scope;
        self
    }
}

impl ValidationRuleResult {
    /// Create a successful result
    pub fn success() -> Self {
        Self {
            passed: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create a failed result with violations
    pub fn with_violations(violations: Vec<ConformanceViolation>) -> Self {
        Self {
            passed: violations.is_empty(),
            violations,
            warnings: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

impl ValidationMetrics {
    /// Calculate validation rate (elements per second)
    pub fn validation_rate(&self) -> f64 {
        if self.total_time_us == 0 {
            0.0
        } else {
            self.rules_evaluated as f64 / (self.total_time_us as f64 / 1_000_000.0)
        }
    }

    /// Calculate cache hit rate
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_stats.hits + self.cache_stats.misses;
        if total == 0 {
            0.0
        } else {
            self.cache_stats.hits as f64 / total as f64
        }
    }
}

impl CacheStatistics {
    /// Record a cache hit
    pub fn record_hit(&mut self) {
        self.hits += 1;
    }

    /// Record a cache miss
    pub fn record_miss(&mut self) {
        self.misses += 1;
    }

    /// Update cache size
    pub fn update_size(&mut self, size: usize) {
        self.size = size;
    }
}
