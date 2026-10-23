use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect, Response},
};
use askama::Template;
use askama_web::WebTemplate;
use serde::{Deserialize};

use crate::state::AppState;
use axum_crud::views::form::{FormField, FieldKind};

#[derive(Template, WebTemplate)]
#[template(path = "form.html")]
pub struct FormTemplate {
    pub title: String,
    pub action: String,
    pub submit_label: String,
    pub fields: Vec<FormField>,
    pub general_error: Option<String>,
}
