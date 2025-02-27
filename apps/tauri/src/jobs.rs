use crate::error::Error;
use crate::processors::NodeProcessor;
use crate::state::App;

use entities::jobs::{JobStatus, Model as Job};
use entities::nodes::{Model as Node, NodeType};
use migrations::DbErr;
use stores::jobs::JobStore;
use stores::nodes::NodeStore;

use std::{collections::VecDeque, fs};
use tauri::{AppHandle, Manager, Runtime, State};

async fn index_path<R: Runtime>(app_handle: &AppHandle<R>, path: String) -> Result<(), Error> {
    let state: State<'_, App> = app_handle.state();
    let store = NodeStore::new(state.db());
    let processor = NodeProcessor::new(store);
    let mut queue = VecDeque::new();
    queue.push_back((path, None::<Node>));

    while let Some((current_path, parent_node)) = queue.pop_front() {
        if let Ok(node) = processor
            .process(current_path.clone(), parent_node.clone())
            .await
        {
            if node.node_type == NodeType::Directory {
                if let Ok(entries) = fs::read_dir(current_path.clone()) {
                    for entry in entries.flatten() {
                        // only index files directories and files that we support
                        queue.push_back((
                            entry.path().to_string_lossy().to_string(),
                            Some(node.clone()),
                        ));
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
crate::background_job! { index_path }
