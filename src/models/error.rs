use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>, // Opcional: para validações detalhadas
}

impl ErrorResponse {
    pub fn not_found(resource: &str, id: impl std::fmt::Display) -> Self {
        Self {
            error: "NOT_FOUND".to_string(),
            message: format!("{} com ID {} não encontrado", resource, id),
            details: None,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            error: "BAD_REQUEST".to_string(),
            message: message.into(),
            details: None,
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self {
            error: "CONFLICT".to_string(),
            message: message.into(),
            details: None,
        }
    }

    pub fn internal(message: impl Into<String>, detail: Option<String>) -> Self {
        Self {
            error: "INTERNAL_ERROR".to_string(),
            message: message.into(),
            details: detail.map(|d| serde_json::json!({ "raw": d })),
        }
    }
}
