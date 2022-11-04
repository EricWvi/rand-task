use rtdb::tasks::{TaskStatus, TaskType};
use rtdb::{task_dao, util};
use sea_orm::DatabaseConnection;
use std::fs;
use std::path::PathBuf;

pub async fn complete_task(db: &DatabaseConnection, ids: &Vec<i32>) {
    for id in ids {
        let task = task_dao::find_tasks_by_id(db, *id)
            .await
            .expect("failed to find task by id from db");
        print!("Task: {}, {:?} to ", task.name, task.status);
        let task = task_dao::update_status(db, task, TaskStatus::Completed)
            .await
            .expect("failed to update task's status'");
        println!("{:?}", task.status);

        if util::is_rt_md(task.md_link.as_ref()) {
            let dir = rtdb::config::task_dir();
            let file_name = task.md_link.as_ref().unwrap();
            let mut prev = PathBuf::from(dir);
            let task_type = match task.r#type {
                TaskType::FocusAnotherThing => "focus-another-thing",
                TaskType::TakeABreak => "take-a-break",
                TaskType::Tired => "tired",
                TaskType::Today => "current-work",
                TaskType::Inbox => "inbox",
                TaskType::En => "en",
            };
            prev.push(task_type);
            prev.push(file_name);
            let mut curr = PathBuf::from(dir);
            curr.push("completed");
            curr.push(file_name);
            if prev.exists() {
                fs::rename(prev, curr).expect("failed to move md file");
                println!("Moving {task_type}/{file_name} to completed/{file_name}");
            }
        }
    }
}
