#[macro_export]
macro_rules! generate_crud {
    (
        $module:ident,
        entity: $entity:path,
        model: $model:path,
        create_dto: $create_dto:path,
        update_dto: $update_dto:path,
        resource_name: $resource_name:literal,
        tag: $tag:literal,
        id_type: $id_type:ty,
        id_field: $id_field:ident
    ) => {
        use axum::{
            extract::{Path, Query, State},
            http::StatusCode,
            Json,
        };
        use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
        use utoipa_axum::{routes};
        use validator::Validate;

        #[utoipa::path(
            get,
            path = concat!("/", $resource_name),
            tag = $tag,
            params(
                ("page" = u64, Query, description = "Número da página (padrão: 1)"),
                ("limit" = u64, Query, description = "Itens por página (padrão: 10)")
            ),
            responses(
                (status = 200, description = "Lista paginada", body = $crate::models::response::ApiResponse<Vec<$model>>),
                (status = 500, description = "Erro interno", body = $crate::models::error::ErrorResponse)
            )
        )]
        pub async fn list(
            State(state): State<$crate::state::AppState>,
            Query(params): Query<$crate::models::response::PaginationParams>,
        ) -> Result<Json<$crate::models::response::ApiResponse<Vec<$model>>>, (StatusCode, Json<$crate::models::error::ErrorResponse>)> {
            let paginator = <$entity>::find().paginate(&state.db, params.limit);

            let items = paginator
                .fetch_page(params.page.saturating_sub(1))
                .await
                .map_err(|err| {
                    tracing::error!("Erro ao buscar {} no banco: {:?}", $resource_name, err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json($crate::models::error::ErrorResponse::internal(
                            "Falha ao buscar registros",
                            Some(err.to_string()),
                        )),
                    )
                })?;

            let total = paginator
                .num_items_and_pages()
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json($crate::models::error::ErrorResponse::internal(
                            "Falha ao contar registros",
                            Some(e.to_string()),
                        )),
                    )
                })?
                .number_of_items;

            Ok(Json($crate::models::response::ApiResponse::paginated(
                items, total, params.page, params.limit,
            )))
        }

        #[utoipa::path(
            post,
            path = concat!("/", $resource_name),
            tag = $tag,
            request_body = $create_dto,
            responses(
                (status = 201, description = "Criado com sucesso", body = $crate::models::response::ApiResponse<$model>),
                (status = 400, description = "Erro de validação", body = $crate::models::error::ErrorResponse),
                (status = 409, description = "Conflito (chave duplicada)", body = $crate::models::error::ErrorResponse),
                (status = 500, description = "Erro interno", body = $crate::models::error::ErrorResponse)
            )
        )]
        pub async fn create(
            State(state): State<$crate::state::AppState>,
            Json(payload): Json<$create_dto>,
        ) -> Result<(StatusCode, Json<$crate::models::response::ApiResponse<$model>>), (StatusCode, Json<$crate::models::error::ErrorResponse>)> {
            if let Err(e) = payload.validate() {
                let mut error_map = serde_json::Map::new();
                for (field, errors) in e.field_errors() {
                    let messages: Vec<String> = errors.iter().map(|err| {
                        err.message.as_ref().map_or("Erro de validação".to_string(), |m| m.to_string())
                    }).collect();
                    error_map.insert(
                        field.to_string(),
                        serde_json::Value::Array(messages.into_iter().map(serde_json::Value::String).collect())
                    );
                }

                return Err((
                    StatusCode::BAD_REQUEST,
                    Json($crate::models::error::ErrorResponse::validation_fields(
                        "Dados inválidos fornecidos",
                        serde_json::Value::Object(error_map),
                    )),
                ));
            }

            let active_model: <$entity as EntityTrait>::ActiveModel = payload.into();

            let result = active_model.insert(&state.db).await.map_err(|e| {
                let err_str = e.to_string();
                let is_unique_violation = err_str.contains("unique")
                    || err_str.contains("duplicate")
                    || err_str.contains("2067")
                    || err_str.contains("23505");

                if is_unique_violation {
                    (
                        StatusCode::CONFLICT,
                        Json($crate::models::error::ErrorResponse::conflict(
                            "Já existe um registro com este slug (o slug deve ser único).",
                        )),
                    )
                } else {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json($crate::models::error::ErrorResponse::internal("Erro ao criar registro", Some(err_str))),
                    )
                }
            })?;

            Ok((StatusCode::CREATED, Json($crate::models::response::ApiResponse::success(result.into()))))
        }

        #[utoipa::path(
            get,
            path = concat!("/", $resource_name, "/{id}"),
            tag = $tag,
            params(("id" = $id_type, Path, description = "ID do registro")),
            responses(
                (status = 200, description = "Registro encontrado", body = $crate::models::response::ApiResponse<$model>),
                (status = 404, description = "Não encontrado", body = $crate::models::error::ErrorResponse),
                (status = 500, description = "Erro interno", body = $crate::models::error::ErrorResponse)
            )
        )]
        pub async fn get_one(
            State(state): State<$crate::state::AppState>,
            Path(id): Path<$id_type>,
        ) -> Result<Json<$crate::models::response::ApiResponse<$model>>, (StatusCode, Json<$crate::models::error::ErrorResponse>)> {
            let item = <$entity>::find_by_id(id)
                .one(&state.db)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json($crate::models::error::ErrorResponse::internal("Erro ao consultar banco", Some(e.to_string()))),
                    )
                })?
                .ok_or((
                    StatusCode::NOT_FOUND,
                    Json($crate::models::error::ErrorResponse::not_found($resource_name, id)),
                ))?;

            Ok(Json($crate::models::response::ApiResponse::success(item.into())))
        }

        #[utoipa::path(
            put,
            path = concat!("/", $resource_name, "/{id}"),
            tag = $tag,
            params(("id" = $id_type, Path, description = "ID do registro")),
            request_body = $update_dto,
            responses(
                (status = 200, description = "Atualizado com sucesso", body = $crate::models::response::ApiResponse<$model>),
                (status = 400, description = "Erro de validação", body = $crate::models::error::ErrorResponse),
                (status = 404, description = "Não encontrado", body = $crate::models::error::ErrorResponse),
                (status = 500, description = "Erro interno", body = $crate::models::error::ErrorResponse)
            )
        )]
        pub async fn update(
            State(state): State<$crate::state::AppState>,
            Path(id): Path<$id_type>,
            Json(payload): Json<$update_dto>,
        ) -> Result<Json<$crate::models::response::ApiResponse<$model>>, (StatusCode, Json<$crate::models::error::ErrorResponse>)> {
            if let Err(e) = payload.validate() {
                let mut error_map = serde_json::Map::new();
                for (field, errors) in e.field_errors() {
                    let messages: Vec<String> = errors.iter().map(|err| {
                        err.message.as_ref().map_or("Erro de validação".to_string(), |m| m.to_string())
                    }).collect();
                    error_map.insert(
                        field.to_string(),
                        serde_json::Value::Array(messages.into_iter().map(serde_json::Value::String).collect())
                    );
                }

                return Err((
                    StatusCode::BAD_REQUEST,
                    Json($crate::models::error::ErrorResponse::validation_fields(
                        "Dados inválidos fornecidos",
                        serde_json::Value::Object(error_map),
                    )),
                ));
            }

            // 1. Verifica se existe para retornar 404 corretamente
            let _ = <$entity>::find_by_id(id)
                .one(&state.db)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json($crate::models::error::ErrorResponse::internal("Erro ao consultar banco", Some(e.to_string()))),
                    )
                })?
                .ok_or((
                    StatusCode::NOT_FOUND,
                    Json($crate::models::error::ErrorResponse::not_found($resource_name, id)),
                ))?;

            // 2. Converte o payload para ActiveModel
            let mut active_model: <$entity as EntityTrait>::ActiveModel = payload.into();

            // 3. Força o ID a ser o da URL (garante que o update seja no registro certo)
            active_model.$id_field = Set(id);

            // 4. Atualiza (o SeaORM atualizará apenas os campos que forem `Set`, ignorando `NotSet`)
            let updated = active_model.update(&state.db).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json($crate::models::error::ErrorResponse::internal("Erro ao atualizar registro", Some(e.to_string()))),
                )
            })?;

            Ok(Json($crate::models::response::ApiResponse::success(updated.into())))
        }

        #[utoipa::path(
            delete,
            path = concat!("/", $resource_name, "/{id}"),
            tag = $tag,
            params(("id" = $id_type, Path, description = "ID do registro")),
            responses(
                (status = 204, description = "Removido com sucesso"),
                (status = 404, description = "Não encontrado", body = $crate::models::error::ErrorResponse),
                (status = 500, description = "Erro interno", body = $crate::models::error::ErrorResponse)
            )
        )]
        pub async fn remove(
            State(state): State<$crate::state::AppState>,
            Path(id): Path<$id_type>,
        ) -> Result<StatusCode, (StatusCode, Json<$crate::models::error::ErrorResponse>)> {
            let item = <$entity>::find_by_id(id)
                .one(&state.db)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json($crate::models::error::ErrorResponse::internal("Erro ao consultar banco", Some(e.to_string()))),
                    )
                })?
                .ok_or((
                    StatusCode::NOT_FOUND,
                    Json($crate::models::error::ErrorResponse::not_found($resource_name, id)),
                ))?;

            let active_model: <$entity as EntityTrait>::ActiveModel = item.into();

            active_model.delete(&state.db).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json($crate::models::error::ErrorResponse::internal("Erro ao remover registro", Some(e.to_string()))),
                )
            })?;

            Ok(StatusCode::NO_CONTENT)
        }

        pub fn routes() -> ::utoipa_axum::router::OpenApiRouter<$crate::state::AppState> {
            ::utoipa_axum::router::OpenApiRouter::new()
                .routes(routes!(list))
                .routes(routes!(create))
                .routes(routes!(get_one))
                .routes(routes!(update))
                .routes(routes!(remove))
        }
    };
}
