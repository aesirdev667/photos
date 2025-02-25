use migrations::{DatabaseConnection, DbErr, Migrator, MigratorTrait};

pub async fn setup_test_db() -> Result<DatabaseConnection, DbErr> {
    let connection = Migrator::connection("sqlite::memory:".to_string()).await?;
    Migrator::up(&connection, None).await?;
    Ok(connection)
}
