#[macro_export]
macro_rules! background_job {
    ($func:ident) => {
        paste::paste! {
            pub async fn [<$func _job>]<R: tauri::Runtime>(app_handle: tauri::AppHandle<R>, payload: String) -> Result<Job, DbErr> {
                let state: tauri::State<'_, App> = app_handle.state();
                let store = JobStore::new(state.db());
                let job = store.enqueue(stringify!($func), payload.clone()).await?;

                tokio::spawn(async move {
                    match $func(&app_handle, payload).await {
                        Ok(_) => store.update_status(job.id, JobStatus::Completed, None).await,
                        Err(e) => store.update_status(job.id, JobStatus::Failed, Some(e.to_string())).await,
                    }
                });

                Ok(job)
            }
        }
    };
}
