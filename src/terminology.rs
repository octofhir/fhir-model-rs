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
                    if let Some(name) = param.get("name").and_then(|n| n.as_str()) {
                        if name == "result" {
                            return Ok(param
                                .get("valueBoolean")
                                .and_then(|b| b.as_bool())
                                .unwrap_or(false));
                        }
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
                    if let Some(name) = param.get("name").and_then(|n| n.as_str()) {
                        if name == "outcome" {
                            if let Some(outcome_str) =
                                param.get("valueCode").and_then(|v| v.as_str())
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
}
