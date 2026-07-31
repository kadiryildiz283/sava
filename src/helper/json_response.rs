// ============================================================================
// Module: JSON Response Handler
// Single Responsibility: Standardizes all system outputs into uniform JSON responses.
//
// EXAMPLE JSON OUTPUT SCHEMA:
// {
//   "status": "SUCCESS",       // or "ERROR"
//   "code": 200,               // HTTP/Execution Status Code
//   "message": "Action completed successfully",
//   "data": { ... },           // Payload data or null
//   "error_details": null      // Stack trace / error message or null
// }
// ============================================================================

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonResponse {
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: Option<Value>,
    pub error_details: Option<String>,
}

impl JsonResponse {
    pub fn success(message: &str, data: Option<Value>) -> Self {
        Self {
            status: "SUCCESS".to_string(),
            code: 200,
            message: message.to_string(),
            data,
            error_details: None,
        }
    }

    pub fn error(code: u16, message: &str, error_details: Option<&str>) -> Self {
        Self {
            status: "ERROR".to_string(),
            code,
            message: message.to_string(),
            data: None,
            error_details: error_details.map(|s| s.to_string()),
        }
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(self)
            .unwrap_or_else(|_| json!({"status": "ERROR", "code": 500, "message": "Failed to serialize JSON response"}).to_string())
    }
}
