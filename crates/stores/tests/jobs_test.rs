use entities::jobs::JobStatus;
use stores::jobs::JobStore;

mod test_helpers;

#[tokio::test]
async fn test_enqueue() {
    let connection = test_helpers::setup_test_db()
        .await
        .expect("Db connection failed");
    let store = JobStore::new(&connection);

    let job = store
        .enqueue("background_function", "{\"value\":120}".to_string())
        .await
        .expect("Can't create job");

    assert_eq!(job.status, JobStatus::Pending);
}

#[tokio::test]
async fn test_update_status() {
    let connection = test_helpers::setup_test_db()
        .await
        .expect("Db connection failed");
    let store = JobStore::new(&connection);

    let job = store
        .enqueue("background_function", "{\"value\":120}".to_string())
        .await
        .expect("Can't create job");

    let job = store
        .update_status(job.id, JobStatus::Running, None)
        .await
        .expect("Can't update job status for {job.id}");

    assert_eq!(job.status, JobStatus::Running);
}

#[tokio::test]
async fn test_find_by_id() {
    let connection = test_helpers::setup_test_db()
        .await
        .expect("Db connection failed");
    let store = JobStore::new(&connection);

    let job = store
        .enqueue("background_function", "{\"value\":120}".to_string())
        .await
        .expect("Can't create job");

    let job = store
        .find_by_id(job.id)
        .await
        .expect("Can't find job {job.id}")
        .unwrap();

    assert_eq!(job.job_type, "background_function");
}

#[tokio::test]
async fn test_update_status_not_found() {
    let connection = test_helpers::setup_test_db()
        .await
        .expect("Db connection failed");
    let store = JobStore::new(&connection);

    let job = store.update_status(99i32, JobStatus::Running, None).await;

    assert!(job.is_err());
}
