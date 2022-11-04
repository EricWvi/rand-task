use rtdb::task_dao;
use sea_orm::DatabaseConnection;

pub async fn get_task(db: &DatabaseConnection, id: i32) {
    let task = match task_dao::find_tasks_by_id(db, id).await {
        Ok(t) => t,
        Err(e) => panic!("{e:?}"),
    };
    println!("{task:#?}");
}
