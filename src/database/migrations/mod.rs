pub use sea_orm_migration::prelude::*;

mod m20260126_000001_create_urls_table;

struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260126_000001_create_urls_table::Migration)]
    }
}

pub async fn run_migrations(database_url: &str) -> Result<(), DbErr> {
    let db = sea_orm::Database::connect(database_url).await?;
    Migrator::up(&db, None).await?;
    Ok(())
}
