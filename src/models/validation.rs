use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationErrors(pub Vec<FieldError>);

impl ValidationErrors {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, field: &str, message: impl Into<String>) {
        self.0.push(FieldError {
            field: field.into(),
            message: message.into(),
        });
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Converte para o JSON que vai no `details` do ErrorResponse.
    pub fn to_details(&self) -> serde_json::Value {
        let map: serde_json::Map<String, serde_json::Value> = self
            .0
            .iter()
            .map(|e| (e.field.clone(), serde_json::Value::String(e.message.clone())))
            .collect();
        serde_json::Value::Object(map)
    }
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorValidation {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorValidation {
    pub fn validation(errors: &ValidationErrors) -> Self {
        Self {
            error: "VALIDATION_ERROR".into(),
            message: "Erro de validação".into(),
            details: Some(errors.to_details()),
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self {
            error: "CONFLICT".into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn internal(message: impl Into<String>, detail: Option<String>) -> Self {
        Self {
            error: "INTERNAL_ERROR".into(),
            message: message.into(),
            details: detail.map(|d| serde_json::json!({ "raw": d })),
        }
    }
}
