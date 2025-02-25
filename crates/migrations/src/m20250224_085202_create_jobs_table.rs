use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{enumeration, pk_auto, string, string_null, timestamp};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    #[allow(elided_lifetimes_in_paths)]
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Jobs::Table)
                    .if_not_exists()
                    .col(pk_auto(Jobs::Id))
                    .col(string(Jobs::JobType))
                    .col(enumeration(
                        Jobs::Status,
                        Alias::new("status"),
                        JobStatus::iter(),
                    ))
                    .col(string(Jobs::Payload))
                    .col(string_null(Jobs::Error))
                    .col(timestamp(Jobs::CreatedAt))
                    .col(timestamp(Jobs::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-jobs-job-type")
                    .table(Jobs::Table)
                    .col(Jobs::JobType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-jobs-status")
                    .table(Jobs::Table)
                    .col(Jobs::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    #[allow(elided_lifetimes_in_paths)]
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-jobs-status").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx-jobs-job-type").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Jobs::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Jobs {
    Table,
    Id,
    JobType,
    Status,
    Payload,
    Error,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]
pub enum JobStatus {
    #[iden = "pending"]
    Pending,
    #[iden = "running"]
    Running,
    #[iden = "completed"]
    Completed,
    #[iden = "failed"]
    Failed,
}
