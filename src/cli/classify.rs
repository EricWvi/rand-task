use rtdb::tasks::TaskType;
use rtdb::{task_dao, util};
use sea_orm::DatabaseConnection;
use std::fs;
use std::path::PathBuf;

pub async fn classify_tasks(db: &DatabaseConnection) {
    let tasks = task_dao::find_tasks_by_type(db, TaskType::Inbox, true, false)
        .await
        .expect("failed to find tasks by type from db");
    for t in tasks {
        let task_type = move_to(&*t.name);
        println!("Task:{}, moving to {:?}", t.name, task_type);
        let msg = format!("failed to move task[id={}]", t.id);
        let task = task_dao::update_type(db, t, task_type).await.expect(&*msg);

        if util::is_rt_md(task.md_link.as_ref()) {
            let dir = rtdb::config::task_dir();
            let mut prev = PathBuf::from(dir);
            prev.push("inbox");
            prev.push(task.md_link.as_ref().unwrap());
            let mut curr = PathBuf::from(dir);
            curr.push(match task_type {
                TaskType::FocusAnotherThing => "focus-another-thing",
                TaskType::TakeABreak => "take-a-break",
                TaskType::Tired => "tired",
                TaskType::Today => "current-work",
                TaskType::Inbox => "inbox",
                TaskType::En => "en",
            });
            curr.push(task.md_link.as_ref().unwrap());
            if prev.exists() {
                fs::rename(prev, curr).expect("failed to move md file");
            }
        }
    }
}

fn move_to(name: &str) -> TaskType {
    let title = format!("Move {} to:", name);
    util::choose_from_list(
        &*title,
        vec![
            TaskType::Inbox.into(),
            TaskType::Today.into(),
            TaskType::En.into(),
            TaskType::FocusAnotherThing.into(),
            TaskType::TakeABreak.into(),
            TaskType::Tired.into(),
        ],
        vec![0],
    )[0]
    .clone()
    .into()
}
