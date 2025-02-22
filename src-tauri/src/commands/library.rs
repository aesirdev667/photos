use std::{collections::VecDeque, fs, path::PathBuf};

use crate::entity::nodes::Model;
use crate::error::Error;
use crate::state::AppState;
use crate::stores::nodes::NodeStore;

#[tauri::command]
pub async fn library_open(path: String, state: tauri::State<'_, AppState>) -> Result<Model, Error> {
    println!("command `library::open` called");

    let store = NodeStore::new(state.db());
    let mut root_node = store.find_by_path(path.clone()).await?;

    if root_node.is_none() {
        let _ = index_path(&store, PathBuf::from(path.clone())).await;
        root_node = store.find_by_path(path.clone()).await?;

        if root_node.is_none() {
            eprintln!("Path `{}` not found.", path);
            return Err(Error::Io(std::io::Error::other("Path not found")));
        }
    }

    Ok(root_node.unwrap().with_children(state.db()).await?)
}

async fn index_path(store: &NodeStore, path: PathBuf) -> Result<(), Error> {
    let mut queue = VecDeque::new();
    queue.push_back((path, None));

    while let Some((current_path, parent_node)) = queue.pop_front() {
        if let Ok(node) = create_node(store, current_path.clone(), parent_node.clone()).await {
            if node.node_type.contains("directory") {
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
    parent: Option<Model>,
) -> Result<Model, Error> {
    let metadata = fs::metadata(path.clone())?;
    let node = store
        .create(
            path.to_string_lossy().to_string(),
            metadata.len().try_into()?,
            if metadata.is_file() {
                "file".to_string()
            } else {
                "directory".to_string()
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
