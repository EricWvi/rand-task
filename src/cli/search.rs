use rtdb::{project_dao, project_view};
use sea_orm::DatabaseConnection;

pub async fn search_project(db: &DatabaseConnection, q: &String) {
    let projects = project_dao::find_projects_by_name(db, &*q)
        .await
        .expect("failed to find projects by type from db");
    let views = projects
        .into_iter()
        .map(|t| project_view::ListView::from(t))
        .collect::<Vec<_>>();
    for view in views {
        println!("{view}");
    }
}
