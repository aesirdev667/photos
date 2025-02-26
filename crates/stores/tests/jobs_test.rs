use entities::jobs::{self, JobStatus};
use entities::nodes::Model as Node;
use migrations::DbErr;
use stores::background_job;
use stores::jobs::JobStore;
use stores::nodes::NodeStore;
use tokio::time::{sleep, Duration};

mod test_helpers;

async fn example_task(store: JobStore, path: String) -> Result<Option<Node>, String> {
    sleep(Duration::from_secs(1)).await;

    if path.contains("error") {
        return Err(format!("An error happened: {path}"));
    }

    let store = NodeStore::new(&store.db);
    let node = store
        .find_by_path(path.clone())
        .await
        .expect("Can't query store");

    println!("Processing payload: {path}");
    Ok(node)
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

    sleep(Duration::from_secs(2)).await;

    let job_again = store
        .find_by_id(job.id)
        .await
        .expect("Job not found in database");
    assert_eq!(job_again.unwrap().status, JobStatus::Completed);
}

#[tokio::test]
async fn test_background_job_failed() {
    let connection = test_helpers::setup_test_db()
        .await
        .expect("Db connection failed");
    let store = JobStore::new(&connection);

    let job = example_task_job(store.clone(), "error payload".to_string())
        .await
        .unwrap();

    sleep(Duration::from_secs(2)).await;

    let job_again = store
        .find_by_id(job.id)
        .await
        .expect("Job not found in database");

    assert_eq!(job_again.unwrap().status, JobStatus::Failed);
}
