use crate::error::Error;

use entities::nodes::{ActiveModel, Model as Node, NodeType};
use entities::prelude::*;
use stores::nodes::NodeStore;

use std::fs;

pub struct NodeProcessor {
    pub store: NodeStore,
}

impl NodeProcessor {
    #[must_use]
    pub fn new(store: NodeStore) -> Self {
        Self { store }
    }

    pub async fn process(&self, path: String, parent: Option<Node>) -> Result<Node, Error> {
        let metadata = fs::metadata(&path)?;
        let is_dir = metadata.is_dir();

        let size: i32 = metadata.len().try_into()?;
        let created_at = metadata.created()?.into();
        let updated_at = metadata.modified()?.into();

        let node_type = if is_dir {
            NodeType::Directory
        } else {
            NodeType::File
        };
        let parent_id = if let Some(parent) = parent {
            Some(parent.id)
        } else {
            None
        };

        let node = ActiveModel {
            id: NotSet,
            path: Set(path),
            size: Set(size),
            node_type: Set(node_type),
            parent_id: Set(parent_id),
            created_at: Set(created_at),
            updated_at: Set(updated_at),
        };

        Ok(self.store.save(node).await?)
    }
}
