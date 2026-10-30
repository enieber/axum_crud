use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261026_204925_posts"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("posts")
                    .if_not_exists()
                    .col(pk_auto("id"))
                    .col(string("title").not_null())
                    .col(string("slug").not_null())
                    .col(string("body").not_null())
                    .col(string("description").not_null())
                    .col(
                        timestamp_with_time_zone("created_at")
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone("updated_at")
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_slug")
                    .table("posts")
                    .col("slug")
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_posts_slug")
                    .table("posts")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table("posts").to_owned())
            .await
    }
}
