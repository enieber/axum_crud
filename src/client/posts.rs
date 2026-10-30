use crate::client::http::ApiError;
use crate::client::resource::Resource;
use crate::models::posts::{CreatePost, Model, UpdatePost};
use crate::models::response::ApiResponse;
use crate::state::AppState;

pub struct PostsClient(Resource);

impl PostsClient {
    pub fn new(state: &AppState) -> Result<Self, ApiError> {
        Ok(Self(Resource::new(state, "api/v1/posts")?))
    }

    pub async fn list(&self, page: u64, limit: u64) -> Result<ApiResponse<Vec<Model>>, ApiError> {
        self.0.list(&[("page", page), ("limit", limit)]).await
    }

    pub async fn find(&self, id: i32) -> Result<ApiResponse<Model>, ApiError> {
        self.0.get(id).await
    }

    pub async fn create(&self, body: &CreatePost) -> Result<ApiResponse<Model>, ApiError> {
        self.0.create(body).await
    }

    pub async fn update(&self, id: i32, body: &UpdatePost) -> Result<ApiResponse<Model>, ApiError> {
        self.0.update(id, body).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), ApiError> {
        self.0.delete(id).await
    }
}
