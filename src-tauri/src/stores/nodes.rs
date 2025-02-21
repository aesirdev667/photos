use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use crate::entity::prelude::*;
use crate::entity::nodes;

pub struct NodeStore {
    db: DatabaseConnection,
}

impl NodeStore {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            db: db.clone(),
        }
    }

    pub async fn find_by_path(&self, path: String) -> Result<Option<nodes::Model>, DbErr> {
        Nodes::find()
            .filter(nodes::Column::Path.eq(path))
            .one(&self.db)
            .await
    }

    pub async fn find_children(&self, parent_id: i32) -> Result<Vec<nodes::Model>, DbErr> {
        Nodes::find()
            .filter(nodes::Column::ParentId.eq(parent_id))
            .all(&self.db)
            .await
    }

    pub async fn create(
        &self,
        path: String,
        size: i32,
        node_type: String,
        parent_id: Option<i32>,
    ) -> Result<nodes::Model, DbErr> {
        let new_node = nodes::ActiveModel {
            path: Set(path.to_owned()),
            size: Set(size.to_owned()),
            node_type: Set(node_type.to_owned()),
            parent_id: Set(parent_id.to_owned()),
            updated_at: Set(chrono::Utc::now()),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        Nodes::insert(new_node)
            .exec_with_returning(&self.db)
            .await
    }
}
