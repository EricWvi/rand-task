use crate::entity::*;
use crate::projects::ProjectType;
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::*, DbConn, DbErr};

pub async fn add_record(
    db: &DbConn,
    name: String,
    datetime: NaiveDateTime,
    schedule_time: i32,
    actual_time: i32,
    use_rate: String,
    project_type: ProjectType,
    project_id: i32,
    task_id: i32,
) -> Result<Record, DbErr> {
    records::ActiveModel {
        name: Set(name),
        date: Set(datetime),
        schedule_time: Set(schedule_time),
        actual_time: Set(actual_time),
        use_rate: Set(use_rate),
        project_type: Set(project_type),
        project_id: Set(project_id),
        task_id: Set(task_id),
        ..Default::default()
    }
    .insert(db)
    .await
}
