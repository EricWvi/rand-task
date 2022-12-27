use crate::cli::page::{LandingPage, Page, TimeSpanPage};
use crate::cli::util;
use crate::record;
use sea_orm::DatabaseConnection;

pub async fn select_project(db: &DatabaseConnection, id: &Option<i32>) {
    if id.is_some() {
        let project = match rtdb::project_dao::find_projects_by_id(db, id.unwrap()).await {
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
    } else {
        let landing_page = LandingPage::new();
        landing_page.display();
        landing_page.eval().await;
    }
}
