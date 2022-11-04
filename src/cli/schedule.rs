use rtdb::task_dao;
use rtdb::tasks::TaskStatus;
use sea_orm::DatabaseConnection;

pub async fn schedule_task(db: &DatabaseConnection, ids: &Vec<i32>) {
    for id in ids {
        let task = task_dao::find_tasks_by_id(db, *id)
            .await
            .expect("failed to find task by id from db");
        print!("Task: {}, {:?} to ", task.name, task.status);
        let task = task_dao::update_status(db, task, TaskStatus::Scheduled)
            .await
            .expect("failed to update task's status'");
        println!("{:?}", task.status)
    }
}

pub async fn deschedule_task(db: &DatabaseConnection, ids: &Vec<i32>) {
    for id in ids {
        let task = task_dao::find_tasks_by_id(db, *id)
            .await
            .expect("failed to find tasks by id from db");
        print!("Task: {}, {:?} to ", task.name, task.status);
        let task = task_dao::update_status(db, task, TaskStatus::Pending)
            .await
            .expect("failed to update task's status");
        println!("{:?}", task.status)
    }
}
