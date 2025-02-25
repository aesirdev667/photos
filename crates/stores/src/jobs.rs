use entities::jobs;
use entities::prelude::*;

use migrations::{DatabaseConnection, DbErr};

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;

#[derive(Clone)]
pub struct JobStore {
    db: Arc<DatabaseConnection>,
}

impl JobStore {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            db: Arc::new(db.clone()),
        }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<jobs::Model>, DbErr> {
        Job::find()
            .filter(jobs::Column::Id.eq(id))
            .one(&*self.db)
            .await
    }

    pub async fn enqueue(&self, job_type: &str, payload: String) -> Result<jobs::Model, DbErr> {
        let job = jobs::ActiveModel {
            job_type: Set(job_type.to_string().to_owned()),
            status: Set(jobs::JobStatus::Pending.to_owned()),
            payload: Set(payload.to_owned()),
            updated_at: Set(Utc::now()),
            created_at: Set(Utc::now()),
            error: Set(None),
            ..Default::default()
        };

        Job::insert(job).exec_with_returning(&*self.db).await
    }

    pub async fn update_status(
        &self,
        job_id: i32,
        status: jobs::JobStatus,
        error: Option<String>,
    ) -> Result<jobs::Model, DbErr> {
        let job: Option<jobs::Model> = Job::find_by_id(job_id).one(&*self.db).await?;
        let mut job: jobs::ActiveModel = job.unwrap().into();

        job.status = Set(status.to_owned());
        job.updated_at = Set(Utc::now());
        job.error = Set(error.to_owned());

        Ok(job.update(&*self.db).await?)
    }
}
