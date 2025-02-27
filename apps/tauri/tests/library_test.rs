use photos_lib::commands::library_open;
use photos_lib::state::App;

use tauri::Manager;
use tauri::test::{mock_builder, mock_context, noop_assets};

// mod test_helpers;

#[tokio::test]
async fn test_open_library() {
    let app = mock_builder().build(mock_context(noop_assets())).unwrap();

    match App::new(&app).await {
        Ok(state) => {
            app.manage(state);
            let app_handle = app.handle();

            let _job = library_open(app_handle.clone(), "/".to_string(), app.state::<App>())
                .await
                .expect("Couldn't create index job");

            // Add your assertions here if needed.
        }
        Err(e) => {
            panic!("Failed to create App state: {:?}", e);
        }
    }
}
