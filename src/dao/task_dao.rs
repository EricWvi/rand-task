use crate::entity::*;
use crate::tasks::TaskStatus;
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::*, query::*, DbConn, DbErr};

pub async fn add_task(
    db: &DbConn,
    name: String,
    file_link: Option<String>,
    project_id: i32,
) -> Result<Task, DbErr> {
    tasks::ActiveModel {
        name: Set(name),
        file_link: Set(file_link),
        project_id: Set(project_id),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn find_tasks_by_project_id(db: &DbConn, project_id: i32) -> Result<Vec<Task>, DbErr> {
    let tasks: Vec<Task> = Tasks::find()
        .filter(tasks::Column::ProjectId.eq(project_id))
        .all(db)
        .await?;
    Ok(tasks)
}

pub async fn get_first_task(db: &DbConn, project_id: i32) -> Result<Option<Task>, DbErr> {
    let task: Option<Task> = Tasks::find()
        .filter(tasks::Column::ProjectId.eq(project_id))
        .filter(tasks::Column::Status.eq(TaskStatus::Unfinished))
        .one(db)
        .await?;
    Ok(task)
}

pub async fn update_status(
    db: &DbConn,
    task: Task,
    status: TaskStatus,
) -> Result<Task, DbErr> {
    let mut task: tasks::ActiveModel = task.into();
    task.status = Set(status);
    task.update(db).await
}

#[cfg(test)]
mod test {
    use sea_orm::DatabaseConnection;

    async fn db() -> &'static DatabaseConnection {
        crate::DB.get_or_init(crate::init).await
    }

    #[tokio::test]
    async fn test_add_task() {
        let task = super::add_task(db().await, "subtask-1".to_string(), None, 71)
            .await
            .unwrap();
        dbg!(task);
    }

    #[tokio::test]
    async fn test_add_project() {
        let project = super::find_tasks_by_project_id(db().await, 72)
            .await
            .unwrap();
        dbg!(project);
    }
}
