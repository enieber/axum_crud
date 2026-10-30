use chrono::Utc;
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub body: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert && self.created_at.is_not_set() {
            self.created_at = Set(Utc::now().naive_utc());
        }

        self.updated_at = Set(Utc::now().naive_utc());

        Ok(self)
    }
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct CreatePost {
    #[validate(length(
        min = 3,
        max = 100,
        message = "O título deve ter entre 3 e 100 caracteres"
    ))]
    pub title: String,

    #[validate(length(min = 10, message = "A descrição deve ter pelo menos 10 caracteres"))]
    pub description: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "O slug deve ter entre 3 e 100 caracteres"
    ))]
    #[validate(regex(path = *SLUG_REGEX, message = "O slug deve conter apenas letras minúsculas, números e hífens"))]
    pub slug: String,

    #[validate(length(min = 20, message = "O conteúdo do post é muito curto"))]
    pub body: String,
}

impl From<CreatePost> for ActiveModel {
    fn from(dto: CreatePost) -> Self {
        Self {
            title: Set(dto.title),
            description: Set(dto.description),
            body: Set(dto.body),
            slug: Set(dto.slug),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdatePost {
    #[validate(length(min = 3, max = 100))]
    pub title: Option<String>,
    #[validate(regex(path = *SLUG_REGEX))]
    pub description: Option<String>,
    #[validate(length(min = 20, message = "O conteúdo do post é muito curto"))]
    pub body: Option<String>,
    #[validate(regex(path = *SLUG_REGEX, message = "O slug deve conter apenas letras minúsculas, números e hífens"))]
    pub slug: Option<String>,
}

// Implementação manual para atualização parcial (Patch)
impl From<UpdatePost> for ActiveModel {
    fn from(dto: UpdatePost) -> Self {
        Self {
            // Se for Some(valor), vira Set(valor). Se for None, vira NotSet (ignora no UPDATE)
            title: dto.title.map(Set).unwrap_or(NotSet),
            description: dto.description.map(Set).unwrap_or(NotSet),
            slug: dto.slug.map(Set).unwrap_or(NotSet),
            ..Default::default()
        }
    }
}
