use crate::cli::page::{Page, TimeSpanPage};
use crate::cli::util;
use crate::record;
use crate::record::ToDo;
use sea_orm::DatabaseConnection;

pub async fn select_project(db: &DatabaseConnection, id: i32, mut todo: ToDo) {
    let old: String = todo.clone().into();
    let project = match rtdb::project_dao::find_projects_by_id(db, id).await {
        Ok(project) => project,
        Err(e) => panic!("{e:?}"),
    };
    todo.select_type(project.r#type);
    tracing::info!(?project);
    util::set_global(&project).await;

    let time_span = TimeSpanPage::new();
    time_span.display();
    time_span.eval().await;

    record::flush_todo(old, todo.into());
}
