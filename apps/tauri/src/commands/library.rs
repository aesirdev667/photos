
use std::{collections::VecDeque, fs, path::PathBuf};

use crate::error::Error;
use crate::state::App;

use entities::nodes::{Model as Node, NodeType};
use stores::nodes::NodeStore;

/// Module name repitition allowed here because the UI calls it with `library_open`.
#[allow(clippy::module_name_repetitions)]
#[tauri::command]
pub async fn library_open(path: String, state: tauri::State<'_, App>) -> Result<Node, Error> {
    println!("command `library_open` called");

    let store = NodeStore::new(state.db());
    let mut root_node = store.with_children(path.clone()).await?;

    if root_node.is_none() {
        let _ = index_path(&store, PathBuf::from(path.clone())).await;
        root_node = store.find_by_path(path.clone()).await?;

        if root_node.is_none() {
            eprintln!("Path `{path}` not found.");
            return Err(Error::Io(std::io::Error::other("Path not found")));
        }
    }

    Ok(root_node.unwrap())
}

async fn index_path(store: &NodeStore, path: PathBuf) -> Result<(), Error> {
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
