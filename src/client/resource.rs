use crate::client::http::ApiError;
use crate::state::AppState;
use serde::{Serialize, de::DeserializeOwned};

pub struct Resource {
    pub base: reqwest::Url,
    pub http: reqwest::Client,
}

impl Resource {
    pub fn new(state: &AppState, path: &str) -> Result<Self, ApiError> {
        // Mantemos a URL base limpa, sem forçar barras no final
        let base = state
            .api_base
            .join(path)
            .map_err(|e| ApiError::Url(e.to_string()))?;
        Ok(Self {
            base,
            http: state.http.clone(),
        })
    }

    /// Método auxiliar seguro para anexar um ID ao final da URL
    fn build_url_with_id(&self, id: impl std::fmt::Display) -> Result<reqwest::Url, ApiError> {
        let mut url = self.base.clone();
        // path_segments_mut garante que o ID seja anexado como um novo segmento de caminho
        // Ex: "http://.../tasks" vira "http://.../tasks/6"
        // Ex: "http://.../tasks/" vira "http://.../tasks/6"
        url.path_segments_mut()
            .map_err(|_| ApiError::Url("URL não suporta segmentos de caminho".to_string()))?
            .push(&id.to_string());
        Ok(url)
    }

    /// GET com query params (Usa a URL base limpa: /api/v1/tasks)
    pub async fn list<Q, R>(&self, query: &Q) -> Result<R, ApiError>
    where
        Q: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let resp = self
            .http
            .get(self.base.clone())
            .query(query)
            .send()
            .await
            .map_err(|e| ApiError::Unreachable(e.to_string()))?;
        Self::parse(resp).await
    }

    /// GET por ID (Usa o auxiliar seguro: /api/v1/tasks/6)
    pub async fn get<R: DeserializeOwned>(
        &self,
        id: impl std::fmt::Display,
    ) -> Result<R, ApiError> {
        let url = self.build_url_with_id(id)?;
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| ApiError::Unreachable(e.to_string()))?;
        Self::parse(resp).await
    }

    /// POST na coleção (Usa a URL base limpa: /api/v1/tasks)
    pub async fn create<B, R>(&self, body: &B) -> Result<R, ApiError>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let resp = self
            .http
            .post(self.base.clone())
            .json(body)
            .send()
            .await
            .map_err(|e| ApiError::Unreachable(e.to_string()))?;
        Self::parse(resp).await
    }

    /// PUT por ID (Usa o auxiliar seguro: /api/v1/tasks/6)
    pub async fn update<B, R>(&self, id: impl std::fmt::Display, body: &B) -> Result<R, ApiError>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = self.build_url_with_id(id)?;
        let resp = self
            .http
            .put(url)
            .json(body)
            .send()
            .await
            .map_err(|e| ApiError::Unreachable(e.to_string()))?;
        Self::parse(resp).await
    }

    /// DELETE por ID (Usa o auxiliar seguro: /api/v1/tasks/6)
    pub async fn delete(&self, id: impl std::fmt::Display) -> Result<(), ApiError> {
        let url = self.build_url_with_id(id)?;
        let resp = self
            .http
            .delete(url)
            .send()
            .await
            .map_err(|e| ApiError::Unreachable(e.to_string()))?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(ApiError::Status(resp.status()))
        }
    }

    async fn parse<R: DeserializeOwned>(resp: reqwest::Response) -> Result<R, ApiError> {
        if !resp.status().is_success() {
            return Err(ApiError::Status(resp.status()));
        }
        resp.json::<R>()
            .await
            .map_err(|e| ApiError::Decode(e.to_string()))
    }
}
