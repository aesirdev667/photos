use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "nodes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub path: String,
    pub size: i32,
    pub node_type: NodeType,
    pub parent_id: Option<i32>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,

    #[sea_orm(ignore)]
    #[serde(skip_deserializing, default)]
    pub children: Vec<Model>,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Text", enum_name = "node_type")]
pub enum NodeType {
    #[sea_orm(string_value = "file")]
    File,
    #[sea_orm(string_value = "directory")]
    Directory,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Parent,

    #[sea_orm(
        has_many = "Entity",
        from = "Column::Id",
        to = "Column::ParentId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Children,
}

impl Related<Self> for Entity {
    fn to() -> RelationDef {
        Relation::Children.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
