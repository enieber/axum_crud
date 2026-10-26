use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_limit")]
    pub limit: u64,
}

fn default_page() -> u64 {
    1
}
fn default_limit() -> u64 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
        }
    }
}

impl PaginationParams {
    pub const MIN_LIMIT: u64 = 1;
    pub const MAX_LIMIT: u64 = 100;

    pub fn limit_clamped(&self) -> u64 {
        self.limit.clamp(Self::MIN_LIMIT, Self::MAX_LIMIT)
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ApiResponse<T: ToSchema> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Meta {
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

impl<T: ToSchema> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { data, meta: None }
    }

    pub fn paginated(data: T, total: u64, page: u64, limit: u64) -> Self {
        Self {
            data,
            meta: Some(Meta {
                total,
                page,
                limit,
                total_pages: (total as f64 / limit as f64).ceil() as u64,
            }),
        }
    }
}
