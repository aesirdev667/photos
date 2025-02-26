use entities::nodes::{self, NodeType};
// use migrations::DbErr;
// use stores::background_job;
use stores::nodes::NodeStore;
// use tokio::time::{sleep, Duration};

mod test_helpers;

// async fn example_task(payload: String) -> Result<(), String> {
//     sleep(Duration::from_secs(1)).await;
//
//     if payload.contains("error") {
//         return Err(format!("An error happened: {payload}"));
//     }
//
//     println!("Processing payload: {}", payload);
//     Ok(())
// }
// background_job! { example_task }

#[tokio::test]
async fn test_with_children() {
    let connection = test_helpers::setup_test_db()
        .await
        .expect("Db connection failed");
    let store = NodeStore::new(&connection);

    let root = store
        .create("/".to_string(), 12i32, NodeType::Directory, None)
        .await
        .expect("Couldn't create root node");

    let _child_file = store
        .create(
            "/file.txt".to_string(),
            122i32,
            NodeType::File,
            Some(root.id),
        )
        .await
        .expect("Couldn't create child file node");

    let child_folder = store
        .create(
            "/folder".to_string(),
            12i32,
            NodeType::Directory,
            Some(root.id),
        )
        .await
        .expect("Couldn't create child folder node");

    let _child_folder_two = store
        .create(
            "/folder_two".to_string(),
            12i32,
            NodeType::Directory,
            Some(root.id),
        )
        .await
        .expect("Couldn't create second child folder node");

    let _child_folder_file = store
        .create(
            "/folder/file.txt".to_string(),
            122i32,
            NodeType::File,
            Some(child_folder.id),
        )
        .await
        .expect("Couldn't create child folder file node");

    let tree = store
        .with_children("/".to_string())
        .await
        .expect("Couldn't load tree from store")
        .unwrap();

    assert_eq!(tree.children.len(), 3);
    assert_eq!(
        tree.children
            .iter()
            .find(|n| n.path == "/folder")
            .unwrap()
            .children
            .len(),
        1
    );
}
