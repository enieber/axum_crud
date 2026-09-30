use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, EntityTrait, Set,
};
use crate::models::task;
use crate::models::task::{Model, UpdateTask, CreateTask};
use crate::state::AppState;

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<Model>>, (StatusCode, String)> {
    let task_list = task::Entity::find()
        .all(&state.db)
        .await
        .map_err(|err| {
            tracing::error!("Erro ao buscar tasks no banco: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Falha ao buscar dados: {}", err),
            )
        })?;

    Ok(Json(task_list))
}

pub async fn create(State(state): State<AppState>, Json(payload): Json<CreateTask>) -> Result<(StatusCode, Json<Model>), StatusCode> {
    let task = task::ActiveModel {
        title: Set(payload.title),
        done: Set(false),
        ..Default::default()
    };

    let task_return = task
            .insert(&state.db)
            .await
            .map_err(|e| {
                        if e.to_string().contains("duplicate key") || e.to_string().contains("unique constraint") {
                            StatusCode::CONFLICT
                        } else {
                            StatusCode::INTERNAL_SERVER_ERROR
                        }
            })?;
    Ok((StatusCode::CREATED, Json(task_return.into())))
}

pub async fn get_one(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Model>, StatusCode> {
    let task = task::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(task.into()))
}


pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Model>, StatusCode> {
        let task = task::Entity::find_by_id(id)
                    .one(&state.db)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                    .ok_or(StatusCode::NOT_FOUND)?;

        let mut task: task::ActiveModel = task.into();

       if let Some(title) = payload.title {
           task.title = Set(title)
       }

       if let Some(done) = payload.done {
           task.done = Set(done)
       }

       let task = task
                    .update(&state.db)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

       Ok(Json(task.into()))
}


pub async fn remove(State(state): State<AppState>, Path(id): Path<i32>) -> Result<StatusCode, StatusCode> {
    let task = task::Entity::find_by_id(id)
                .one(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::NOT_FOUND)?;

    let task: task::ActiveModel = task.into();

    task.delete(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
