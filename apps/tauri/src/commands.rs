use entities::jobs::Model as Job;
use entities::nodes::Model as Node;
use stores::nodes::NodeStore;

use crate::error::Error;
use crate::jobs::index_path_job;
use crate::state::App;

use serde::Serialize;
use tauri::{AppHandle, Runtime, State};

#[derive(Debug, Clone, Serialize)]
pub enum LibraryOpenResult {
    Node(Node),
    Job(Job),
}

#[tauri::command]
pub async fn library_open<R: Runtime>(
    app_handle: AppHandle<R>,
    path: String,
    state: State<'_, App>,
) -> Result<LibraryOpenResult, Error> {
    println!("command `library_open` called");

    let store = NodeStore::new(state.db());
    let node = store.with_children(path.clone()).await?;

    if let Some(node) = node {
        return Ok(LibraryOpenResult::Node(node));
    }

    let job = index_path_job(app_handle.clone(), path).await?;

    Ok(LibraryOpenResult::Job(job))
}
