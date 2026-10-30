pub use sea_orm_migration::prelude::*;

mod m20260928_133512_tasks;
mod m20261021_121109_auth_user;
mod m20261026_204925_posts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260928_133512_tasks::Migration),
            Box::new(m20261021_121109_auth_user::Migration),
            Box::new(m20261026_204925_posts::Migration),
        ]
    }
}
