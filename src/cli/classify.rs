use rtdb::projects::ProjectType;
use rtdb::{project_dao, util};
use sea_orm::DatabaseConnection;
use std::fs;
use std::path::PathBuf;

pub async fn classify_projects(db: &DatabaseConnection) {
    let projects = project_dao::find_projects_by_type(db, ProjectType::Inbox, true, false)
        .await
        .expect("failed to find projects by type from db");
    for t in projects {
        let project_type = move_to(&*t.name);
        println!("Project:{}, moving to {:?}", t.name, project_type);
        let msg = format!("failed to move project[id={}]", t.id);
        let project = project_dao::update_type(db, t, project_type)
            .await
            .expect(&*msg);

        if util::is_rt_md(project.md_link.as_ref()) {
            let dir = rtdb::config::project_dir();
            let mut prev = PathBuf::from(dir);
            prev.push("inbox");
            prev.push(project.md_link.as_ref().unwrap());
            let mut curr = PathBuf::from(dir);
            curr.push(match project_type {
                ProjectType::FocusAnotherThing => "focus-another-thing",
                ProjectType::TakeABreak => "take-a-break",
                ProjectType::Tired => "tired",
                ProjectType::Today => "current-work",
                ProjectType::Inbox => "inbox",
                ProjectType::En => "en",
            });
            curr.push(project.md_link.as_ref().unwrap());
            if prev.exists() {
                fs::rename(prev, curr).expect("failed to move md file");
            }
        }
    }
}

fn move_to(name: &str) -> ProjectType {
    let title = format!("Move {} to:", name);
    util::choose_from_list(
        &*title,
        vec![
            ProjectType::Inbox.into(),
            ProjectType::Today.into(),
            ProjectType::En.into(),
            ProjectType::FocusAnotherThing.into(),
            ProjectType::TakeABreak.into(),
            ProjectType::Tired.into(),
        ],
        vec![0],
    )[0]
    .clone()
    .into()
}
