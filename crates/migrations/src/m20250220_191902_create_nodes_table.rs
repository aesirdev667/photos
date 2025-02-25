use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{enumeration, integer, integer_null, pk_auto, string, timestamp};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    #[allow(elided_lifetimes_in_paths)]
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Nodes::Table)
                    .if_not_exists()
                    .col(pk_auto(Nodes::Id))
                    .col(string(Nodes::Path))
                    .col(integer(Nodes::Size))
                    .col(enumeration(
                        Nodes::NodeType,
                        Alias::new("node_type"),
                        NodeType::iter(),
                    ))
                    .col(integer_null(Nodes::ParentId))
                    .col(timestamp(Nodes::CreatedAt))
                    .col(timestamp(Nodes::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-node-parent")
                            .from(Nodes::Table, Nodes::ParentId)
                            .to(Nodes::Table, Nodes::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-nodes-path")
                    .table(Nodes::Table)
                    .col(Nodes::Path)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-nodes-parent")
                    .table(Nodes::Table)
                    .col(Nodes::ParentId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    #[allow(elided_lifetimes_in_paths)]
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-nodes-path").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx-nodes-parent").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Nodes::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Nodes {
    Table,
    Id,
    Path,
    Size,
    NodeType,
    ParentId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]
enum NodeType {
    #[iden = "file"]
    File,
    #[iden = "directory"]
    Directory,
}
