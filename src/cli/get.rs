use rtdb::tasks::TaskStatus;
use rtdb::{project_dao, task_dao};
use sea_orm::DatabaseConnection;

pub async fn get_project(db: &DatabaseConnection, id: i32) {
    let project = match project_dao::find_projects_by_id(db, id).await {
        Ok(p) => p,
        Err(e) => panic!("{e:?}"),
    };

    let tasks = match task_dao::find_tasks_by_project_id(db, project.id).await {
        Ok(t) => t,
        Err(e) => panic!("{e:?}"),
    };
    println!("{project:#?}");
    let tlen = tasks.len();
    if tlen != 0 {
        println!("Tasks");
        for t in tasks.iter().take(tlen - 1) {
            let status = match t.status {
                TaskStatus::Unfinished => "",
                TaskStatus::Completed => "✅",
                TaskStatus::Discarded => "❎",
            };
            println!("├── {} {} {}", t.id, t.name, status);
        }
        let last = tasks.last().unwrap();
        let status = match last.status {
            TaskStatus::Unfinished => "",
            TaskStatus::Completed => "✅",
            TaskStatus::Discarded => "❎",
        };
        println!("└── {} {} {}", last.id, last.name, status);
    }
}
