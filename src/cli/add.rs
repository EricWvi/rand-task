use crate::util;
use rtdb::projects::ProjectType;
use rtdb::{config, project_dao};
use sea_orm::DatabaseConnection;
use std::fs::File;
use std::path::PathBuf;

pub async fn add_project(db: &DatabaseConnection, pname: &Option<String>) {
    let (name, link) = if pname.is_none() {
        println!("ProjectName:");
        let name = util::get_dialog_answer("ProjectName", "")
            .trim()
            .to_string();
        println!("{name}\n");

        println!("MdLink:");
        let link = util::get_dialog_answer("MdLink", (name.clone() + ".md").as_str())
            .trim()
            .to_string();
        println!("{link}\n");
        (name, link)
    } else {
        (pname.as_ref().unwrap().clone(), "null".to_string())
    };

    let md_link = match link.as_str() {
        "null" => None,
        _ => Some(link),
    };

    let project_type = ProjectType::Inbox;
    if md_link.is_some() && md_link.as_ref().unwrap().ends_with("md") {
        let mut project_dir = PathBuf::from(config::project_dir());
        let folder = "inbox";
        project_dir.push(folder);
        project_dir.push(md_link.clone().unwrap());
        if !project_dir.exists() {
            File::create(project_dir).expect("failed to create file");
        }
    }

    let weight = 1;

    match project_dao::add_project(db, name, md_link, project_type, weight).await {
        Ok(t) => {
            println!("{t:?}")
        }
        Err(e) => {
            println!("{e:?}");
        }
    };
}
