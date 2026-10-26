use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, Set};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::models::user;
use crate::models::user::{CreateUser, ResponseCreatedUser};
use crate::models::validation::{ErrorValidation, ValidationErrors};
use crate::services::auth::hash_password;
use crate::state::AppState;

#[utoipa::path(
    post,
    path = "/register",
    tag = "Auth",
    request_body = CreateUser,
    responses(
        (status = 201, description = "register user", body = ResponseCreatedUser),
        (status = 400, description = "VALIDATION_ERROR", body = ErrorValidation),
        (status = 500, description = "Internal Server Error", body = ErrorValidation)
    )
)]
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<ResponseCreatedUser>), (StatusCode, Json<ErrorValidation>)> {
    if let Err(errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorValidation {
                error: "VALIDATION_ERROR".into(),
                message: "Erro de validação".into(),
                details: Some(errors.to_details()),
            }),
        ));
    }

    let password = hash_password(&payload.password)
        .map_err(|e| {
            tracing::error!("Falha ao hashear senha: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorValidation {
                    error: "INTERNAL_ERROR".into(),
                    message: "Erro ao processar senha".into(),
                    details: None,
                }),
            )
        })
        .unwrap();

    let user = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        password: Set(password),
        ..Default::default()
    };

    match user.insert(&state.db).await {
        Ok(u) => Ok((StatusCode::CREATED, Json(ResponseCreatedUser::from_user(u)))),
        Err(sea_orm::DbErr::Custom(msg)) if msg.starts_with("VALIDATION:") => {
            let mut parts = msg.splitn(3, ':');
            let _ = parts.next();
            let field = parts.next().unwrap_or("").to_string();
            let message = parts.next().unwrap_or("").to_string();

            let mut errors = ValidationErrors::new();
            errors.push(&field, message);

            Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorValidation {
                    error: "VALIDATION_ERROR".into(),
                    message: "Erro de validação".into(),
                    details: Some(errors.to_details()),
                }),
            ))
        }
        Err(sea_orm::DbErr::Query(sea_orm::RuntimeErr::SqlxError(sqlx_err))) => {
            if let Some(db_err) = sqlx_err.as_database_error() {
                if db_err.is_unique_violation() {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ErrorValidation {
                            error: "INTERNAL_ERROR".into(),
                            message: "Erro de validação".into(),
                            details: None,
                        }),
                    ));
                }
            }
            tracing::error!("Erro de banco ao inserir user: {:?}", sqlx_err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorValidation {
                    error: "INTERNAL_ERROR".into(),
                    message: "Erro ao inserir usuário".into(),
                    details: None,
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Erro ao inserir user: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorValidation {
                    error: "INTERNAL_ERROR".into(),
                    message: "Erro ao inserir usuário".into(),
                    details: None,
                }),
            ))
        }
    }
}

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(create))
}
