use sea_orm::DatabaseConnection;
use crate::cli::page::{Page, TimeSpanPage};
use crate::{record, TASK};
use crate::record::ToDo;

pub async fn select_task(db: &DatabaseConnection, id: i32, mut todo: ToDo) {
    let old: String = todo.clone().into();
    let task = match rtdb::task_dao::find_tasks_by_id(db, id).await {
        Ok(task) => task,
        Err(e) => panic!("{e:?}"),
    };
    todo.select_type(task.r#type);
    tracing::info!(?task);
    TASK.set(task).expect("failed to set global TASK");
    println!("Task: {}", TASK.get().unwrap().name);

    let time_span = TimeSpanPage::new();
    time_span.display();
    time_span.eval().await;

    record::flush_todo(old, todo.into());
}
