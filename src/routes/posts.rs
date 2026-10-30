use crate::generate_crud;
use crate::models::posts::{CreatePost, Entity, Model, UpdatePost};
use crate::models::response::ApiResponse;

generate_crud!(
    task,
    entity: Entity,
    model: Model,
    create_dto: CreatePost,
    update_dto: UpdatePost,
    resource_name: "posts",
    tag: "Posts",
    id_type: i32,
    id_field: id
);
