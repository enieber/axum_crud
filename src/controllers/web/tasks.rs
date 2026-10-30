use axum::{
    extract::{Form, Path, Query, State},
    response::{IntoResponse, Response},
};

use crate::client::tasks::TasksClient;
use crate::controllers::web::form::FormTemplate;
use crate::controllers::web::helpers::form_error;
use crate::controllers::web::list::ListTemplate;
use crate::generate_web_crud_handlers;
use crate::macros::config_web::WebConfig;
use crate::models::response::PaginationParams;
use crate::models::task::{CreateTask, Model as Task, UpdateTask};
use crate::state::AppState;

use axum_crud::views::form::FormField;
use axum_crud::views::list::{ListAction, ListItemView};

pub struct TaskConfig;

impl WebConfig for TaskConfig {
    type Model = Task;
    type Client = TasksClient;
    type CreateDto = CreateTask;
    type UpdateDto = UpdateTask;
    type Id = i32;

    const ROUTE_BASE: &'static str = "tasks";
    const TITLE_SINGULAR: &'static str = "Tarefa";
    const TITLE_PLURAL: &'static str = "Tarefas";

    fn build_list_item(model: &Self::Model) -> ListItemView {
        ListItemView::new(model.id.to_string())
            .with_fields(vec![
                FormField::checkbox("done", "Concluída", model.done),
                FormField::text("title", "Título", model.title.clone()),
            ])
            .with_actions(vec![
                ListAction::edit(&model.id.to_string(), Self::ROUTE_BASE),
                ListAction::delete(&model.id.to_string(), Self::ROUTE_BASE),
            ])
    }

    fn build_create_fields() -> Vec<FormField> {
        vec![
            FormField::text("title", "Título", "")
                .required()
                .placeholder("Digite o título da tarefa"),
        ]
    }

    fn build_update_fields(model: &Self::Model) -> Vec<FormField> {
        vec![
            FormField::text("title", "Título", model.title.clone()).required(),
            FormField::checkbox("done", "Concluída", model.done),
        ]
    }

    fn validate_create(dto: &Self::CreateDto) -> Option<String> {
        if dto.title.trim().is_empty() {
            Some("O título não pode estar vazio.".to_string())
        } else {
            None
        }
    }

    fn validate_update(dto: &Self::UpdateDto) -> Option<String> {
        if let Some(title) = &dto.title {
            if title.trim().is_empty() {
                return Some("O título não pode estar vazio.".to_string());
            }
        }
        None
    }
}

generate_web_crud_handlers!(TaskConfig);
