use std::{fs, path::Path};

use serde::Serialize;

use crate::entity::nodes::Model;
use crate::error::Error;
use crate::state::AppState;
use crate::stores::nodes::NodeStore;

#[tauri::command]
pub async fn library_open(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<Model, Error> {
    println!("command `library::open` called");

    let store = NodeStore::new(state.db());
    let root = store.find_by_path(path.clone()).await?;

    match root {
        Some(r) => Ok(r.with_children(state.db()).await?),
        None => Ok(index_path_rec(&store, Path::new(&path), None).await?),
    }
}

async fn index_path_rec(
    store: &NodeStore,
    path: &Path,
    parent: Option<Model>,
) -> Result<Model, Error> {
    let node = create_node(store, path, parent.clone()).await?;
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let pnode = Some(node.clone());

        if path.is_file() {
            Box::pin(create_node(store, &path, pnode)).await?;
        } else if path.is_dir() {
            Box::pin(index_path_rec(store, &path, pnode)).await?;
        }
    }

    Ok(node)
}

async fn create_node(
    store: &NodeStore,
    path: &Path,
    parent: Option<Model>,
) -> Result<Model, Error> {
    let metadata = fs::metadata(path)?;
    let node = store.create(
        path.to_string_lossy().to_string(),
        metadata.len().try_into()?,
        if metadata.is_file() { "file".to_string() } else { "directory".to_string() },
        if parent.is_some() { Some(parent.unwrap().id) } else { None },
    ).await?;

    Ok(node)
}
