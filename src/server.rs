//! FHIR Server API abstractions for FHIRPath %server variable
//!
//! This module defines the `ServerProvider` trait that enables FHIRPath expressions
//! to interact with FHIR servers for CRUD operations, search, and server operations.

use async_trait::async_trait;
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::error::Result;

/// Server provider trait for FHIR RESTful API operations
///
/// Implementations of this trait provide access to FHIR server operations
/// that can be invoked from FHIRPath expressions via the `%server` variable.
///
/// All methods return `Result<Option<JsonValue>>` where:
/// - `Ok(Some(value))` = successful operation with result
/// - `Ok(None)` = operation completed but no result (e.g., not found, failed)
/// - `Err(e)` = operation could not be attempted (connectivity, configuration errors)
#[async_trait]
pub trait ServerProvider: Send + Sync + std::fmt::Debug {
    /// Read a resource by type and id
    ///
    /// Corresponds to `GET [base]/[type]/[id]`
    async fn read(&self, resource_type: &str, id: &str) -> Result<Option<JsonValue>>;

    /// Create a new resource
    ///
    /// Corresponds to `POST [base]/[type]`
    /// The resource id, if present, will be ignored by the server.
    async fn create(&self, resource: &JsonValue) -> Result<Option<JsonValue>>;

    /// Update an existing resource
    ///
    /// Corresponds to `PUT [base]/[type]/[id]`
    /// The resource must have an id.
    async fn update(&self, resource: &JsonValue) -> Result<Option<JsonValue>>;

    /// Delete a resource
    ///
    /// Corresponds to `DELETE [base]/[type]/[id]`
    /// Returns true if successfully deleted, false otherwise.
    async fn delete(&self, resource: &JsonValue) -> Result<bool>;

    /// Search for resources
    ///
    /// - `do_post`: if true, use POST-based search; otherwise GET
    /// - `parameters`: either a Parameters resource (JSON) or URL-encoded query string
    ///
    /// Corresponds to `GET/POST [base]/[type]?[parameters]`
    async fn search(&self, do_post: bool, parameters: &JsonValue) -> Result<Option<JsonValue>>;

    /// Apply a FHIR Patch operation
    ///
    /// - `parameters`: Parameters resource describing the patch
    ///
    /// Corresponds to `PATCH [base]/[type]/[id]`
    async fn patch(&self, parameters: &JsonValue) -> Result<Option<JsonValue>>;

    /// Get server capabilities
    ///
    /// - `mode`: optional mode string for the capabilities operation
    ///
    /// Corresponds to `GET [base]/metadata`
    async fn capabilities(&self, mode: Option<&str>) -> Result<Option<JsonValue>>;

    /// Validate a resource
    ///
    /// - `resource`: the resource to validate
    /// - `mode`: validation mode
    /// - `parameters`: additional parameters
    ///
    /// Corresponds to `POST [base]/[type]/$validate`
    async fn validate(
        &self,
        resource: &JsonValue,
        mode: &str,
        parameters: &JsonValue,
    ) -> Result<Option<JsonValue>>;

    /// Transform a resource using a StructureMap
    ///
    /// - `source`: the StructureMap to use
    /// - `content`: the resource/content to transform
    ///
    /// Corresponds to `POST [base]/$transform`
    async fn transform(&self, source: &JsonValue, content: &JsonValue)
    -> Result<Option<JsonValue>>;

    /// Retrieve all related resources ($everything)
    ///
    /// - `resource_type`: resource type
    /// - `id`: resource id
    /// - `parameters`: additional parameters
    ///
    /// Corresponds to `GET [base]/[type]/[id]/$everything`
    async fn everything(
        &self,
        resource_type: &str,
        id: &str,
        parameters: &JsonValue,
    ) -> Result<Option<JsonValue>>;

    /// Apply a PlanDefinition or ActivityDefinition
    ///
    /// - `resource`: the definition resource
    /// - `subject`: subject reference (type/id)
    /// - `parameters`: additional parameters
    ///
    /// Corresponds to `POST [base]/[type]/[id]/$apply`
    async fn apply(
        &self,
        resource: &JsonValue,
        subject: &str,
        parameters: &JsonValue,
    ) -> Result<Option<JsonValue>>;

    /// Get the base URL of this server
    fn base_url(&self) -> &str;

    /// Create a new provider instance pointing at a different base URL.
    /// Used by %server.at(url) to create providers for different endpoints.
    /// Default returns None (provider does not support URL switching).
    fn with_base_url(&self, _url: &str) -> Option<Arc<dyn ServerProvider>> {
        None
    }
}

/// No-op server provider that returns empty/error for all operations
///
/// Used as a default when no server is configured.
#[derive(Debug, Default, Clone)]
pub struct NoOpServerProvider;

#[async_trait]
impl ServerProvider for NoOpServerProvider {
    async fn read(&self, _resource_type: &str, _id: &str) -> Result<Option<JsonValue>> {
        Ok(None)
    }

    async fn create(&self, _resource: &JsonValue) -> Result<Option<JsonValue>> {
        Ok(None)
    }

    async fn update(&self, _resource: &JsonValue) -> Result<Option<JsonValue>> {
        Ok(None)
    }

    async fn delete(&self, _resource: &JsonValue) -> Result<bool> {
        Ok(false)
    }

    async fn search(&self, _do_post: bool, _parameters: &JsonValue) -> Result<Option<JsonValue>> {
        Ok(None)
    }

    async fn patch(&self, _parameters: &JsonValue) -> Result<Option<JsonValue>> {
        Ok(None)
    }

    async fn capabilities(&self, _mode: Option<&str>) -> Result<Option<JsonValue>> {
        Ok(None)
    }

    async fn validate(
        &self,
        _resource: &JsonValue,
        _mode: &str,
        _parameters: &JsonValue,
    ) -> Result<Option<JsonValue>> {
        Ok(None)
    }

    async fn transform(
        &self,
        _source: &JsonValue,
        _content: &JsonValue,
    ) -> Result<Option<JsonValue>> {
        Ok(None)
    }

    async fn everything(
        &self,
        _resource_type: &str,
        _id: &str,
        _parameters: &JsonValue,
    ) -> Result<Option<JsonValue>> {
        Ok(None)
    }

    async fn apply(
        &self,
        _resource: &JsonValue,
        _subject: &str,
        _parameters: &JsonValue,
    ) -> Result<Option<JsonValue>> {
        Ok(None)
    }

    fn base_url(&self) -> &str {
        ""
    }
}

/// HTTP-based FHIR server provider
///
/// Implements `ServerProvider` by making HTTP requests to a FHIR REST API.
/// Requires the `http-client` feature.
#[cfg(feature = "http-client")]
#[derive(Debug)]
pub struct HttpServerProvider {
    /// HTTP client for making requests
    client: reqwest::Client,
    /// Base URL of the FHIR server (e.g., "https://hapi.fhir.org/baseR4")
    base_url: String,
}

#[cfg(feature = "http-client")]
impl HttpServerProvider {
    /// Create a new HttpServerProvider with the given base URL
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
        })
    }

    /// Helper to extract resourceType and id from a resource JSON
    fn extract_type_and_id(resource: &JsonValue) -> Option<(String, String)> {
        let resource_type = resource.get("resourceType")?.as_str()?.to_string();
        let id = resource.get("id")?.as_str()?.to_string();
        Some((resource_type, id))
    }

    /// Parse a JSON response, returning None for non-success status codes
    async fn parse_response(&self, response: reqwest::Response) -> Result<Option<JsonValue>> {
        if response.status().is_success() {
            let json: JsonValue = response.json().await.map_err(|e| {
                crate::error::ModelError::schema_load_error(format!(
                    "Failed to parse server response: {e}"
                ))
            })?;
            Ok(Some(json))
        } else {
            Ok(None)
        }
    }
}

#[cfg(feature = "http-client")]
#[async_trait]
impl ServerProvider for HttpServerProvider {
    async fn read(&self, resource_type: &str, id: &str) -> Result<Option<JsonValue>> {
        let url = format!("{}/{resource_type}/{id}", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("Accept", "application/fhir+json")
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::schema_load_error(format!("Server read failed: {e}"))
            })?;
        self.parse_response(response).await
    }

    async fn create(&self, resource: &JsonValue) -> Result<Option<JsonValue>> {
        let resource_type = resource
            .get("resourceType")
            .and_then(|rt| rt.as_str())
            .unwrap_or("Resource");
        let url = format!("{}/{resource_type}", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/fhir+json")
            .header("Accept", "application/fhir+json")
            .json(resource)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::schema_load_error(format!("Server create failed: {e}"))
            })?;
        self.parse_response(response).await
    }

    async fn update(&self, resource: &JsonValue) -> Result<Option<JsonValue>> {
        let (resource_type, id) = Self::extract_type_and_id(resource).ok_or_else(|| {
            crate::error::ModelError::schema_load_error(
                "Resource must have resourceType and id for update".to_string(),
            )
        })?;
        let url = format!("{}/{resource_type}/{id}", self.base_url);
        let response = self
            .client
            .put(&url)
            .header("Content-Type", "application/fhir+json")
            .header("Accept", "application/fhir+json")
            .json(resource)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::schema_load_error(format!("Server update failed: {e}"))
            })?;
        self.parse_response(response).await
    }

    async fn delete(&self, resource: &JsonValue) -> Result<bool> {
        let (resource_type, id) = Self::extract_type_and_id(resource).ok_or_else(|| {
            crate::error::ModelError::schema_load_error(
                "Resource must have resourceType and id for delete".to_string(),
            )
        })?;
        let url = format!("{}/{resource_type}/{id}", self.base_url);
        let response = self.client.delete(&url).send().await.map_err(|e| {
            crate::error::ModelError::schema_load_error(format!("Server delete failed: {e}"))
        })?;
        Ok(response.status().is_success())
    }

    async fn search(&self, do_post: bool, parameters: &JsonValue) -> Result<Option<JsonValue>> {
        // Extract resource type from parameters if available
        let resource_type = parameters
            .get("resourceType")
            .and_then(|rt| rt.as_str())
            .or_else(|| parameters.get("_type").and_then(|t| t.as_str()))
            .unwrap_or("");

        let url = if resource_type.is_empty() {
            format!("{}/_search", self.base_url)
        } else {
            format!("{}/{resource_type}", self.base_url)
        };

        let response = if do_post {
            self.client
                .post(&url)
                .header("Content-Type", "application/fhir+json")
                .header("Accept", "application/fhir+json")
                .json(parameters)
                .send()
                .await
        } else {
            // Convert parameters to query string
            let mut query_parts = Vec::new();
            if let Some(obj) = parameters.as_object() {
                for (key, value) in obj {
                    if key != "resourceType"
                        && key != "_type"
                        && let Some(s) = value.as_str()
                    {
                        query_parts.push(format!("{key}={s}"));
                    }
                }
            }
            let full_url = if query_parts.is_empty() {
                url
            } else {
                format!("{url}?{}", query_parts.join("&"))
            };
            self.client
                .get(&full_url)
                .header("Accept", "application/fhir+json")
                .send()
                .await
        };

        let response = response.map_err(|e| {
            crate::error::ModelError::schema_load_error(format!("Server search failed: {e}"))
        })?;
        self.parse_response(response).await
    }

    async fn patch(&self, parameters: &JsonValue) -> Result<Option<JsonValue>> {
        // Extract target resource info from parameters
        let resource_type = parameters
            .get("resourceType")
            .and_then(|rt| rt.as_str())
            .unwrap_or("Resource");
        let id = parameters
            .get("id")
            .and_then(|id| id.as_str())
            .unwrap_or("");
        let url = format!("{}/{resource_type}/{id}", self.base_url);

        let response = self
            .client
            .patch(&url)
            .header("Content-Type", "application/fhir+json")
            .header("Accept", "application/fhir+json")
            .json(parameters)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::schema_load_error(format!("Server patch failed: {e}"))
            })?;
        self.parse_response(response).await
    }

    async fn capabilities(&self, mode: Option<&str>) -> Result<Option<JsonValue>> {
        let mut url = format!("{}/metadata", self.base_url);
        if let Some(mode) = mode {
            url = format!("{url}?mode={mode}");
        }
        let response = self
            .client
            .get(&url)
            .header("Accept", "application/fhir+json")
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::schema_load_error(format!(
                    "Server capabilities failed: {e}"
                ))
            })?;
        self.parse_response(response).await
    }

    async fn validate(
        &self,
        resource: &JsonValue,
        mode: &str,
        parameters: &JsonValue,
    ) -> Result<Option<JsonValue>> {
        let resource_type = resource
            .get("resourceType")
            .and_then(|rt| rt.as_str())
            .unwrap_or("Resource");
        let url = format!("{}/{resource_type}/$validate?mode={mode}", self.base_url);

        // Build Parameters resource with the resource to validate
        let params = serde_json::json!({
            "resourceType": "Parameters",
            "parameter": [
                {"name": "resource", "resource": resource},
                {"name": "mode", "valueCode": mode}
            ]
        });

        let _ = parameters; // Additional parameters could be merged in future

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/fhir+json")
            .header("Accept", "application/fhir+json")
            .json(&params)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::schema_load_error(format!("Server validate failed: {e}"))
            })?;
        self.parse_response(response).await
    }

    async fn transform(
        &self,
        source: &JsonValue,
        content: &JsonValue,
    ) -> Result<Option<JsonValue>> {
        let url = format!("{}/$transform", self.base_url);
        let params = serde_json::json!({
            "resourceType": "Parameters",
            "parameter": [
                {"name": "source", "resource": source},
                {"name": "content", "resource": content}
            ]
        });

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/fhir+json")
            .header("Accept", "application/fhir+json")
            .json(&params)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::schema_load_error(format!("Server transform failed: {e}"))
            })?;
        self.parse_response(response).await
    }

    async fn everything(
        &self,
        resource_type: &str,
        id: &str,
        parameters: &JsonValue,
    ) -> Result<Option<JsonValue>> {
        let mut url = format!("{}/{resource_type}/{id}/$everything", self.base_url);

        // Add query parameters
        if let Some(obj) = parameters.as_object() {
            let query_parts: Vec<String> = obj
                .iter()
                .filter_map(|(k, v)| v.as_str().map(|s| format!("{k}={s}")))
                .collect();
            if !query_parts.is_empty() {
                url = format!("{url}?{}", query_parts.join("&"));
            }
        }

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/fhir+json")
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::schema_load_error(format!(
                    "Server everything failed: {e}"
                ))
            })?;
        self.parse_response(response).await
    }

    async fn apply(
        &self,
        resource: &JsonValue,
        subject: &str,
        parameters: &JsonValue,
    ) -> Result<Option<JsonValue>> {
        let resource_type = resource
            .get("resourceType")
            .and_then(|rt| rt.as_str())
            .unwrap_or("PlanDefinition");
        let id = resource.get("id").and_then(|id| id.as_str()).unwrap_or("");
        let url = format!("{}/{resource_type}/{id}/$apply", self.base_url);

        let params = serde_json::json!({
            "resourceType": "Parameters",
            "parameter": [
                {"name": "subject", "valueString": subject}
            ]
        });

        let _ = parameters; // Additional parameters could be merged in future

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/fhir+json")
            .header("Accept", "application/fhir+json")
            .json(&params)
            .send()
            .await
            .map_err(|e| {
                crate::error::ModelError::schema_load_error(format!("Server apply failed: {e}"))
            })?;
        self.parse_response(response).await
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn with_base_url(&self, url: &str) -> Option<Arc<dyn ServerProvider>> {
        HttpServerProvider::new(url.to_string())
            .ok()
            .map(|p| Arc::new(p) as Arc<dyn ServerProvider>)
    }
}
