use crate::entity::*;
use crate::tasks::{TaskStatus, TaskType};
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::*, query::*, DbConn, DbErr};

pub async fn find_tasks_by_type(
    db: &DbConn,
    task_type: tasks::TaskType,
    with_pending: bool,
    with_completed: bool,
) -> Result<Vec<Task>, DbErr> {
    let select = Tasks::find().filter(tasks::Column::Type.eq(task_type));
    let select = if with_pending {
        select
    } else {
        select.filter(tasks::Column::Status.ne(TaskStatus::Pending))
    };
    let select = if with_completed {
        select
    } else {
        select.filter(tasks::Column::Status.ne(TaskStatus::Completed))
    };
    let tasks: Vec<Task> = select.all(db).await?;
    Ok(tasks)
}

pub async fn find_tasks_by_status(
    db: &DbConn,
    task_status: TaskStatus,
) -> Result<Vec<Task>, DbErr> {
    let tasks: Vec<Task> = Tasks::find()
        .filter(tasks::Column::Status.eq(task_status))
        .all(db)
        .await?;
    Ok(tasks)
}

pub async fn find_tasks_by_name(db: &DbConn, name: &str) -> Result<Vec<Task>, DbErr> {
    let tasks: Vec<Task> = Tasks::find()
        .filter(tasks::Column::Name.contains(name))
        .all(db)
        .await?;
    Ok(tasks)
}

pub async fn find_tasks_by_id(db: &DbConn, id: i32) -> Result<Task, DbErr> {
    let task: Option<Task> = Tasks::find_by_id(id).one(db).await?;
    task.ok_or(DbErr::RecordNotFound("task_id does not exist".to_string()))
}

pub async fn find_all_tasks(db: &DbConn) -> Result<Vec<Task>, DbErr> {
    let tasks: Vec<Task> = Tasks::find().all(db).await?;
    Ok(tasks)
}

pub async fn add_task(
    db: &DbConn,
    name: String,
    md_link: Option<String>,
    task_type: tasks::TaskType,
    weight: i32,
) -> Result<Task, DbErr> {
    tasks::ActiveModel {
        name: Set(name),
        md_link: Set(md_link),
        r#type: Set(task_type),
        weight: Set(weight),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn update_status(db: &DbConn, task: Task, status: TaskStatus) -> Result<Task, DbErr> {
    let mut task: tasks::ActiveModel = task.into();
    task.status = Set(status);
    task.update(db).await
}

pub async fn update_type(db: &DbConn, task: Task, task_type: TaskType) -> Result<Task, DbErr> {
    let mut task: tasks::ActiveModel = task.into();
    task.r#type = Set(task_type);
    task.update(db).await
}

pub async fn update_weight(db: &DbConn, task: &Task, weight: i32) -> Result<Task, DbErr> {
    let mut task: tasks::ActiveModel = task.clone().into();
    task.weight = Set(weight);
    task.update(db).await
}

pub async fn update_task(db: &DbConn, old: &Task, new: &Task) -> Result<Task, DbErr> {
    if old == new {
        return Ok(new.clone());
    }

    let mut task: tasks::ActiveModel = old.clone().into();
    if old.name != new.name {
        task.name = Set(new.name.clone());
    }
    if old.md_link != new.md_link {
        task.md_link = Set(new.md_link.clone());
    }
    if old.r#type != new.r#type {
        task.r#type = Set(new.r#type);
    }
    if old.weight != new.weight {
        task.weight = Set(new.weight);
    }
    if old.status != new.status {
        task.status = Set(new.status);
    }
    task.update(db).await
}

#[cfg(test)]
mod test {
    use crate::tasks::{TaskStatus, TaskType};
    use sea_orm::DatabaseConnection;

    async fn db() -> &'static DatabaseConnection {
        crate::DB.get_or_init(crate::init).await
    }

    #[tokio::test]
    async fn test_find_tasks_by_name() {
        let tasks = super::find_tasks_by_name(db().await, "人").await.unwrap();
        dbg!(tasks);
    }

    #[tokio::test]
    async fn test_add_task() {
        let task = super::add_task(
            db().await,
            "test".to_string(),
            None,
            TaskType::FocusAnotherThing,
            1,
        )
        .await
        .unwrap();
        dbg!(task);
    }
}
