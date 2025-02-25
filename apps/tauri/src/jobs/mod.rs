// Queue manager
pub struct QueueManager {
    db: DatabaseConnection,
}

impl QueueManager {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: Arc::new(db.clone()) }
    }

    pub async fn enqueue(&self, job_type: &str, payload: String) -> Result<String, String> {
        let job = Job {
            id: Uuid::new_v4().to_string(),
            job_type: job_type.to_string(),
            status: JobStatus::Pending,
            payload,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            error: None,
        };

        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO jobs (id, job_type, status, payload, created_at, updated_at, error)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                job.id,
                job.job_type,
                serde_json::to_string(&job.status).unwrap(),
                job.payload,
                job.created_at.to_rfc3339(),
                job.updated_at.to_rfc3339(),
                job.error,
            ],
        ).map_err(|e| e.to_string())?;

        Ok(job.id)
    }

    pub async fn update_status(&self, job_id: &str, status: JobStatus, error: Option<String>) -> Result<(), String> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE jobs SET status = ?1, updated_at = ?2, error = ?3 WHERE id = ?4",
            params![
                serde_json::to_string(&status).unwrap(),
                Utc::now().to_rfc3339(),
                error,
                job_id,
            ],
        ).map_err(|e| e.to_string())?;

        Ok(())
    }
}

// Macro to create a background job from a function
#[macro_export]
macro_rules! background_job {
    ($func:ident) => {
        paste::paste! {
            pub async fn [<$func _job>](queue: Arc<QueueManager>, payload: String) -> Result<String, String> {
                let job_id = queue.enqueue(stringify!($func), payload.clone()).await?;

                tokio::spawn(async move {
                    match $func(payload).await {
                        Ok(_) => {
                            queue.update_status(&job_id, JobStatus::Completed, None).await.unwrap_or_else(|e| {
                                eprintln!("Failed to update job status: {}", e);
                            });
                        },
                        Err(e) => {
                            queue.update_status(&job_id, JobStatus::Failed, Some(e.to_string())).await.unwrap_or_else(|e| {
                                eprintln!("Failed to update job status: {}", e);
                            });
                        }
                    }
                });

                Ok(job_id)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    async fn example_task(payload: String) -> Result<(), String> {
        // Simulate some work
        sleep(Duration::from_secs(2)).await;
        println!("Processing payload: {}", payload);
        Ok(())
    }

    background_job!(example_task);

    #[tokio::test]
    async fn test_background_job() {
        let conn = Connection::open_in_memory().unwrap();
        let queue = Arc::new(QueueManager::new(conn));

        let job_id = example_task_job(queue.clone(), "test payload".to_string()).await.unwrap();

        // Wait for job to complete
        sleep(Duration::from_secs(3)).await;

        // Check job status
        let conn = queue.conn.lock().await;
        let status: String = conn.query_row(
            "SELECT status FROM jobs WHERE id = ?1",
            params![job_id],
            |row| row.get(0),
        ).unwrap();

        assert_eq!(status, serde_json::to_string(&JobStatus::Completed).unwrap());
    }
}
