use rtdb::task_dao;
use rtdb::tasks::TaskStatus;
use sea_orm::DatabaseConnection;

pub async fn complete_tasks(db: &DatabaseConnection, tids: &Vec<i32>) {
    for id in tids {
        let task = task_dao::find_task_by_id(db, *id)
            .await
            .expect("failed to find task by id from db");
        print!("Task: {}, {:?} to ", task.name, task.status);
        let task = task_dao::update_status(db, task, TaskStatus::Completed)
            .await
            .expect("failed to update task's status");
        println!("{:?}", task.status);
    }
}
