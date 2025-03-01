use entities::jobs;
use entities::prelude::*;

use migrations::{DatabaseConnection, DbErr};

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;

#[derive(Clone)]
pub struct JobStore {
    pub db: Arc<DatabaseConnection>,
}

impl JobStore {
    #[must_use]
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            db: Arc::new(db.clone()),
        }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<jobs::Model>, DbErr> {
        Job::find()
            .filter(jobs::Column::Id.eq(id))
            .one(self.db())
            .await
    }

    pub async fn enqueue(&self, job_type: &str, payload: String) -> Result<jobs::Model, DbErr> {
        let job = jobs::ActiveModel {
            job_type: Set(job_type.to_string()),
            status: Set(jobs::JobStatus::Pending.clone()),
            payload: Set(payload.clone()),
            created: Set(Utc::now()),
            modified: Set(Utc::now()),
            error: Set(None),
            ..Default::default()
        };

        Job::insert(job).exec_with_returning(self.db()).await
    }

    pub async fn update_status(
        &self,
        job_id: i32,
        status: jobs::JobStatus,
        error: Option<String>,
    ) -> Result<jobs::Model, DbErr> {
        let job = Job::find_by_id(job_id).one(self.db()).await?;

        if job.is_none() {
            return Err(DbErr::Custom(format!("Can't find job with id '{job_id}'")));
        }

        let mut job: jobs::ActiveModel = job.unwrap().into();

        job.status = Set(status.clone());
        job.modified = Set(Utc::now());
        job.error = Set(error.clone());

        let job = job.update(self.db()).await?;

        Ok(job)
    }
}
