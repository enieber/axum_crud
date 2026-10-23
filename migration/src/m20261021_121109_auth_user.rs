use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261021_121109_auth_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("users")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("name").not_null())
                    .col(string("email").not_null())
                    .col(string("password").not_null())
                    .col(string_null("reset_password_token"))
                    .col(timestamp_with_time_zone_null("reset_password_sent_at"))
                    .col(timestamp_with_time_zone_null("remember_created_at"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()).not_null())
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()).not_null())
                    .to_owned(),
            )
            .await?;

            manager
            .create_index(
                Index::create()
                    .name("idx_users_email_unique")
                    .table("users")
                    .col("email")
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       manager
            .drop_index(
                Index::drop()
                    .name("idx_users_email_unique")
                    .table("users")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table("users").to_owned())
            .await
    }
}
