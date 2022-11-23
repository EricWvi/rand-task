use crate::cli::page::{Page, TimeSpanPage};
use crate::cli::util;
use crate::record;
use sea_orm::DatabaseConnection;

pub async fn select_project(db: &DatabaseConnection, id: i32) {
    let project = match rtdb::project_dao::find_projects_by_id(db, id).await {
        Ok(project) => project,
        Err(e) => panic!("{e:?}"),
    };
    tracing::info!(?project);
    util::set_global_project(&project);
    util::set_global_task(&project).await;

    let time_span = TimeSpanPage::new();
    time_span.display();
    time_span.eval().await;

    let has = record::flush_todo(project.r#type);
    if has {
        println!(" TODO {:?} ✅", project.r#type);
    }
}
