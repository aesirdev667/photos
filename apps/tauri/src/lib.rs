use tauri::{Builder, Manager, async_runtime, generate_context, generate_handler};

mod commands;
mod error;
mod state;

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(clippy::large_stack_frames)]
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
                let state = state::App::new(app)
                    .await
                    .expect("failed to initialize state");

                app.manage(state);
            });

            Ok(())
        })
        .invoke_handler(generate_handler![commands::library_open])
        .run(generate_context!())
        .expect("error while running tauri application");
}
