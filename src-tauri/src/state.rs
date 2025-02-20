use sea_orm::{Database, DatabaseConnection, DbErr};
use std::{env, fs, sync::Arc};
use tauri::{App, Manager};

use crate::migration::Migrator;
use sea_orm_migration::MigratorTrait;

#[derive(Default)]
pub struct AppState {
    db: Arc<DatabaseConnection>,
}

impl AppState {
    pub async fn new(app: &App) -> Result<Self, DbErr> {
        // get the app's data dir
        let app_dir = app.path().app_data_dir().expect("failed to get app dir");
        // ensure the app dir exists
        fs::create_dir_all(&app_dir).expect("failed to create app dir");

        let db_path = app_dir.join("photos.db");
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

        env::set_var("DATABASE_URL", db_url.clone());

        println!("-----------------------------------------------");
        println!("Initializing database at: {:?}", db_path);

        let connection = Database::connect(db_url).await?;

        println!("Running database migrations...");
        Migrator::up(&connection, None).await?;
        println!("Migrations completed sucessfully");
        println!("-----------------------------------------------");

        Ok(Self {
            db: Arc::new(connection),
        })
    }
}
