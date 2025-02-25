use std::{path::PathBuf, sync::Arc};

use migrations::{DatabaseConnection, DbErr, Migrator, MigratorTrait};

#[derive(Default)]
pub struct App {
    db: Arc<DatabaseConnection>,
}

impl App {
    #[allow(clippy::future_not_send)]
    pub async fn new(app: &tauri::App) -> Result<Self, DbErr> {
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
    fn get_db_path(_app: &tauri::App) -> PathBuf {
        use tempfile::tempdir;

        static TEMP_DIR: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();
        let temp_dir = TEMP_DIR.get_or_init(|| tempdir().expect("Failed to create temp dir"));
        temp_dir.path().join("test.db")
    }

    #[cfg(not(test))]
    fn get_db_path(app: &tauri::App) -> PathBuf {
        use std::fs;
        use tauri::Manager;

        let app_dir = app.path().app_data_dir().expect("failed to get app dir");
        fs::create_dir_all(&app_dir).expect("failed to create app dir");
        app_dir.join("photos.db")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tauri::test::{mock_builder, mock_context, noop_assets};
//
//     #[tokio::test]
//     async fn test_database_init() {
//         let app = mock_builder().build(mock_context(noop_assets())).unwrap();
//         let state = AppState::new(&app).await;
//         assert!(state.is_ok(), "Should initialize AppState");
//
//         let state = state.unwrap();
//         let result = state.db.execute(sea_orm::Statement::from_string(
//             sea_orm::DatabaseBackend::Sqlite,
//             "SELECT 1".to_string()
//         )).await;
//
//         assert!(result.is_ok(), "Should be able to query database");
//     }
//
// }
