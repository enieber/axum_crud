use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
};
use askama::Template;
use askama_web::WebTemplate;
use serde::Deserialize;

use crate::state::AppState;
use axum_crud::views::form::{FormField, FieldKind};

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
    let url = match state.api_base.join("/api/v1/tasks") {
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

#[derive(Template, WebTemplate)]
#[template(path = "task_form.html")]
pub struct TaskFormTemplate {
    pub title: String,
    pub action: String,
    pub submit_label: String,
    pub fields: Vec<FormField>,
    pub general_error: Option<String>,
}


pub async fn new_task_page() -> TaskFormTemplate {
    TaskFormTemplate {
        title: "Nova tarefa".into(),
        action: "/tasks".into(),
        submit_label: "Criar".into(),
        fields: vec![
            FormField::text("title", "Título", "")
                .required()
                .placeholder("Digite o título da tarefa"),
        ],
        general_error: None,
    }
}


#[derive(Deserialize)]
pub struct TaskFormInput {
    pub title: String,
}

pub async fn create_task(
    State(state): State<AppState>,
    Form(input): Form<TaskFormInput>,
) -> impl IntoResponse {
    let title = input.title.trim().to_string();
    if title.is_empty() {
        return TaskFormTemplate {
            title: "Nova tarefa".into(),
            action: "/tasks".into(),
            submit_label: "Criar".into(),
            fields: vec![
                FormField::text("title", "Título", input.title)
                    .required()
                    .error("O título não pode estar vazio."),
            ],
            general_error: None,
        }.into_response();
    }

    let url = match state.api_base.join("/api/v1/tasks") {
        Ok(u) => u,
        Err(e) => return render_error(format!("URL inválida: {e}"), input.title),
    };

    let payload = serde_json::json!({ "title": title });

    match state.http.post(url).json(&payload).send().await {
        Ok(resp) if resp.status().is_success() => Redirect::to("/tasks").into_response(),
        Ok(resp) => render_error(
            format!("API retornou {}", resp.status()),
            input.title,
        ),
        Err(e) => render_error(format!("Falha na requisição: {e}"), input.title),
    }
}

fn render_error(msg: String, title: String) -> axum::response::Response {
    TaskFormTemplate {
        title: "Nova tarefa".into(),
        action: "/tasks".into(),
        submit_label: "Criar".into(),
        fields: vec![
            FormField::text("title", "Título", title).required(),
        ],
        general_error: Some(msg),
    }.into_response()
}
