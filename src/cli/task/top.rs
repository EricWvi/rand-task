use rtdb::task_dao;
use sea_orm::DatabaseConnection;

pub async fn top_task(db: &DatabaseConnection, tid: i32) {
    let task = task_dao::find_task_by_id(db, tid)
        .await
        .expect("failed to find task by id from db");
    let tasks = match task_dao::find_tasks_by_project_id(db, task.project_id).await {
        Ok(t) => t,
        Err(e) => panic!("{e:?}"),
    };
    let lower_bound = if tasks.len() > 1 {
        tasks[0].seq - 1
    } else {
        task.seq
    };
    let task = task_dao::update_seq(db, task, lower_bound)
        .await
        .expect("failed to update task's status");
    print!("Task: {} moving to top", task.name);
}
