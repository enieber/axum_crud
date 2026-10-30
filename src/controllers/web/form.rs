use askama::Template;
use askama_web::WebTemplate;

use axum_crud::views::form::*;

#[derive(Template, WebTemplate)]
#[template(path = "form.html")]
pub struct FormTemplate {
    pub title: String,
    pub action: String,
    pub base_action: String,
    pub submit_label: String,
    pub fields: Vec<FormField>,
    pub general_error: Option<String>,
}

impl FormTemplate {
    pub fn empty(err: Option<String>) -> Self {
        Self {
            title: "Form".to_string(),
            base_action: "/".to_string(),
            action: "form".to_string(),
            submit_label: "save".to_string(),
            fields: vec![],
            general_error: err,
        }
    }
}
