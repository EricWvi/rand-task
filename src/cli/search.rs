use rtdb::{task_dao, task_view};
use sea_orm::DatabaseConnection;

pub async fn search_task(db: &DatabaseConnection, q: &String) {
    let tasks = task_dao::find_tasks_by_name(db, &*q)
        .await
        .expect("failed to find tasks by type from db");
    let views = tasks
        .into_iter()
        .map(|t| task_view::ListView::from(t))
        .collect::<Vec<_>>();
    for view in views {
        println!("{view}");
    }
}
