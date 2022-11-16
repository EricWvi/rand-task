use rtdb::projects::{ProjectStatus, ProjectType};
use rtdb::{project_dao, util};
use sea_orm::DatabaseConnection;
use std::fs;
use std::path::PathBuf;

pub async fn complete_project(db: &DatabaseConnection, ids: &Vec<i32>) {
    for id in ids {
        let project = project_dao::find_projects_by_id(db, *id)
            .await
            .expect("failed to find project by id from db");
        print!("Project: {}, {:?} to ", project.name, project.status);
        let project = project_dao::update_status(db, project, ProjectStatus::Completed)
            .await
            .expect("failed to update project's status'");
        println!("{:?}", project.status);

        if util::is_rt_md(project.md_link.as_ref()) {
            let dir = rtdb::config::project_dir();
            let file_name = project.md_link.as_ref().unwrap();
            let mut prev = PathBuf::from(dir);
            let project_type = match project.r#type {
                ProjectType::FocusAnotherThing => "focus-another-thing",
                ProjectType::TakeABreak => "take-a-break",
                ProjectType::Tired => "tired",
                ProjectType::Today => "current-work",
                ProjectType::Inbox => "inbox",
                ProjectType::En => "en",
            };
            prev.push(project_type);
            prev.push(file_name);
            let mut curr = PathBuf::from(dir);
            curr.push("completed");
            curr.push(file_name);
            if prev.exists() {
                fs::rename(prev, curr).expect("failed to move md file");
                println!("Moving {project_type}/{file_name} to completed/{file_name}");
            }
        }
    }
}
