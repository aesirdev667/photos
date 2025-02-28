use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250220_191902_create_nodes_table::Nodes;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Metadata::Table)
                    .if_not_exists()
                    .col(pk_auto(Metadata::Id))
                    .col(integer(Metadata::NodeId))
                    .col(string_null(Metadata::CameraMake))
                    .col(string_null(Metadata::CameraModel))
                    .col(float_null(Metadata::ExposureTime))
                    .col(float_null(Metadata::FNumber))
                    .col(integer_null(Metadata::IsoSpeed))
                    .col(float_null(Metadata::FocalLength))
                    .col(string_null(Metadata::LensModel))
                    .col(integer_null(Metadata::Width))
                    .col(integer_null(Metadata::Height))
                    .col(float_null(Metadata::GpsLatitude))
                    .col(float_null(Metadata::GpsLongitude))
                    .col(float_null(Metadata::GpsAltitude))
                    .col(timestamp(Metadata::Created))
                    .col(timestamp(Metadata::Modified))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-node")
                            .from(Metadata::Table, Metadata::NodeId)
                            .to(Nodes::Table, Nodes::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-node")
                    .table(Metadata::Table)
                    .col(Metadata::NodeId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-node").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Metadata::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Metadata {
    Table,
    Id,
    NodeId,
    CameraMake,
    CameraModel,
    ExposureTime,
    FNumber,
    IsoSpeed,
    FocalLength,
    LensModel,
    Width,
    Height,
    GpsLatitude,
    GpsLongitude,
    GpsAltitude,
    Created,
    Modified,
}
