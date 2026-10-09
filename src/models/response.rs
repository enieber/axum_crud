use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_limit")]
    pub limit: u64,
}

fn default_page() -> u64 { 1 }
fn default_limit() -> u64 { 20 }

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T: ToSchema> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Serialize, ToSchema)]
pub struct Meta {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

impl<T: ToSchema> ApiResponse<T> {
    /// Resposta simples sem paginação
    pub fn success(data: T) -> Self {
        Self { data, meta: None }
    }

    /// Resposta com metadados de paginação
    pub fn paginated(data: T, total: i64, page: i64, limit: i64) -> Self {
        Self {
            data,
            meta: Some(Meta {
                total,
                page,
                limit,
                total_pages: (total as f64 / limit as f64).ceil() as i64,
            }),
        }
    }
}
