use askama::Template;
use askama_web::WebTemplate;
use axum::{
    extract::{Form, Path, Query, State},
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::state::AppState;
use axum_crud::views::form::FormField;

use crate::client::tasks::TasksClient;
use crate::controllers::web::form::FormTemplate;
use crate::controllers::web::helpers::{form_error, redirect_tasks};
use crate::models::response::PaginationParams;
use crate::models::task::{CreateTask, Model, UpdateTask};

#[derive(Template, WebTemplate)]
#[template(path = "tasks.html")]
pub struct TaskTemplate {
    pub tasks: Vec<Model>,
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub error: Option<String>,
}

impl TaskTemplate {
    pub fn empty(page: u64, limit: u64, err: impl ToString) -> Self {
        Self {
            tasks: vec![],
            page,
            limit,
            total: 0,
            error: Some(err.to_string()),
        }
    }
}

pub async fn list_view(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> TaskTemplate {
    let page = params.page;
    let limit = params.limit_clamped();

    let client = match TasksClient::new(&state) {
        Ok(c) => c,
        Err(e) => return TaskTemplate::empty(page, limit, e),
    };

    match client.list(page, limit).await {
        Ok(api) => TaskTemplate {
            tasks: api.data,
            page,
            limit,
            total: api.meta.map(|m| m.total).unwrap_or(0),
            error: None,
        },
        Err(e) => TaskTemplate::empty(page, limit, e),
    }
}

pub async fn create_view() -> FormTemplate {
    FormTemplate {
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

pub async fn register_form(
    State(state): State<AppState>,
    Form(input): Form<TaskFormInput>,
) -> Response {
    let title = input.title.trim().to_string();
    let form = FormTemplate {
        title: "Nova tarefa".into(),
        action: "/tasks".into(),
        submit_label: "Criar".into(),
        fields: vec![FormField::text("title", "Título", input.title.clone()).required()],
        general_error: None,
    };

    if title.is_empty() {
        return form_error(form, "O título não pode estar vazio.");
    }

    let client = match TasksClient::new(&state) {
        Ok(c) => c,
        Err(e) => return form_error(form, e),
    };

    let payload = CreateTask { title };
    match client.create(&payload).await {
        Ok(_) => redirect_tasks(),
        Err(e) => form_error(form, e),
    }
}

#[derive(Deserialize)]
pub struct UpdateTaskFormInput {
    pub title: String,
    #[serde(default)]
    pub done: Option<String>,
}

pub async fn update_form(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(input): Form<UpdateTaskFormInput>,
) -> impl IntoResponse {
    let title = input.title.trim().to_string();
    let done = input.done.as_deref() == Some("true");

    let form = FormTemplate {
        title: "Editar tarefa".into(),
        action: "/tasks".into(),
        submit_label: "Savlar".into(),
        fields: vec![FormField::text("title", "Título", title.clone()).required()],
        general_error: None,
    };

    if title.is_empty() {
        return form_error(form, "O título não pode estar vazio.");
    }

    let client = match TasksClient::new(&state) {
        Ok(c) => c,
        Err(e) => return form_error(form, e),
    };

    let payload = UpdateTask {
        title: Some(title),
        done: Some(done),
    };
    match client.update(id, &payload).await {
        Ok(_) => redirect_tasks(),
        Err(e) => form_error(form, e),
    }
}

pub async fn update_view(State(state): State<AppState>, Path(id): Path<i32>) -> Response {
    let client = match TasksClient::new(&state) {
        Ok(c) => c,
        Err(err) => return FormTemplate::empty(Some(err.to_string())).into_response(),
    };

    let task = match client.find(id).await {
        Ok(api) => {
            tracing::info!("get id: {:?}", api);
            api.data
        }
        Err(err) => return FormTemplate::empty(Some(err.to_string())).into_response(),
    };

    FormTemplate {
        title: "Editar tarefa".into(),
        action: format!("/tasks/{id}"),
        submit_label: "Salvar".into(),
        fields: vec![
            FormField::text("title", "Título", task.title).required(),
            FormField::checkbox("done", "Concluída", task.done),
        ],
        general_error: None,
    }
    .into_response()
}

pub async fn delete_form(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let form = FormTemplate {
        title: "Editar tarefa".into(),
        action: format!("/tasks/{id}/delete"),
        submit_label: "Savlar".into(),
        fields: vec![FormField::text("title", "Título", "").required()],
        general_error: None,
    };

    let client = match TasksClient::new(&state) {
        Ok(c) => c,
        Err(e) => return form_error(form, e),
    };

    match client.delete(id).await {
        Ok(_) => redirect_tasks(),
        Err(e) => form_error(form, e),
    }
}
