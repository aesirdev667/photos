use entities::nodes::{ActiveModel, Entity, Model as Node, NodeType};
use migrations::{DatabaseConnection, DbErr};

use sea_orm::prelude::*;
use sea_orm::Statement;

use std::collections::HashMap;

pub struct NodeStore {
    db: DatabaseConnection,
}

impl NodeStore {
    #[must_use]
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    pub async fn with_children(&self, path: String) -> Result<Option<Node>, DbErr> {
        let all_nodes: Vec<Node> = Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r"
                WITH RECURSIVE NodeTree AS (
                    SELECT id, path, size, node_type, parent_id, created, modified
                    FROM nodes
                    WHERE path = $1

                    UNION ALL

                    SELECT n.id, n.path, n.size, n.node_type, n.parent_id, n.created, n.modified
                    FROM nodes n
                    JOIN NodeTree nt ON n.parent_id = nt.id
                )
                SELECT *
                FROM NodeTree;
                ",
                vec![path.clone().into()],
            ))
            .all(&self.db)
            .await?;

        // Create initial map of all nodes
        let mut node_map: HashMap<i32, Node> =
            all_nodes.into_iter().map(|node| (node.id, node)).collect();

        // First, move all files to their parent directories
        let file_ids: Vec<i32> = node_map
            .values()
            .filter(|node| node.node_type == NodeType::File)
            .map(|node| node.id)
            .collect();

        for file_id in file_ids {
            if let Some(file_node) = node_map.remove(&file_id) {
                if let Some(parent_id) = file_node.parent_id {
                    if let Some(parent) = node_map.get_mut(&parent_id) {
                        parent.children.push(file_node);
                    }
                }
            }
        }

        // Then move directories to their parents, starting from the deepest
        let mut dir_ids: Vec<i32> = node_map
            .values()
            .filter(|node| node.node_type == NodeType::Directory && node.path != path)
            .map(|node| node.id)
            .collect();

        // Sort directories by path length (descending) to process deepest first
        dir_ids.sort_by_key(|id| {
            node_map
                .get(id)
                .map_or(0, |node| node.path.matches('/').count())
        });
        dir_ids.reverse();

        for dir_id in dir_ids {
            if let Some(dir_node) = node_map.remove(&dir_id) {
                if let Some(parent_id) = dir_node.parent_id {
                    if let Some(parent) = node_map.get_mut(&parent_id) {
                        parent.children.push(dir_node);
                    }
                }
            }
        }

        Ok(node_map.values().find(|v| v.path == path).cloned())
    }

    pub async fn save(&self, node: ActiveModel) -> Result<Node, DbErr> {
        if node.id.is_set() {
            Ok(Entity::update(node).exec(&self.db).await?)
        } else {
            Ok(Entity::insert(node).exec_with_returning(&self.db).await?)
        }
    }
}
