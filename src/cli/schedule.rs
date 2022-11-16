use rtdb::project_dao;
use rtdb::projects::ProjectStatus;
use sea_orm::DatabaseConnection;

pub async fn schedule_project(db: &DatabaseConnection, ids: &Vec<i32>) {
    for id in ids {
        let project = project_dao::find_projects_by_id(db, *id)
            .await
            .expect("failed to find project by id from db");
        print!("Project: {}, {:?} to ", project.name, project.status);
        let project = project_dao::update_status(db, project, ProjectStatus::Scheduled)
            .await
            .expect("failed to update project's status'");
        println!("{:?}", project.status)
    }
}

pub async fn deschedule_project(db: &DatabaseConnection, ids: &Vec<i32>) {
    for id in ids {
        let project = project_dao::find_projects_by_id(db, *id)
            .await
            .expect("failed to find projects by id from db");
        print!("Project: {}, {:?} to ", project.name, project.status);
        let project = project_dao::update_status(db, project, ProjectStatus::Pending)
            .await
            .expect("failed to update project's status");
        println!("{:?}", project.status)
    }
}
