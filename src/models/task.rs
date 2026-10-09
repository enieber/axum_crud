use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use chrono::Utc;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    #[sea_orm(column_type = "Boolean")]
    pub done: bool,
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


#[derive(Deserialize, ToSchema)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub done: Option<bool>,
}
