use askama::Template;
use askama_web::WebTemplate;

use axum_crud::views::form::*;
use axum_crud::views::list::*;

#[derive(Template, WebTemplate)]
#[template(path = "list.html")]
pub struct ListTemplate {
    pub title: String,
    pub list: Vec<ListItemView>,
    pub page_name: String,
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub error: Option<String>,
}

impl ListTemplate {
    pub fn new(
        title: impl Into<String>,
        list: Vec<ListItemView>,
        page_name: impl Into<String>,
        page: u64,
        limit: u64,
        total: u64,
    ) -> Self {
        Self {
            title: title.into(),
            list,
            page_name: page_name.into(),
            page,
            limit,
            total,
            error: None,
        }
    }

    pub fn empty(page: u64, limit: u64, err: impl ToString) -> Self {
        Self {
            title: "Lista".to_string(),
            list: vec![],
            page_name: "items".to_string(),
            page,
            limit,
            total: 0,
            error: Some(err.to_string()),
        }
    }
}
