#[macro_export]
macro_rules! background_job {
    ($func:ident) => {
        paste::paste! {
            pub async fn [<$func _job>](store: JobStore, payload: String) -> Result<jobs::Model, DbErr> {
                let job = store.enqueue(stringify!($func), payload.clone()).await?;

                tokio::spawn(async move {
                    match $func(payload).await {
                        Ok(_) => store.update_status(job.id, JobStatus::Completed, None).await,
                        Err(e) => store.update_status(job.id, JobStatus::Failed, Some(e.to_string())).await,
                    }
                });

                Ok(job)
            }
        }
    };
}
