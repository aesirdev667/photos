use entities::nodes::{ActiveModel, NodeType};
use entities::prelude::*;
use stores::nodes::NodeStore;

mod test_helpers;

#[tokio::test]
async fn test_with_children() {
    let connection = test_helpers::setup_test_db()
        .await
        .expect("Db connection failed");
    let store = NodeStore::new(&connection);

    let mut node = ActiveModel {
        id: NotSet,
        path: Set("/".to_string()),
        size: Set(12i32),
        node_type: Set(NodeType::Directory),
        parent_id: Set(None),
        created_at: Set(chrono::Utc::now()),
        updated_at: Set(chrono::Utc::now()),
    };
    let root = store
        .save(node.clone())
        .await
        .expect("Couldn't save root node");

    node.path = Set("/file.txt".to_string());
    node.size = Set(122i32);
    node.node_type = Set(NodeType::File);
    node.parent_id = Set(Some(root.id));

    let _child_file = store
        .save(node.clone())
        .await
        .expect("Couldn't save child file node");

    node.path = Set("/folder".to_string());
    node.size = Set(12i32);
    node.node_type = Set(NodeType::Directory);
    node.parent_id = Set(Some(root.id));

    let child_folder = store
        .save(node.clone())
        .await
        .expect("Couldn't save child folder node");

    node.path = Set("/folder_two".to_string());
    node.size = Set(12i32);
    node.node_type = Set(NodeType::Directory);
    node.parent_id = Set(Some(root.id));

    let _child_folder_two = store
        .save(node.clone())
        .await
        .expect("Couldn't save second child folder node");

    node.path = Set("/folder/file.txt".to_string());
    node.size = Set(122i32);
    node.node_type = Set(NodeType::File);
    node.parent_id = Set(Some(child_folder.id));

    let _child_folder_file = store
        .save(node)
        .await
        .expect("Couldn't save child folder file node");

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

#[tokio::test]
async fn test_update_node() {
    let connection = test_helpers::setup_test_db()
        .await
        .expect("Db connection failed");
    let store = NodeStore::new(&connection);

    let mut node = ActiveModel {
        id: NotSet,
        path: Set("/".to_string()),
        size: Set(12i32),
        node_type: Set(NodeType::Directory),
        parent_id: Set(None),
        created_at: Set(chrono::Utc::now()),
        updated_at: Set(chrono::Utc::now()),
    };

    let root = store.save(node.clone()).await.expect("Couldn't save node");

    node.id = Set(root.id);
    node.size = Set(155i32);
    node.updated_at = Set(chrono::Utc::now());

    let updated = store
        .save(node.clone())
        .await
        .expect("Couldn't update node");

    assert_eq!(updated.updated_at, node.updated_at.unwrap())
}
