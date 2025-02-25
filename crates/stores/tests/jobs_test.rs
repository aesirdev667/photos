use entities::jobs::{self, JobStatus};
use migrations::DbErr;
use stores::background_job;
use stores::jobs::JobStore;
use tokio::time::{sleep, Duration};

mod test_helpers;

async fn example_task(payload: String) -> Result<(), String> {
    sleep(Duration::from_secs(2)).await;
    println!("Processing payload: {}", payload);
    Ok(())
}
background_job! { example_task }

#[tokio::test]
async fn test_background_job() {
    let connection = test_helpers::setup_test_db()
        .await
        .expect("Db connection failed");
    let store = JobStore::new(&connection);

    let job = example_task_job(store.clone(), "test payload".to_string())
        .await
        .unwrap();

    sleep(Duration::from_secs(3)).await;

    let job_again = store
        .find_by_id(job.id)
        .await
        .expect("Job not found in database");
    assert_eq!(job_again.unwrap().status, JobStatus::Completed);
}
