use std::{path::PathBuf, sync::Arc};
use tauri;

use migrations::{DatabaseConnection, DbErr, Migrator, MigratorTrait};

#[derive(Default)]
pub struct App {
    db: Arc<DatabaseConnection>,
}

impl App {
    #[allow(clippy::future_not_send)]
    pub async fn new<R: tauri::Runtime>(app: &tauri::App<R>) -> Result<Self, DbErr> {
        let db_path = Self::get_db_path(app);
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

        println!("-----------------------------------------------");
        println!("Initializing database at: {db_path:?}");

        let connection = Migrator::connection(db_url).await?;

        println!("Running database migrations...");
        Migrator::up(&connection, None).await?;
        println!("Migrations completed sucessfully");
        println!("-----------------------------------------------");

        Ok(Self {
            db: Arc::new(connection),
        })
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    #[cfg(test)]
    fn get_db_path<R: tauri::Runtime>(_app: &tauri::App<R>) -> PathBuf {
        use tempfile::tempdir;

        static TEMP_DIR: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();
        let temp_dir = TEMP_DIR.get_or_init(|| tempdir().expect("Failed to create temp dir"));
        temp_dir.path().join("test.db")
    }

    #[cfg(not(test))]
    fn get_db_path<R: tauri::Runtime>(app: &tauri::App<R>) -> PathBuf {
        use std::fs;
        use tauri::Manager;

        let app_dir = app.path().app_data_dir().expect("failed to get app dir");
        fs::create_dir_all(&app_dir).expect("failed to create app dir");
        app_dir.join("photos.db")
    }
}
