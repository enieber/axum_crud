use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;

use crate::state::AppState;

use crate::models::task::{Model};
use crate::models::response::{ApiResponse};

#[derive(Template, WebTemplate)]
#[template(path = "tasks.html")]
pub struct TaskTemplate {
    pub tasks: Vec<Model>,
      pub page: i64,
      pub limit: i64,
      pub total: i64,
      pub error: Option<String>,
}

pub async fn tasks_page(State(state): State<AppState>) -> TaskTemplate {
    let url = match state.api_base.join("tasks") {
        Ok(u) => u,
        Err(e) => return TaskTemplate {
            tasks: vec![],
            page: 1,
            limit: 20,
            total: 0,
            error: Some(format!("URL inválida: {e}")),
        },
    };


    let result = state
           .http
           .get(url)
           .query(&[("page", "1"), ("limit", "20")])
           .send()
           .await;
    match result {
        Ok(resp) if resp.status().is_success() => {
            match resp.json::<ApiResponse<Vec<Model>>>().await {
                Ok(api) => TaskTemplate {
                    tasks: api.data,
                    page: 1,
                    limit: 20,
                    total: api.meta.unwrap().total,
                    error: None,
                },
                Err(e) => TaskTemplate {
                    tasks: vec![],
                    page: 1,
                    limit: 20,
                    total: 0,
                    error: Some(format!("JSON inválido: {e}")),
                },
            }
        }
        Ok(resp) => TaskTemplate {
            tasks: vec![],
            page: 1,
            limit: 20,
            total: 0,
            error: Some(format!("API retornou {}", resp.status())),
        },
        Err(e) => TaskTemplate {
            tasks: vec![],
            page: 1,
            limit: 20,
            total: 0,
            error: Some(format!("Falha na requisição: {e}")),
        },
    }

}
