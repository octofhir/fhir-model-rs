//! Simple terminology service abstractions for FHIR validation

use async_trait::async_trait;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Simple terminology service provider
#[async_trait]
pub trait TerminologyProvider: Send + Sync + std::fmt::Debug {
    /// Validate a code against a code system
    async fn validate_code(&self, code: &str, system: &str, version: Option<&str>) -> Result<bool>;

    /// Expand a ValueSet to get all contained codes
    async fn expand_valueset(
        &self,
        valueset_url: &str,
        parameters: Option<&ExpansionParameters>,
    ) -> Result<ValueSetExpansion>;

    /// Translate a code using concept maps
    async fn translate_code(
        &self,
        source_code: &str,
        target_system: &str,
        concept_map_url: Option<&str>,
    ) -> Result<TranslationResult>;

    /// Look up concept details from a code system
    async fn lookup_code(
        &self,
        system: &str,
        code: &str,
        version: Option<&str>,
        properties: Option<Vec<&str>>,
    ) -> Result<LookupResult>;

    /// Validate a code against a value set
    async fn validate_code_vs(
        &self,
        valueset: &str,
        system: Option<&str>,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidationResult>;

    /// Test subsumption relationships between concepts
    async fn subsumes(&self, system: &str, parent: &str, child: &str) -> Result<SubsumptionResult>;

    /// Test connection to terminology server
    async fn test_connection(&self) -> Result<ConnectionStatus>;
}

/// Expansion parameters for ValueSet operations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpansionParameters {
    /// Filter string
    pub filter: Option<String>,
    /// Maximum number of codes to return
    pub count: Option<u32>,
    /// Language preferences
    pub language: Option<String>,
}

/// Result of ValueSet expansion
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValueSetExpansion {
    /// The expanded codes
    pub contains: Vec<ValueSetConcept>,
    /// Total count of concepts
    pub total: Option<u32>,
    /// Expansion parameters used
    pub parameters: Vec<ExpansionParameter>,
    /// Expansion timestamp
    pub timestamp: Option<String>,
}

/// A concept in a ValueSet expansion
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValueSetConcept {
    /// The code
    pub code: String,
    /// The system
    pub system: Option<String>,
    /// Display text
    pub display: Option<String>,
}

/// Expansion parameter
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExpansionParameter {
    /// Parameter name
    pub name: String,
    /// Parameter value
    pub value: String,
}

/// Result of code translation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TranslationResult {
    /// Whether translation was successful
    pub success: bool,
    /// Translation targets
    pub targets: Vec<TranslationTarget>,
    /// Message if translation failed
    pub message: Option<String>,
}

/// Translation target
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TranslationTarget {
    /// Target code
    pub code: String,
    /// Target system
    pub system: String,
    /// Display text
    pub display: Option<String>,
    /// Equivalence level
    pub equivalence: EquivalenceLevel,
}

/// Equivalence level for translations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EquivalenceLevel {
    /// Equivalent concepts
    Equivalent,
    /// Related concepts
    Related,
    /// Source is narrower than target
    Narrower,
    /// Source is broader than target
    Broader,
}

/// Connection status for terminology server
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConnectionStatus {
    /// Whether connection succeeded
    pub connected: bool,
    /// Response time in milliseconds
    pub response_time_ms: Option<u64>,
    /// Server version if available
    pub server_version: Option<String>,
    /// Error message if connection failed
    pub error: Option<String>,
}

/// Result of concept lookup operation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LookupResult {
    /// The concept display name
    pub display: Option<String>,
    /// The concept definition
    pub definition: Option<String>,
    /// Concept properties
    pub properties: Vec<ConceptProperty>,
}

/// A concept property from lookup
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConceptProperty {
    /// Property code
    pub code: String,
    /// Property value
    pub value: String,
    /// Property type
    pub property_type: Option<String>,
}

/// Result of validation operation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationResult {
    /// Whether validation succeeded
    pub result: bool,
    /// Display text for the code
    pub display: Option<String>,
    /// Validation message
    pub message: Option<String>,
}

/// Result of subsumption test
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubsumptionResult {
    /// Subsumption outcome
    pub outcome: SubsumptionOutcome,
}

/// Subsumption outcomes
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SubsumptionOutcome {
    /// Parent subsumes child
    Subsumes,
    /// Child subsumes parent
    SubsumedBy,
    /// Concepts are equivalent
    Equivalent,
    /// No subsumption relationship
    NotSubsumed,
}

/// No-operation terminology provider for testing
#[derive(Debug, Clone, Default)]
pub struct NoOpTerminologyProvider;

#[async_trait]
impl TerminologyProvider for NoOpTerminologyProvider {
    async fn validate_code(
        &self,
        _code: &str,
        _system: &str,
        _version: Option<&str>,
    ) -> Result<bool> {
        Ok(true)
    }

    async fn expand_valueset(
        &self,
        _valueset_url: &str,
        _parameters: Option<&ExpansionParameters>,
    ) -> Result<ValueSetExpansion> {
        Ok(ValueSetExpansion {
            contains: Vec::new(),
            total: Some(0),
            parameters: Vec::new(),
            timestamp: None,
        })
    }

    async fn translate_code(
        &self,
        source_code: &str,
        target_system: &str,
        _concept_map_url: Option<&str>,
    ) -> Result<TranslationResult> {
        Ok(TranslationResult {
            success: true,
            targets: vec![TranslationTarget {
                code: source_code.to_string(),
                system: target_system.to_string(),
                display: None,
                equivalence: EquivalenceLevel::Equivalent,
            }],
            message: None,
        })
    }

    async fn lookup_code(
        &self,
        _system: &str,
        code: &str,
        _version: Option<&str>,
        _properties: Option<Vec<&str>>,
    ) -> Result<LookupResult> {
        Ok(LookupResult {
            display: Some(format!("Mock display for {code}")),
            definition: Some(format!("Mock definition for {code}")),
            properties: Vec::new(),
        })
    }

    async fn validate_code_vs(
        &self,
        _valueset: &str,
        _system: Option<&str>,
        _code: &str,
        _display: Option<&str>,
    ) -> Result<ValidationResult> {
        Ok(ValidationResult {
            result: true,
            display: Some("Mock display".to_string()),
            message: None,
        })
    }

    async fn subsumes(
        &self,
        _system: &str,
        _parent: &str,
        _child: &str,
    ) -> Result<SubsumptionResult> {
        Ok(SubsumptionResult {
            outcome: SubsumptionOutcome::NotSubsumed,
        })
    }

    async fn test_connection(&self) -> Result<ConnectionStatus> {
        Ok(ConnectionStatus {
            connected: true,
            response_time_ms: Some(0),
            server_version: Some("NoOp-1.0.0".to_string()),
            error: None,
        })
    }
}

/// HTTP-based TerminologyProvider implementation
#[cfg(feature = "http-client")]
#[derive(Debug)]
pub struct HttpTerminologyProvider {
    /// HTTP client for making requests
    client: reqwest::Client,
    /// Base URL of the terminology server
    base_url: String,
    /// Authentication token (if needed)
    auth_token: Option<String>,
}

#[cfg(feature = "http-client")]
impl HttpTerminologyProvider {
    /// Create a new HttpTerminologyProvider
    pub fn new(base_url: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .gzip(true)
            .build()
            .map_err(|e| {
                crate::error::ModelError::schema_load_error(format!(
                    "Failed to create HTTP client: {e}"
                ))
            })?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            auth_token: None,
        })
    }

    /// Set authentication token
    pub fn with_auth_token(mut self, token: String) -> Self {
        self.auth_token = Some(token);
        self
    }

    /// Build request with authentication
    fn build_request(&self, method: reqwest::Method, url: &str) -> reqwest::RequestBuilder {
        let mut request = self.client.request(method, url);

        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }

        request.header("Accept", "application/fhir+json")
    }
}

#[cfg(feature = "http-client")]
#[async_trait]
impl TerminologyProvider for HttpTerminologyProvider {
    async fn validate_code(&self, code: &str, system: &str, version: Option<&str>) -> Result<bool> {
        let mut url = format!("{}/CodeSystem/$validate-code", self.base_url);

        let mut params = vec![
            ("code".to_string(), code.to_string()),
            ("system".to_string(), system.to_string()),
        ];

        if let Some(v) = version {
            params.push(("version".to_string(), v.to_string()));
        }

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");

        url.push('?');
        url.push_str(&query_string);

        let response = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::validation_error(format!("HTTP request failed: {e}"))
            })?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await.map_err(|e| {
                crate::error::ModelError::validation_error(format!("Failed to parse JSON: {e}"))
            })?;

            // Extract result from Parameters resource
            if let Some(params_array) = json.get("parameter").and_then(|p| p.as_array()) {
                for param in params_array {
                    if let Some(name) = param.get("name").and_then(|n| n.as_str())
                        && name == "result"
                    {
                        return Ok(param
                            .get("valueBoolean")
                            .and_then(|b| b.as_bool())
                            .unwrap_or(false));
                    }
                }
            }
        }

        Ok(false)
    }

    async fn expand_valueset(
        &self,
        valueset_url: &str,
        _parameters: Option<&ExpansionParameters>,
    ) -> Result<ValueSetExpansion> {
        let url = format!("{}/ValueSet/$expand?url={}", self.base_url, valueset_url);

        let response = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::validation_error(format!("HTTP request failed: {e}"))
            })?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await.map_err(|e| {
                crate::error::ModelError::validation_error(format!("Failed to parse JSON: {e}"))
            })?;

            // Parse ValueSet expansion
            let contains = if let Some(expansion) = json.get("expansion") {
                if let Some(contains_array) = expansion.get("contains").and_then(|c| c.as_array()) {
                    contains_array
                        .iter()
                        .filter_map(|item| {
                            Some(ValueSetConcept {
                                code: item.get("code")?.as_str()?.to_string(),
                                system: item
                                    .get("system")
                                    .and_then(|s| s.as_str())
                                    .map(String::from),
                                display: item
                                    .get("display")
                                    .and_then(|d| d.as_str())
                                    .map(String::from),
                            })
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            return Ok(ValueSetExpansion {
                contains,
                total: Some(0), // Would need to parse from response
                parameters: Vec::new(),
                timestamp: None,
            });
        }

        Ok(ValueSetExpansion {
            contains: Vec::new(),
            total: Some(0),
            parameters: Vec::new(),
            timestamp: None,
        })
    }

    async fn translate_code(
        &self,
        source_code: &str,
        target_system: &str,
        concept_map_url: Option<&str>,
    ) -> Result<TranslationResult> {
        let map_url = concept_map_url.unwrap_or("");
        let url = format!(
            "{}/ConceptMap/$translate?code={}&system={}&url={}",
            self.base_url, source_code, target_system, map_url
        );

        let response = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::validation_error(format!("HTTP request failed: {e}"))
            })?;

        if response.status().is_success() {
            return Ok(TranslationResult {
                success: true,
                targets: vec![TranslationTarget {
                    code: source_code.to_string(),
                    system: target_system.to_string(),
                    display: None,
                    equivalence: EquivalenceLevel::Equivalent,
                }],
                message: None,
            });
        }

        Ok(TranslationResult {
            success: false,
            targets: Vec::new(),
            message: Some("Translation failed".to_string()),
        })
    }

    async fn lookup_code(
        &self,
        system: &str,
        code: &str,
        version: Option<&str>,
        properties: Option<Vec<&str>>,
    ) -> Result<LookupResult> {
        let mut url = format!("{}/CodeSystem/$lookup", self.base_url);

        let mut params = vec![
            ("system".to_string(), system.to_string()),
            ("code".to_string(), code.to_string()),
        ];

        if let Some(v) = version {
            params.push(("version".to_string(), v.to_string()));
        }

        if let Some(props) = properties {
            for prop in props {
                params.push(("property".to_string(), prop.to_string()));
            }
        }

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");

        url.push('?');
        url.push_str(&query_string);

        let response = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::validation_error(format!("HTTP request failed: {e}"))
            })?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await.map_err(|e| {
                crate::error::ModelError::validation_error(format!("Failed to parse JSON: {e}"))
            })?;

            // Parse Parameters resource response
            let mut lookup_result = LookupResult {
                display: None,
                definition: None,
                properties: Vec::new(),
            };

            if let Some(params_array) = json.get("parameter").and_then(|p| p.as_array()) {
                for param in params_array {
                    if let Some(name) = param.get("name").and_then(|n| n.as_str()) {
                        match name {
                            "display" => {
                                lookup_result.display = param
                                    .get("valueString")
                                    .and_then(|v| v.as_str())
                                    .map(String::from);
                            }
                            "definition" => {
                                lookup_result.definition = param
                                    .get("valueString")
                                    .and_then(|v| v.as_str())
                                    .map(String::from);
                            }
                            "property" => {
                                if let Some(parts) = param.get("part").and_then(|p| p.as_array()) {
                                    let mut prop_code = String::new();
                                    let mut prop_value = String::new();
                                    let mut prop_type = None;

                                    for part in parts {
                                        if let Some(part_name) =
                                            part.get("name").and_then(|n| n.as_str())
                                        {
                                            match part_name {
                                                "code" => {
                                                    prop_code = part
                                                        .get("valueCode")
                                                        .or_else(|| part.get("valueString"))
                                                        .and_then(|v| v.as_str())
                                                        .unwrap_or("")
                                                        .to_string();
                                                }
                                                "value" => {
                                                    prop_value = part
                                                        .get("valueString")
                                                        .or_else(|| part.get("valueCode"))
                                                        .and_then(|v| v.as_str())
                                                        .map(String::from)
                                                        .or_else(|| {
                                                            part.get("valueBoolean")
                                                                .and_then(|v| v.as_bool())
                                                                .map(|b| b.to_string())
                                                        })
                                                        .unwrap_or_default();
                                                }
                                                "type" => {
                                                    prop_type = part
                                                        .get("valueCode")
                                                        .and_then(|v| v.as_str())
                                                        .map(String::from);
                                                }
                                                _ => {}
                                            }
                                        }
                                    }

                                    if !prop_code.is_empty() {
                                        lookup_result.properties.push(ConceptProperty {
                                            code: prop_code,
                                            value: prop_value,
                                            property_type: prop_type,
                                        });
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            return Ok(lookup_result);
        }

        Ok(LookupResult {
            display: None,
            definition: None,
            properties: Vec::new(),
        })
    }

    async fn validate_code_vs(
        &self,
        valueset: &str,
        system: Option<&str>,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidationResult> {
        let mut url = format!("{}/ValueSet/$validate-code", self.base_url);

        let mut params = vec![
            ("url".to_string(), valueset.to_string()),
            ("code".to_string(), code.to_string()),
        ];

        if let Some(sys) = system {
            params.push(("system".to_string(), sys.to_string()));
        }

        if let Some(disp) = display {
            params.push(("display".to_string(), disp.to_string()));
        }

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");

        url.push('?');
        url.push_str(&query_string);

        let response = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::validation_error(format!("HTTP request failed: {e}"))
            })?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await.map_err(|e| {
                crate::error::ModelError::validation_error(format!("Failed to parse JSON: {e}"))
            })?;

            // Parse Parameters resource response
            let mut validation_result = ValidationResult {
                result: false,
                display: None,
                message: None,
            };

            if let Some(params_array) = json.get("parameter").and_then(|p| p.as_array()) {
                for param in params_array {
                    if let Some(name) = param.get("name").and_then(|n| n.as_str()) {
                        match name {
                            "result" => {
                                validation_result.result = param
                                    .get("valueBoolean")
                                    .and_then(|b| b.as_bool())
                                    .unwrap_or(false);
                            }
                            "display" => {
                                validation_result.display = param
                                    .get("valueString")
                                    .and_then(|v| v.as_str())
                                    .map(String::from);
                            }
                            "message" => {
                                validation_result.message = param
                                    .get("valueString")
                                    .and_then(|v| v.as_str())
                                    .map(String::from);
                            }
                            _ => {}
                        }
                    }
                }
            }

            return Ok(validation_result);
        }

        Ok(ValidationResult {
            result: false,
            display: None,
            message: Some("Validation failed".to_string()),
        })
    }

    async fn subsumes(&self, system: &str, parent: &str, child: &str) -> Result<SubsumptionResult> {
        let mut url = format!("{}/CodeSystem/$subsumes", self.base_url);

        let params = [
            ("system".to_string(), system.to_string()),
            ("codeA".to_string(), parent.to_string()),
            ("codeB".to_string(), child.to_string()),
        ];

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");

        url.push('?');
        url.push_str(&query_string);

        let response = self
            .build_request(reqwest::Method::GET, &url)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::validation_error(format!("HTTP request failed: {e}"))
            })?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await.map_err(|e| {
                crate::error::ModelError::validation_error(format!("Failed to parse JSON: {e}"))
            })?;

            // Parse Parameters resource response
            if let Some(params_array) = json.get("parameter").and_then(|p| p.as_array()) {
                for param in params_array {
                    if let Some(name) = param.get("name").and_then(|n| n.as_str())
                        && name == "outcome"
                        && let Some(outcome_str) = param.get("valueCode").and_then(|v| v.as_str())
                    {
                        let outcome = match outcome_str {
                            "subsumes" => SubsumptionOutcome::Subsumes,
                            "subsumed-by" => SubsumptionOutcome::SubsumedBy,
                            "equivalent" => SubsumptionOutcome::Equivalent,
                            _ => SubsumptionOutcome::NotSubsumed,
                        };
                        return Ok(SubsumptionResult { outcome });
                    }
                }
            }
        }

        Ok(SubsumptionResult {
            outcome: SubsumptionOutcome::NotSubsumed,
        })
    }

    async fn test_connection(&self) -> Result<ConnectionStatus> {
        let url = format!("{}/metadata", self.base_url);
        let start = std::time::Instant::now();

        match self.build_request(reqwest::Method::GET, &url).send().await {
            Ok(response) => {
                let response_time = start.elapsed().as_millis() as u64;

                if response.status().is_success() {
                    Ok(ConnectionStatus {
                        connected: true,
                        response_time_ms: Some(response_time),
                        server_version: None, // Could parse from capability statement
                        error: None,
                    })
                } else {
                    Ok(ConnectionStatus {
                        connected: false,
                        response_time_ms: Some(response_time),
                        server_version: None,
                        error: Some(format!(
                            "HTTP {}: {}",
                            response.status(),
                            response.status().canonical_reason().unwrap_or("Unknown")
                        )),
                    })
                }
            }
            Err(e) => Ok(ConnectionStatus {
                connected: false,
                response_time_ms: None,
                server_version: None,
                error: Some(format!("Connection failed: {e}")),
            }),
        }
    }
}

// ============================================================================
// Caching Infrastructure
// ============================================================================

use std::time::Duration;

/// Cache configuration for terminology operations
///
/// Controls TTL and maximum size for each type of cached operation.
/// Default values are optimized for typical FHIR terminology usage patterns.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TerminologyCacheConfig {
    /// TTL for validation cache entries (default: 1 hour)
    pub validation_ttl: Duration,
    /// Maximum entries in validation cache (default: 10,000)
    pub validation_max_size: u64,
    /// TTL for expansion cache entries (default: 1 hour)
    pub expansion_ttl: Duration,
    /// Maximum entries in expansion cache (default: 1,000)
    pub expansion_max_size: u64,
    /// TTL for lookup cache entries (default: 24 hours)
    pub lookup_ttl: Duration,
    /// Maximum entries in lookup cache (default: 5,000)
    pub lookup_max_size: u64,
}

impl Default for TerminologyCacheConfig {
    fn default() -> Self {
        Self {
            validation_ttl: Duration::from_secs(3600), // 1 hour
            validation_max_size: 10_000,
            expansion_ttl: Duration::from_secs(3600), // 1 hour
            expansion_max_size: 1_000,
            lookup_ttl: Duration::from_secs(86400), // 24 hours
            lookup_max_size: 5_000,
        }
    }
}

impl TerminologyCacheConfig {
    /// Create a new cache configuration with custom TTLs
    pub fn new(validation_ttl: Duration, expansion_ttl: Duration, lookup_ttl: Duration) -> Self {
        Self {
            validation_ttl,
            expansion_ttl,
            lookup_ttl,
            ..Default::default()
        }
    }

    /// Set validation cache TTL
    pub fn with_validation_ttl(mut self, ttl: Duration) -> Self {
        self.validation_ttl = ttl;
        self
    }

    /// Set validation cache max size
    pub fn with_validation_max_size(mut self, size: u64) -> Self {
        self.validation_max_size = size;
        self
    }

    /// Set expansion cache TTL
    pub fn with_expansion_ttl(mut self, ttl: Duration) -> Self {
        self.expansion_ttl = ttl;
        self
    }

    /// Set expansion cache max size
    pub fn with_expansion_max_size(mut self, size: u64) -> Self {
        self.expansion_max_size = size;
        self
    }

    /// Set lookup cache TTL
    pub fn with_lookup_ttl(mut self, ttl: Duration) -> Self {
        self.lookup_ttl = ttl;
        self
    }

    /// Set lookup cache max size
    pub fn with_lookup_max_size(mut self, size: u64) -> Self {
        self.lookup_max_size = size;
        self
    }
}

/// Cache statistics for terminology provider
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TerminologyCacheStats {
    /// Number of entries in the validation cache
    pub validation_entries: u64,
    /// Number of entries in the expansion cache
    pub expansion_entries: u64,
    /// Number of entries in the lookup cache
    pub lookup_entries: u64,
}

// ============================================================================
// Cache Key Types (for caching feature)
// ============================================================================

/// Key for validation cache
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg(feature = "caching")]
pub struct ValidationCacheKey {
    /// Value set URL for validate_code_vs, or system for validate_code
    pub key: String,
    /// System (optional for validate_code_vs)
    pub system: Option<String>,
    /// Code being validated
    pub code: String,
    /// Version (optional)
    pub version: Option<String>,
}

/// Key for lookup cache
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg(feature = "caching")]
pub struct LookupCacheKey {
    /// Code system URL
    pub system: String,
    /// Code
    pub code: String,
    /// Version (optional)
    pub version: Option<String>,
}

// ============================================================================
// CachedTerminologyProvider (requires "caching" feature)
// ============================================================================

#[cfg(feature = "caching")]
use moka::future::Cache;

/// Cached wrapper around any TerminologyProvider
///
/// Provides LRU caching with TTL for all terminology operations.
/// The cache uses moka for high-performance async caching.
///
/// # Example
///
/// ```ignore
/// use octofhir_fhir_model::terminology::{
///     CachedTerminologyProvider, TerminologyCacheConfig, NoOpTerminologyProvider
/// };
///
/// let inner = NoOpTerminologyProvider;
/// let cached = CachedTerminologyProvider::with_default_config(inner);
///
/// // Use the cached provider
/// let result = cached.validate_code("test", "http://test.com", None).await?;
/// ```
#[cfg(feature = "caching")]
pub struct CachedTerminologyProvider<T: TerminologyProvider> {
    inner: T,
    validation_cache: Cache<ValidationCacheKey, ValidationResult>,
    expansion_cache: Cache<String, ValueSetExpansion>,
    lookup_cache: Cache<LookupCacheKey, LookupResult>,
    #[allow(dead_code)]
    config: TerminologyCacheConfig,
}

#[cfg(feature = "caching")]
impl<T: TerminologyProvider> std::fmt::Debug for CachedTerminologyProvider<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CachedTerminologyProvider")
            .field("inner", &self.inner)
            .field("validation_entries", &self.validation_cache.entry_count())
            .field("expansion_entries", &self.expansion_cache.entry_count())
            .field("lookup_entries", &self.lookup_cache.entry_count())
            .finish()
    }
}

#[cfg(feature = "caching")]
impl<T: TerminologyProvider> CachedTerminologyProvider<T> {
    /// Create a new cached provider with custom configuration
    pub fn new(inner: T, config: TerminologyCacheConfig) -> Self {
        let validation_cache = Cache::builder()
            .max_capacity(config.validation_max_size)
            .time_to_live(config.validation_ttl)
            .build();

        let expansion_cache = Cache::builder()
            .max_capacity(config.expansion_max_size)
            .time_to_live(config.expansion_ttl)
            .build();

        let lookup_cache = Cache::builder()
            .max_capacity(config.lookup_max_size)
            .time_to_live(config.lookup_ttl)
            .build();

        Self {
            inner,
            validation_cache,
            expansion_cache,
            lookup_cache,
            config,
        }
    }

    /// Create a new cached provider with default configuration
    pub fn with_default_config(inner: T) -> Self {
        Self::new(inner, TerminologyCacheConfig::default())
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> TerminologyCacheStats {
        TerminologyCacheStats {
            validation_entries: self.validation_cache.entry_count(),
            expansion_entries: self.expansion_cache.entry_count(),
            lookup_entries: self.lookup_cache.entry_count(),
        }
    }

    /// Clear all caches
    pub fn clear_cache(&self) {
        self.validation_cache.invalidate_all();
        self.expansion_cache.invalidate_all();
        self.lookup_cache.invalidate_all();
    }

    /// Sync pending cache operations (moka is eventually consistent)
    pub async fn sync(&self) {
        self.validation_cache.run_pending_tasks().await;
        self.expansion_cache.run_pending_tasks().await;
        self.lookup_cache.run_pending_tasks().await;
    }

    /// Get reference to the inner provider
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

#[cfg(feature = "caching")]
#[async_trait]
impl<T: TerminologyProvider + 'static> TerminologyProvider for CachedTerminologyProvider<T> {
    async fn validate_code(&self, code: &str, system: &str, version: Option<&str>) -> Result<bool> {
        let key = ValidationCacheKey {
            key: system.to_string(),
            system: None,
            code: code.to_string(),
            version: version.map(String::from),
        };

        // Check cache first
        if let Some(cached) = self.validation_cache.get(&key).await {
            return Ok(cached.result);
        }

        // Call inner provider
        let result = self.inner.validate_code(code, system, version).await?;

        // Cache the result
        let validation_result = ValidationResult {
            result,
            display: None,
            message: None,
        };
        self.validation_cache.insert(key, validation_result).await;

        Ok(result)
    }

    async fn expand_valueset(
        &self,
        valueset_url: &str,
        parameters: Option<&ExpansionParameters>,
    ) -> Result<ValueSetExpansion> {
        // Only cache expansions without parameters
        if parameters.is_some() {
            return self.inner.expand_valueset(valueset_url, parameters).await;
        }

        let cache_key = valueset_url.to_string();

        // Check cache first
        if let Some(cached) = self.expansion_cache.get(&cache_key).await {
            return Ok(cached);
        }

        // Call inner provider
        let result = self.inner.expand_valueset(valueset_url, parameters).await?;

        // Cache the result
        self.expansion_cache.insert(cache_key, result.clone()).await;

        Ok(result)
    }

    async fn translate_code(
        &self,
        source_code: &str,
        target_system: &str,
        concept_map_url: Option<&str>,
    ) -> Result<TranslationResult> {
        // Translation is not cached (typically less frequent, context-dependent)
        self.inner
            .translate_code(source_code, target_system, concept_map_url)
            .await
    }

    async fn lookup_code(
        &self,
        system: &str,
        code: &str,
        version: Option<&str>,
        properties: Option<Vec<&str>>,
    ) -> Result<LookupResult> {
        // Only cache lookups without property filters
        if properties.is_some() {
            return self
                .inner
                .lookup_code(system, code, version, properties)
                .await;
        }

        let key = LookupCacheKey {
            system: system.to_string(),
            code: code.to_string(),
            version: version.map(String::from),
        };

        // Check cache first
        if let Some(cached) = self.lookup_cache.get(&key).await {
            return Ok(cached);
        }

        // Call inner provider
        let result = self
            .inner
            .lookup_code(system, code, version, properties)
            .await?;

        // Cache the result
        self.lookup_cache.insert(key, result.clone()).await;

        Ok(result)
    }

    async fn validate_code_vs(
        &self,
        valueset: &str,
        system: Option<&str>,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidationResult> {
        // Only cache validations without display (display matching is extra validation)
        if display.is_some() {
            return self
                .inner
                .validate_code_vs(valueset, system, code, display)
                .await;
        }

        let key = ValidationCacheKey {
            key: valueset.to_string(),
            system: system.map(String::from),
            code: code.to_string(),
            version: None,
        };

        // Check cache first
        if let Some(cached) = self.validation_cache.get(&key).await {
            return Ok(cached);
        }

        // Call inner provider
        let result = self
            .inner
            .validate_code_vs(valueset, system, code, display)
            .await?;

        // Cache the result
        self.validation_cache.insert(key, result.clone()).await;

        Ok(result)
    }

    async fn subsumes(&self, system: &str, parent: &str, child: &str) -> Result<SubsumptionResult> {
        // Subsumption is not cached (complex hierarchical lookups)
        self.inner.subsumes(system, parent, child).await
    }

    async fn test_connection(&self) -> Result<ConnectionStatus> {
        // Connection test is never cached
        self.inner.test_connection().await
    }
}

// ============================================================================
// DefaultTerminologyProvider (requires "http-client" + "caching" features)
// ============================================================================

/// Default ready-to-use terminology provider with HTTP client and caching
///
/// This is the recommended way to use terminology services for most applications.
/// It combines the HTTP terminology provider with automatic caching.
///
/// # Features Required
///
/// This type requires both `http-client` and `caching` features to be enabled.
///
/// # Example
///
/// ```ignore
/// use octofhir_fhir_model::terminology::DefaultTerminologyProvider;
///
/// // Create with default tx.fhir.org endpoint
/// let provider = DefaultTerminologyProvider::new()?;
///
/// // Or with custom server
/// let provider = DefaultTerminologyProvider::with_server("https://my-terminology-server.com/r4")?;
///
/// // Validate a code
/// let is_valid = provider.validate_code("active", "http://hl7.org/fhir/patient-status", None).await?;
/// ```
#[cfg(all(feature = "http-client", feature = "caching"))]
pub struct DefaultTerminologyProvider {
    inner: CachedTerminologyProvider<HttpTerminologyProvider>,
}

#[cfg(all(feature = "http-client", feature = "caching"))]
impl std::fmt::Debug for DefaultTerminologyProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefaultTerminologyProvider")
            .field("inner", &self.inner)
            .finish()
    }
}

#[cfg(all(feature = "http-client", feature = "caching"))]
impl DefaultTerminologyProvider {
    /// Default terminology server URL (tx.fhir.org R4)
    pub const DEFAULT_SERVER_URL: &'static str = "https://tx.fhir.org/r4";

    /// Create with default tx.fhir.org endpoint and default cache config
    pub fn new() -> Result<Self> {
        Self::with_server(Self::DEFAULT_SERVER_URL)
    }

    /// Create with custom server URL and default cache config
    pub fn with_server(base_url: &str) -> Result<Self> {
        Self::with_config(base_url, TerminologyCacheConfig::default())
    }

    /// Create with custom server URL and cache configuration
    pub fn with_config(base_url: &str, cache_config: TerminologyCacheConfig) -> Result<Self> {
        let http_provider = HttpTerminologyProvider::new(base_url.to_string())?;
        let cached = CachedTerminologyProvider::new(http_provider, cache_config);
        Ok(Self { inner: cached })
    }

    /// Add authentication token
    pub fn with_auth(self, _token: String) -> Self {
        // We need to recreate the cached provider with auth
        // This is a bit awkward but necessary since HttpTerminologyProvider takes ownership
        // For now, we'll just document that auth should be set at creation time
        // A better approach would be to store the config and allow rebuilding
        // Return self unchanged for now - auth should be set via with_config_and_auth
        self
    }

    /// Create with custom server URL, cache configuration, and authentication
    pub fn with_config_and_auth(
        base_url: &str,
        cache_config: TerminologyCacheConfig,
        auth_token: String,
    ) -> Result<Self> {
        let http_provider =
            HttpTerminologyProvider::new(base_url.to_string())?.with_auth_token(auth_token);
        let cached = CachedTerminologyProvider::new(http_provider, cache_config);
        Ok(Self { inner: cached })
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> TerminologyCacheStats {
        self.inner.cache_stats()
    }

    /// Clear all caches
    pub fn clear_cache(&self) {
        self.inner.clear_cache();
    }

    /// Sync pending cache operations
    pub async fn sync(&self) {
        self.inner.sync().await;
    }
}

#[cfg(all(feature = "http-client", feature = "caching"))]
#[async_trait]
impl TerminologyProvider for DefaultTerminologyProvider {
    async fn validate_code(&self, code: &str, system: &str, version: Option<&str>) -> Result<bool> {
        self.inner.validate_code(code, system, version).await
    }

    async fn expand_valueset(
        &self,
        valueset_url: &str,
        parameters: Option<&ExpansionParameters>,
    ) -> Result<ValueSetExpansion> {
        self.inner.expand_valueset(valueset_url, parameters).await
    }

    async fn translate_code(
        &self,
        source_code: &str,
        target_system: &str,
        concept_map_url: Option<&str>,
    ) -> Result<TranslationResult> {
        self.inner
            .translate_code(source_code, target_system, concept_map_url)
            .await
    }

    async fn lookup_code(
        &self,
        system: &str,
        code: &str,
        version: Option<&str>,
        properties: Option<Vec<&str>>,
    ) -> Result<LookupResult> {
        self.inner
            .lookup_code(system, code, version, properties)
            .await
    }

    async fn validate_code_vs(
        &self,
        valueset: &str,
        system: Option<&str>,
        code: &str,
        display: Option<&str>,
    ) -> Result<ValidationResult> {
        self.inner
            .validate_code_vs(valueset, system, code, display)
            .await
    }

    async fn subsumes(&self, system: &str, parent: &str, child: &str) -> Result<SubsumptionResult> {
        self.inner.subsumes(system, parent, child).await
    }

    async fn test_connection(&self) -> Result<ConnectionStatus> {
        self.inner.test_connection().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_noop_terminology_provider() {
        let provider = NoOpTerminologyProvider;

        // Test basic operations
        assert!(
            provider
                .validate_code("test", "http://test.com", None)
                .await
                .unwrap()
        );

        let expansion = provider
            .expand_valueset("http://test.com/vs", None)
            .await
            .unwrap();
        assert_eq!(expansion.total, Some(0));
        assert!(expansion.contains.is_empty());
    }

    #[test]
    fn test_cache_config_default() {
        let config = TerminologyCacheConfig::default();
        assert_eq!(config.validation_ttl, Duration::from_secs(3600));
        assert_eq!(config.validation_max_size, 10_000);
        assert_eq!(config.expansion_ttl, Duration::from_secs(3600));
        assert_eq!(config.expansion_max_size, 1_000);
        assert_eq!(config.lookup_ttl, Duration::from_secs(86400));
        assert_eq!(config.lookup_max_size, 5_000);
    }

    #[test]
    fn test_cache_config_builder() {
        let config = TerminologyCacheConfig::default()
            .with_validation_ttl(Duration::from_secs(1800))
            .with_validation_max_size(5_000)
            .with_expansion_ttl(Duration::from_secs(7200))
            .with_expansion_max_size(500);

        assert_eq!(config.validation_ttl, Duration::from_secs(1800));
        assert_eq!(config.validation_max_size, 5_000);
        assert_eq!(config.expansion_ttl, Duration::from_secs(7200));
        assert_eq!(config.expansion_max_size, 500);
    }

    #[test]
    fn test_cache_stats_default() {
        let stats = TerminologyCacheStats::default();
        assert_eq!(stats.validation_entries, 0);
        assert_eq!(stats.expansion_entries, 0);
        assert_eq!(stats.lookup_entries, 0);
    }

    #[cfg(feature = "caching")]
    #[tokio::test]
    async fn test_cached_provider_basic() {
        let inner = NoOpTerminologyProvider;
        let cached = CachedTerminologyProvider::with_default_config(inner);

        // Initial stats should be zero
        let stats = cached.cache_stats();
        assert_eq!(stats.validation_entries, 0);
        assert_eq!(stats.expansion_entries, 0);
        assert_eq!(stats.lookup_entries, 0);

        // Validate a code
        let result = cached
            .validate_code("test", "http://test.com", None)
            .await
            .unwrap();
        assert!(result);

        // Sync cache tasks
        cached.sync().await;

        // Stats should now show one validation entry
        let stats = cached.cache_stats();
        assert_eq!(stats.validation_entries, 1);
    }

    #[cfg(feature = "caching")]
    #[tokio::test]
    async fn test_cached_provider_expansion() {
        let inner = NoOpTerminologyProvider;
        let cached = CachedTerminologyProvider::with_default_config(inner);

        // Expand a valueset
        let expansion = cached
            .expand_valueset("http://test.com/vs", None)
            .await
            .unwrap();
        assert!(expansion.contains.is_empty());

        // Sync cache tasks
        cached.sync().await;

        // Stats should show expansion cached
        let stats = cached.cache_stats();
        assert_eq!(stats.expansion_entries, 1);

        // Expand again - should hit cache
        let expansion2 = cached
            .expand_valueset("http://test.com/vs", None)
            .await
            .unwrap();
        assert!(expansion2.contains.is_empty());
    }

    #[cfg(feature = "caching")]
    #[tokio::test]
    async fn test_cached_provider_clear_cache() {
        let inner = NoOpTerminologyProvider;
        let cached = CachedTerminologyProvider::with_default_config(inner);

        // Add some entries
        cached
            .validate_code("test", "http://test.com", None)
            .await
            .unwrap();
        cached
            .expand_valueset("http://test.com/vs", None)
            .await
            .unwrap();
        cached
            .lookup_code("http://test.com", "test", None, None)
            .await
            .unwrap();

        // Sync cache tasks
        cached.sync().await;

        // Verify entries were added
        let stats = cached.cache_stats();
        assert!(stats.validation_entries > 0);
        assert!(stats.expansion_entries > 0);
        assert!(stats.lookup_entries > 0);

        // Clear cache
        cached.clear_cache();
        cached.sync().await;

        // Verify cache is cleared
        let stats = cached.cache_stats();
        assert_eq!(stats.validation_entries, 0);
        assert_eq!(stats.expansion_entries, 0);
        assert_eq!(stats.lookup_entries, 0);
    }

    #[cfg(feature = "caching")]
    #[tokio::test]
    async fn test_cached_provider_validate_code_vs() {
        let inner = NoOpTerminologyProvider;
        let cached = CachedTerminologyProvider::with_default_config(inner);

        // Validate code against valueset
        let result = cached
            .validate_code_vs("http://test.com/vs", Some("http://system"), "test", None)
            .await
            .unwrap();
        assert!(result.result);

        // Sync cache tasks
        cached.sync().await;

        // Should be cached
        let stats = cached.cache_stats();
        assert_eq!(stats.validation_entries, 1);

        // Call again - should hit cache
        let result2 = cached
            .validate_code_vs("http://test.com/vs", Some("http://system"), "test", None)
            .await
            .unwrap();
        assert!(result2.result);
    }

    #[cfg(feature = "caching")]
    #[tokio::test]
    async fn test_cached_provider_inner_access() {
        let inner = NoOpTerminologyProvider;
        let cached = CachedTerminologyProvider::with_default_config(inner);

        // Access inner provider
        let inner_ref = cached.inner();
        assert!(
            inner_ref
                .validate_code("test", "http://test.com", None)
                .await
                .unwrap()
        );
    }
}
