use sea_orm::{ConnectOptions, Database};
use std::time::Duration;

pub use sea_orm::DatabaseConnection;
pub use sea_orm_migration::prelude::*;

mod m20250220_191902_create_nodes_table;
mod m20250224_085202_create_jobs_table;
mod m20250228_051742_create_metadata_table;

pub struct Migrator;

impl Migrator {
    pub async fn connection(db_url: String) -> Result<DatabaseConnection, DbErr> {
        let mut opt = ConnectOptions::new(db_url);

        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false)
            .sqlx_logging_level(log::LevelFilter::Info);

        Database::connect(opt).await
    }
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250220_191902_create_nodes_table::Migration),
            Box::new(m20250224_085202_create_jobs_table::Migration),
            Box::new(m20250228_051742_create_metadata_table::Migration),
        ]
    }
}
