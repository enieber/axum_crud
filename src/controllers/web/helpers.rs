use crate::controllers::web::form::FormTemplate;
use axum::response::{IntoResponse, Redirect, Response};

pub fn form_error(form: FormTemplate, msg: impl ToString) -> Response {
    FormTemplate {
        general_error: Some(msg.to_string()),
        ..form
    }
    .into_response()
}

pub fn redirect_tasks() -> Response {
    Redirect::to("/tasks").into_response()
}
