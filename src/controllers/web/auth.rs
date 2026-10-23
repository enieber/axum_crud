use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect, Response},
};
use askama::Template;
use askama_web::WebTemplate;
use serde::{Deserialize};

use crate::state::AppState;
use axum_crud::views::form::{FormField, FieldKind};
use crate::controllers::web::form::FormTemplate;

pub async fn register_view() -> FormTemplate {
    FormTemplate {
        title: "Register".into(),
        action: "/auth/register".into(),
        submit_label: "Register".into(),
        fields: vec![
            FormField::text("name", "Nome", "")
                .required(),
            FormField::text("email", "Email", "")
                .required(),
            FormField::password("password", "Password", "")
                .required(),
        ],
        general_error: None,
    }
}

#[derive(Deserialize)]
pub struct UserFormInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn register_form(
    State(state): State<AppState>,
    Form(input): Form<UserFormInput>,
) -> impl IntoResponse {
    let name = input.name.trim().to_string();
    let email = input.email.trim().to_string();
    let password = input.password.trim().to_string();
    if name.is_empty() {
        return FormTemplate {
            title: "Register".into(),
            action: "/auth/register".into(),
            submit_label: "Register".into(),
            fields: vec![
                FormField::text("name", "Nome", input.name)
                    .required()
                    .error("O Nome não pode estar vazio."),
                FormField::text("email", "Email", input.email)
                    .required()
                    .error("O Email não pode estar vazio."),
                FormField::password("password", "Password", input.password)
                    .required()
                    .error("O Password não pode estar vazio."),
            ],
            general_error: None,
        }.into_response();
    }

    if email.is_empty() {
        return FormTemplate {
            title: "Register".into(),
            action: "/auth/register".into(),
            submit_label: "Register".into(),
            fields: vec![
                FormField::text("name", "Nome", input.name)
                    .required()
                    .error("O Nome não pode estar vazio."),
                FormField::text("email", "Email", input.email)
                    .required()
                    .error("O Email não pode estar vazio."),
                FormField::password("password", "Password", input.password)
                    .required()
                    .error("O Password não pode estar vazio."),
            ],
            general_error: None,
        }.into_response();
    }

    if password.is_empty() {
        return FormTemplate {
            title: "Register".into(),
            action: "/auth/register".into(),
            submit_label: "Register".into(),
            fields: vec![
                FormField::text("name", "Nome", input.name)
                    .required()
                    .error("O Nome não pode estar vazio."),
                FormField::text("email", "Email", input.email)
                    .required()
                    .error("O Email não pode estar vazio."),
                FormField::password("password", "Password", "")
                    .required()
                    .error("O Password não pode estar vazio."),
            ],
            general_error: None,
        }.into_response();
    }

    let url = match state.api_base.join("/api/v1/register") {
        Ok(u) => u,
        Err(e) => return render_error(format!("URL inválida: {e}"), input.name, input.email, input.password),
    };

    let payload = serde_json::json!({ "name": name, "email": email, "password": password });

    match state.http.post(url).json(&payload).send().await {
        Ok(resp) if resp.status().is_success() => Redirect::to("/tasks").into_response(),
        Ok(resp) => render_error(
            format!("API retornou {}", resp.status()),
            input.name,
            input.email,
            input.password
        ),
        Err(e) => render_error(format!("Falha na requisição: {e}"), input.name, input.email, input.password),
    }
}

fn render_error(msg: String, name: String, email: String, password: String) -> Response {
    FormTemplate {
        title: "Register".into(),
        action: "/auth/register".into(),
        submit_label: "Register".into(),
        fields: vec![
            FormField::text("name", "Nome", name).required(),
            FormField::text("email", "Email", email).required(),
            FormField::password("password", "Password", password).required(),
        ],
        general_error: Some(msg),
    }.into_response()
}
