use crate::util;
use rtdb::tasks::TaskType;
use rtdb::{config, task_dao};
use sea_orm::DatabaseConnection;
use std::fs::File;
use std::path::PathBuf;

pub async fn add_task(db: &DatabaseConnection) {
    println!("TaskName:");
    let name = util::get_dialog_answer("TaskName", "").trim().to_string();
    println!("{name}\n");

    println!("MdLink:");
    let link = util::get_dialog_answer("MdLink", (name.clone() + ".md").as_str())
        .trim()
        .to_string();
    println!("{link}\n");
    let md_link = match link.as_str() {
        "null" => None,
        _ => Some(link),
    };

    let task_type = TaskType::Inbox;
    if md_link.is_some() && md_link.as_ref().unwrap().ends_with("md") {
        let mut task_dir = PathBuf::from(config::task_dir());
        let folder = "inbox";
        task_dir.push(folder);
        task_dir.push(md_link.clone().unwrap());
        if !task_dir.exists() {
            File::create(task_dir).expect("failed to create file");
        }
    }

    let weight = 1;

    match task_dao::add_task(db, name, md_link, task_type, weight).await {
        Ok(t) => {
            println!("{t:?}")
        }
        Err(e) => {
            println!("{e:?}");
        }
    };
}
