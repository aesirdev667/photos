use entities::nodes::Model as Node;
use stores::nodes::NodeStore;

use crate::error::Error;
use crate::state::App;
use tauri::State;

#[tauri::command]
pub async fn library_open(path: String, state: State<'_, App>) -> Result<Node, Error> {
    println!("command `library_open` called");

    let store = NodeStore::new(state.db());
    let root_node = store.with_children(path.clone()).await?;

    // if root_node.is_none() {
    //     /// add index job
    //     /// return index job
    //     let _ = index_path(&store, PathBuf::from(path.clone())).await;
    //     root_node = store.find_by_path(path.clone()).await?;
    //
    //     if root_node.is_none() {
    //         eprintln!("Path `{path}` not found.");
    //         return Err(Error::Io(std::io::Error::other("Path not found")));
    //     }
    // }

    Ok(root_node.unwrap())
}
