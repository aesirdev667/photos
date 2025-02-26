
use std::{collections::VecDeque, fs, path::PathBuf};

use crate::error::Error;
use crate::state::App;

use entities::nodes::{Model as Node, NodeType};
use entities::jobs::{Model as Job};
use stores::nodes::NodeStore;
use stores::jobs::JobStore;

#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn library_open(path: String, state: tauri::State<'_, App>) -> Result<Node | Job, Error> {
    println!("command `library_open` called");

    let store = NodeStore::new(state.db());
    let node = store.with_children(path.clone()).await?;

    if node.is_some() {
        return Ok(root_node.unwrap());
    }

    let job_store = JobStore::new(state.db());
    let job = index_path_job(job_store.clone(), path).await?;

    Ok(job)
}

async fn index_path(store: JobStore, path: String) -> Result<(), Error> {
    let store = NodeStore::new(store.db());
    let mut queue = VecDeque::new();
    queue.push_back((path, None));

    while let Some((current_path, parent_node)) = queue.pop_front() {
        if let Ok(node) = create_node(store, current_path.clone(), parent_node.clone()).await {
            if node.node_type == NodeType::Directory {
                if let Ok(entries) = fs::read_dir(current_path.clone()) {
                    for entry in entries.flatten() {
                        queue.push_back((entry.path(), Some(node.clone())));
                    }
                } else {
                    eprintln!("Can't read entries from path {:?}", current_path.clone());
                }
            }
        } else {
            eprintln!("Can't create node from path {:?}", current_path.clone());
        }
    }

    Ok(())
}
background_job! { index_path }

async fn create_node(
    store: &NodeStore,
    path: PathBuf,
    parent: Option<Node>,
) -> Result<Node, Error> {
    let metadata = fs::metadata(path.clone())?;
    let node = store
        .create(
            path.to_string_lossy().to_string(),
            metadata.len().try_into()?,
            if metadata.is_file() {
                NodeType::File
            } else {
                NodeType::Directory
            },
            if parent.is_some() {
                Some(parent.unwrap().id)
            } else {
                None
            },
        )
        .await?;

    Ok(node)
}
