#[macro_export]
macro_rules! generate_web_crud_handlers {
    ($config:ty) => {
        type Model = <$config as WebConfig>::Model;
        type Client = <$config as WebConfig>::Client;
        type CreateDto = <$config as WebConfig>::CreateDto;
        type UpdateDto = <$config as WebConfig>::UpdateDto;
        type Id = <$config as WebConfig>::Id;

        // Gera a conversão automática para a lista
        impl From<Model> for ListItemView {
            fn from(model: Model) -> Self {
                <$config as WebConfig>::build_list_item(&model)
            }
        }

        fn redirect_to_base() -> Response {
            axum::response::Redirect::to(&format!("/{}", <$config as WebConfig>::ROUTE_BASE))
                .into_response()
        }

        pub async fn list_view(
            State(state): State<AppState>,
            Query(params): Query<PaginationParams>,
        ) -> ListTemplate {
            let page = params.page;
            let limit = params.limit_clamped();
            let route_base = <$config as WebConfig>::ROUTE_BASE;
            let title_plural = <$config as WebConfig>::TITLE_PLURAL;

            let client = match Client::new(&state) {
                Ok(c) => c,
                Err(e) => return ListTemplate::empty(page, limit, e),
            };

            match client.list(page, limit).await {
                Ok(api) => {
                    let list: Vec<ListItemView> = api.data.into_iter().map(Into::into).collect();
                    ListTemplate::new(
                        title_plural,
                        list,
                        route_base,
                        page,
                        limit,
                        api.meta.map(|m| m.total).unwrap_or(0),
                    )
                }
                Err(e) => ListTemplate::empty(page, limit, e),
            }
        }

        pub async fn create_view() -> FormTemplate {
            FormTemplate {
                title: format!("Novo {}", <$config as WebConfig>::TITLE_SINGULAR).into(),
                action: format!("/{}", <$config as WebConfig>::ROUTE_BASE).into(),
                base_action: format!("/{}", <$config as WebConfig>::ROUTE_BASE).into(),
                submit_label: "Criar".into(),
                fields: <$config as WebConfig>::build_create_fields(),
                general_error: None,
            }
        }

        pub async fn register_form(
            State(state): State<AppState>,
            Form(input): Form<CreateDto>,
        ) -> Response {
            if let Some(err) = <$config as WebConfig>::validate_create(&input) {
                let form = FormTemplate {
                    title: format!("Novo {}", <$config as WebConfig>::TITLE_SINGULAR).into(),
                    action: format!("/{}", <$config as WebConfig>::ROUTE_BASE).into(),
                    base_action: format!("/{}", <$config as WebConfig>::ROUTE_BASE).into(),
                    submit_label: "Criar".into(),
                    fields: <$config as WebConfig>::build_create_fields(), // Idealmente preenchido com os dados do input
                    general_error: Some(err),
                };
                return form_error(form, "");
            }

            let client = match Client::new(&state) {
                Ok(c) => c,
                Err(e) => return form_error(FormTemplate::empty(None), e),
            };

            match client.create(&input).await {
                Ok(_) => redirect_to_base(),
                Err(e) => form_error(FormTemplate::empty(None), e),
            }
        }

        pub async fn update_view(State(state): State<AppState>, Path(id): Path<Id>) -> Response {
            let client = match Client::new(&state) {
                Ok(c) => c,
                Err(err) => return FormTemplate::empty(Some(err.to_string())).into_response(),
            };

            let model = match client.find(id).await {
                Ok(api) => api.data,
                Err(err) => return FormTemplate::empty(Some(err.to_string())).into_response(),
            };

            FormTemplate {
                title: format!("Editar {}", <$config as WebConfig>::TITLE_SINGULAR).into(),
                action: format!("/{}/{}", <$config as WebConfig>::ROUTE_BASE, id),
                base_action: format!("/{}", <$config as WebConfig>::ROUTE_BASE).into(),
                submit_label: "Salvar".into(),
                fields: <$config as WebConfig>::build_update_fields(&model),
                general_error: None,
            }
            .into_response()
        }

        pub async fn update_form(
            State(state): State<AppState>,
            Path(id): Path<Id>,
            Form(input): Form<UpdateDto>,
        ) -> Response {
            if let Some(err) = <$config as WebConfig>::validate_update(&input) {
                let form = FormTemplate {
                    title: format!("Editar {}", <$config as WebConfig>::TITLE_SINGULAR).into(),
                    action: format!("/{}/{}", <$config as WebConfig>::ROUTE_BASE, id),
                    base_action: format!("/{}", <$config as WebConfig>::ROUTE_BASE).into(),
                    submit_label: "Salvar".into(),
                    fields: vec![], // Simplificado para o exemplo
                    general_error: Some(err),
                };
                return form_error(form, "");
            }

            let client = match Client::new(&state) {
                Ok(c) => c,
                Err(e) => return form_error(FormTemplate::empty(None), e),
            };

            match client.update(id, &input).await {
                Ok(_) => redirect_to_base(),
                Err(e) => form_error(FormTemplate::empty(None), e),
            }
        }

        pub async fn delete_form(State(state): State<AppState>, Path(id): Path<Id>) -> Response {
            let client = match Client::new(&state) {
                Ok(c) => c,
                Err(e) => return form_error(FormTemplate::empty(None), e),
            };

            match client.delete(id).await {
                Ok(_) => redirect_to_base(),
                Err(e) => form_error(FormTemplate::empty(None), e),
            }
        }
    };
}
