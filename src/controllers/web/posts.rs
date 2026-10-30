use axum::{
    extract::{Form, Path, Query, State},
    response::{IntoResponse, Response},
};

use axum_crud::views::form::FormField;
use axum_crud::views::list::{ListAction, ListItemView};

use crate::client::posts::PostsClient;
use crate::controllers::web::form::FormTemplate;
use crate::controllers::web::helpers::form_error;
use crate::controllers::web::list::ListTemplate;
use crate::generate_web_crud_handlers;
use crate::macros::config_web::WebConfig;
use crate::models::posts::{CreatePost, Model as Post, UpdatePost};
use crate::models::response::PaginationParams;
use crate::state::AppState;

pub struct PostConfig;

impl WebConfig for PostConfig {
    type Model = Post;
    type Client = PostsClient;
    type CreateDto = CreatePost;
    type UpdateDto = UpdatePost;
    type Id = i32;

    const ROUTE_BASE: &'static str = "posts";
    const TITLE_SINGULAR: &'static str = "Post";
    const TITLE_PLURAL: &'static str = "Posts";

    fn build_list_item(model: &Self::Model) -> ListItemView {
        ListItemView::new(model.id.to_string())
            .with_fields(vec![
                FormField::text("title", "Título", model.title.clone()),
                FormField::textarea("description", "Descrição", model.description.clone()),
                FormField::text("slug", "Slug", model.slug.clone()),
                FormField::text("body", "Conteudo", model.body.clone()),
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
            FormField::textarea("description", "Descrição", "")
                .required()
                .placeholder("Digite a decrição"),
            FormField::text("slug", "Slug", "")
                .required()
                .placeholder("Slug do post"),
            FormField::text("body", "Conteudo", "")
                .required()
                .placeholder("Digite o conteudo do post"),
        ]
    }

    fn build_update_fields(model: &Self::Model) -> Vec<FormField> {
        vec![
            FormField::text("title", "Título", model.title.clone()).required(),
            FormField::textarea("description", "Descrição", model.description.clone()).required(),
            FormField::text("slug", "Slug", model.slug.clone()).required(),
            FormField::text("body", "Conteudo", model.body.clone()).required(),
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

generate_web_crud_handlers!(PostConfig);
