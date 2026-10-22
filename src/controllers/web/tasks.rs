use axum::{
    extract::{Form, State, Query, Path},
    response::{IntoResponse, Redirect, Response},
};
use askama::Template;
use askama_web::WebTemplate;
use serde::Deserialize;

use crate::state::AppState;
use axum_crud::views::form::{FormField, FieldKind};

use crate::models::task::{Model};
use crate::models::response::{ApiResponse, PaginationParams};

#[derive(Template, WebTemplate)]
#[template(path = "tasks.html")]
pub struct TaskTemplate {
    pub tasks: Vec<Model>,
      pub page: u64,
      pub limit: u64,
      pub total: u64,
      pub error: Option<String>,
}

pub async fn tasks_page(State(state): State<AppState>, Query(params): Query<PaginationParams>) -> TaskTemplate {
    let page = params.page;
    let limit = params.limit_clamped();

    let url = match state.api_base.join("/api/v1/tasks") {
        Ok(u) => u,
        Err(e) => return TaskTemplate {
            tasks: vec![],
            page,
            limit,
            total: 0,
            error: Some(format!("URL inválida: {e}")),
        },
    };


    let result = state
           .http
           .get(url)
           .query(&[("page", page), ("limit", limit)])
           .send()
           .await;

    match result {
        Ok(resp) if resp.status().is_success() => {
            match resp.json::<ApiResponse<Vec<Model>>>().await {
                Ok(api) => TaskTemplate {
                    tasks: api.data,
                    page,
                    limit,
                    total: api.meta.unwrap().total,
                    error: None,
                },
                Err(e) => TaskTemplate {
                    tasks: vec![],
                    page,
                    limit,
                    total: 0,
                    error: Some(format!("JSON inválido: {e}")),
                },
            }
        }
        Ok(resp) => TaskTemplate {
            tasks: vec![],
            page,
            limit,
            total: 0,
            error: Some(format!("API retornou {}", resp.status())),
        },
        Err(e) => TaskTemplate {
            tasks: vec![],
            page,
            limit,
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

#[derive(Deserialize)]
pub struct UpdateTaskFormInput {
    pub title: String,
    #[serde(default)]
    pub done: Option<String>, // checkbox ausente = None
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(input): Form<UpdateTaskFormInput>,
) -> impl IntoResponse {
    let title = input.title.trim().to_string();
    let done = input.done.as_deref() == Some("true");

    if title.is_empty() {
        return edit_error(id, "O título não pode estar vazio.", input.title, done);
    }

    let url = match state.api_base.join(&format!("/api/v1/tasks/{id}")) {
        Ok(u) => u,
        Err(e) => return edit_error(id, &format!("URL inválida: {e}"), input.title, done),
    };

    let payload = serde_json::json!({
        "title": title,
        "done": done,
    });

    match state.http.put(url).json(&payload).send().await {
        Ok(resp) if resp.status().is_success() => Redirect::to("/tasks").into_response(),
        Ok(resp) => edit_error(id, &format!("API retornou {}", resp.status()), input.title, done),
        Err(e) => edit_error(id, &format!("Falha na requisição: {e}"), input.title, done),
    }
}

fn edit_error(id: i32, msg: &str, title: String, done: bool) -> Response {
    TaskFormTemplate {
        title: "Editar tarefa".into(),
        action: format!("/tasks/{id}"),
        submit_label: "Salvar".into(),
        fields: vec![
            FormField::text("title", "Título", title).required(),
            FormField::checkbox("done", "Concluída", done),
        ],
        general_error: Some(msg.into()),
    }.into_response()
}


pub async fn edit_task_page(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> TaskFormTemplate {
    let url = match state.api_base.join(&format!("/api/v1/tasks/{id}")) {
        Ok(u) => u,
        Err(e) => return error_form(format!("URL inválida: {e}")),
    };

    let task = match state.http.get(url).send().await {
        Ok(resp) if resp.status().is_success() => {
            match resp
                .json::<crate::models::response::ApiResponse<crate::models::task::Model>>()
                .await
            {
                Ok(api) => api.data,
                Err(e) => return error_form(format!("JSON inválido: {e}")),
            }
        }
        Ok(resp) => return error_form(format!("API retornou {}", resp.status())),
        Err(e) => return error_form(format!("Falha na requisição: {e}")),
    };

    TaskFormTemplate {
        title: "Editar tarefa".into(),
        action: format!("/tasks/{id}"),   // ✅ rota web de update
        submit_label: "Salvar".into(),
        fields: vec![
            FormField::text("title", "Título", task.title).required(),
            FormField::checkbox("done", "Concluída", task.done),
        ],
        general_error: None,
    }
}

fn error_form(msg: String) -> TaskFormTemplate {
    TaskFormTemplate {
        title: "Editar tarefa".into(),
        action: "#".into(),
        submit_label: "Salvar".into(),
        fields: vec![],
        general_error: Some(msg),
    }
}
