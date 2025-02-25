use entities::nodes;
use entities::prelude::*;
use migrations::{DatabaseConnection, DbErr};

use sea_orm::{
    ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Statement,
};
use std::collections::HashMap;

pub struct NodeStore {
    db: DatabaseConnection,
}

impl NodeStore {
    #[must_use]
    pub fn new(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }

    /// # Errors
    /// Errors on database issues.
    pub async fn find_by_path(&self, path: String) -> Result<Option<nodes::Model>, DbErr> {
        Node::find()
            .filter(nodes::Column::Path.eq(path))
            .one(&self.db)
            .await
    }

    /// # Errors
    /// Errors on database issues.
    pub async fn with_children(&self, path: String) -> Result<Option<nodes::Model>, DbErr> {
        let all_nodes: Vec<nodes::Model> = Node::find()
            .from_raw_sql(Statement::from_sql_and_values(
                self.db.get_database_backend(),
                r"
                WITH RECURSIVE NodeTree AS (
                    SELECT id, path, size, node_type, parent_id, created_at, updated_at
                    FROM nodes
                    WHERE path = $1

                    UNION ALL

                    SELECT n.id, n.path, n.size, n.node_type, n.parent_id, n.created_at, n.updated_at
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
        let mut node_map: HashMap<i32, nodes::Model> =
            all_nodes.into_iter().map(|node| (node.id, node)).collect();

        // First, move all files to their parent directories
        let file_ids: Vec<i32> = node_map
            .values()
            .filter(|node| node.node_type == nodes::NodeType::File)
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
            .filter(|node| node.node_type == nodes::NodeType::Directory && node.path != path)
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

    /// # Errors
    /// Errors on database issues.
    pub async fn create(
        &self,
        path: String,
        size: i32,
        node_type: nodes::NodeType,
        parent_id: Option<i32>,
    ) -> Result<nodes::Model, DbErr> {
        let new_node = nodes::ActiveModel {
            path: Set(path.clone()),
            size: Set(size),
            node_type: Set(node_type.clone()),
            parent_id: Set(parent_id),
            updated_at: Set(chrono::Utc::now()),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        Node::insert(new_node).exec_with_returning(&self.db).await
    }
}
