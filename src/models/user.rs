use chrono::Utc;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::validation::ValidationErrors;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub reset_password_token: Option<String>,
    pub reset_password_send_at: Option<DateTime>,
    pub remember_created_at: Option<DateTime>,
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

        if let Set(name) = &self.name {
            if name.trim().is_empty() {
                return Err(DbErr::Custom("VALIDATION:name:não pode estar vazio".into()));
            }
        }

        if let Set(email) = &self.email {
            let e = email.trim();
            if e.is_empty() {
                return Err(DbErr::Custom(
                    "VALIDATION:email:não pode estar vazio".into(),
                ));
            }
            if !e.contains('@') {
                return Err(DbErr::Custom("VALIDATION:email:formato inválido".into()));
            }
        }
        Ok(self)
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl CreateUser {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        let name = self.name.trim();
        if name.is_empty() {
            errors.push("name", "não pode estar vazio");
        } else if name.chars().count() > 100 {
            errors.push("name", "máximo de 100 caracteres");
        }

        let email = self.email.trim();
        if email.is_empty() {
            errors.push("email", "não pode estar vazio");
        } else if !email.contains('@') {
            errors.push("email", "formato inválido");
        }

        if self.password.chars().count() < 8 {
            errors.push("password", "mínimo de 8 caracteres");
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ResponseCreatedUser {
    pub name: String,
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl ResponseCreatedUser {
    pub fn from_user(user: Model) -> Self {
        Self {
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ResetPassword {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct ResetPasswordResponse {
    pub reset_password_token: String,
}
