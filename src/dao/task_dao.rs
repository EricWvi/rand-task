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

pub async fn find_task_by_id(db: &DbConn, task_id: i32) -> Result<Task, DbErr> {
    let task: Option<Task> = Tasks::find_by_id(task_id).one(db).await?;
    task.ok_or(DbErr::RecordNotFound("task_id does not exist".to_string()))
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

pub async fn update_task(db: &DbConn, old: &Task, new: &Task) -> Result<Task, DbErr> {
    if old == new {
        return Ok(new.clone());
    }

    let mut task: tasks::ActiveModel = old.clone().into();
    if old.name != new.name {
        task.name = Set(new.name.clone());
    }
    if old.file_link != new.file_link {
        task.file_link = Set(new.file_link.clone());
    }
    if old.project_id != new.project_id {
        task.project_id = Set(new.project_id);
    }
    if old.status != new.status {
        task.status = Set(new.status);
    }
    task.update(db).await
}

pub async fn update_status(db: &DbConn, task: Task, status: TaskStatus) -> Result<Task, DbErr> {
    let mut task: tasks::ActiveModel = task.into();
    task.status = Set(status);
    task.update(db).await
}

#[cfg(test)]
mod test {
    use crate::tasks::TaskStatus;
    use crate::Task;
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

    #[tokio::test]
    async fn test_update_task() {
        let project = super::update_status(
            db().await,
            Task {
                id: 0,
                name: "".to_string(),
                file_link: None,
                project_id: 0,
                status: TaskStatus::Unfinished,
            },
            TaskStatus::Completed,
        )
        .await
        .unwrap();
        dbg!(project);
    }
}
