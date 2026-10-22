use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use sea_orm::{
    ActiveModelTrait, EntityTrait, Set,
};
use crate::models::task;
use crate::models::task::{Model, UpdateTask, CreateTask};
use crate::state::AppState;
use crate::models::error::{ErrorResponse};
use crate::models::response::{ApiResponse, PaginationParams};
use sea_orm::PaginatorTrait;
use utoipa_axum::{router::OpenApiRouter, routes};

#[utoipa::path(
    get,
    path = "/tasks",
    tag = "Tasks",
    params(
            ("page" = u64, Query, description = "Número da página (padrão: 1)"),
            ("limit" = u64, Query, description = "Itens por página (padrão: 20)")
        ),
        responses(
            (status = 200, description = "Lista de tarefas paginada", body = ApiResponse<Vec<Model>>),
            (status = 500, description = "Erro interno", body = ErrorResponse)
        )
)]
pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<Model>>>, (StatusCode, Json<ErrorResponse>)> {
    let paginator = task::Entity::find()
        .paginate(&state.db, params.limit);

    let (tasks, total) = paginator
        .fetch_page(params.page.saturating_sub(1))
        .await
        .and_then(|items| {
            Ok((items, paginator.num_items_and_pages()))
        })
        .map_err(|err| {
            tracing::error!("Erro ao buscar tasks no banco: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal("Falha ao buscar tarefas", Some(err.to_string()))),
            )
        })?;
    let total = total.await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal("Falha ao contar registros", Some(e.to_string()))),
        )
    })?.number_of_items;

    Ok(Json(ApiResponse::paginated(
        tasks,
        total as i64,
        params.page as i64,
        params.limit as i64,
    )))

}

#[utoipa::path(
    post,
    path = "/tasks",
    tag = "Tasks",
    request_body = CreateTask,
    responses(
        (status = 201, description = "Task created successfully", body = ApiResponse<Model>),
        (status = 409, description = "Conflict (Duplicate key)", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn create(State(state): State<AppState>, Json(payload): Json<CreateTask>) -> Result<(StatusCode, Json<ApiResponse<Model>>), (StatusCode, Json<ErrorResponse>)> {
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
                            (
                                StatusCode::CONFLICT,
                                Json(ErrorResponse::conflict("Erro ao consultar o banco de dados"))
                            )
                        } else {
                            (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse::internal("Erro ao consultar o banco de dados", Some(e.to_string())))
                            )
                        }
            })?;
    Ok((StatusCode::CREATED, Json(ApiResponse::success(task_return.into()))))
}

#[utoipa::path(
    get,
    path = "/tasks/{id}",
    tag = "Tasks",
    params(
        ("id" = i32, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task found", body = ApiResponse<Model>),
        (status = 404, description = "Task not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn get_one(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<ApiResponse<Model>>, (StatusCode, Json<ErrorResponse>)> {
    let task = task::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e|
            (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal("Erro ao consultar o banco de dados", Some(e.to_string())))
            )
        )?
        .ok_or(
            (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::not_found("tasks", id))
            )
        )?;

    Ok(Json(ApiResponse::success(task.into())))
}

#[utoipa::path(
    put, // ou patch, dependendo de como você registrou a rota no seu router
    path = "/tasks/{id}",
    tag = "Tasks",
    params(
        ("id" = i32, Path, description = "Task ID")
    ),
    request_body = UpdateTask,
    responses(
        (status = 200, description = "Task updated", body = ApiResponse<Model>),
        (status = 404, description = "Task not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<ApiResponse<Model>>, (StatusCode, Json<ErrorResponse>)> {
        let task = task::Entity::find_by_id(id)
                    .one(&state.db)
                    .await
                    .map_err(|e|
                        (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse::internal("Erro ao consultar o banco de dados", Some(e.to_string())))
                        )
                    )?
                    .ok_or(
                        (
                        StatusCode::NOT_FOUND,
                        Json(ErrorResponse::not_found("tasks", id))
                        )
                    )?;

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
                    .map_err(|e|
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse::internal("Erro ao consultar o banco de dados", Some(e.to_string())))
                        )
                    )?;

       Ok(Json(ApiResponse::success(task.into())))
}

#[utoipa::path(
    delete,
    path = "/tasks/{id}",
    tag = "Tasks",
    params(
        ("id" = i32, Path, description = "Task ID")
    ),
    responses(
        (status = 204, description = "Task deleted successfully"),
        (status = 404, description = "Task not found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
pub async fn remove(State(state): State<AppState>, Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let task = task::Entity::find_by_id(id)
                .one(&state.db)
                .await
                .map_err(|e|
                    (StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::internal("Erro ao consultar o banco de dados", Some(e.to_string())))
                    )
                )?
                .ok_or(
                    (
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse::not_found("tasks", id))
                    )
                )?;

    let task: task::ActiveModel = task.into();

    task.delete(&state.db)
        .await
        .map_err(|e|
            (StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::internal("Erro ao consultar o banco de dados", Some(e.to_string())))
            )
            )?;
    Ok(StatusCode::NO_CONTENT)
}


pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list))
        .routes(routes!(create))
        .routes(routes!(get_one))
        .routes(routes!(update))
        .routes(routes!(remove))
}
