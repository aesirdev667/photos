use tauri::{async_runtime, generate_context, generate_handler, Builder, Manager};

mod commands;
mod entity;
mod error;
mod migration;
mod state;
mod stores;

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

        .invoke_handler(generate_handler![
            commands::library::library_open,
        ])

        .run(generate_context!())
        .expect("error while running tauri application");
}
