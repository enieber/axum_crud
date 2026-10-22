use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use chrono::Utc;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name="users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub reset_password_token: String,
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

        Ok(self)
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateUser {
    pub email: String,
    pub password: String, 
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
