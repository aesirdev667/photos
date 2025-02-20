use tauri::{async_runtime, generate_context, Builder, Manager};

mod entity;
mod migration;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            async_runtime::block_on(async {
                let state = state::AppState::new(app)
                    .await
                    .expect("failed to initialize state");

                app.manage(state);
            });

            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
