use crate::util;
use rtdb::{project_dao, task_dao};
use sea_orm::DatabaseConnection;

pub async fn add_task(db: &DatabaseConnection, project_id: i32) {
    let project = match project_dao::find_projects_by_id(db, project_id).await {
        Ok(t) => t,
        Err(e) => panic!("{e:?}"),
    };
    println!("Project {project:#?}");

    println!("TaskName:");
    let name = util::get_dialog_answer("TaskName", "").trim().to_string();
    println!("{name}\n");

    println!("FileLink:");
    let link = util::get_dialog_answer("FileLink", "null")
        .trim()
        .to_string();
    println!("{link}\n");
    let file_link = match link.as_str() {
        "null" => None,
        _ => Some(link),
    };

    match task_dao::add_task(db, name, file_link, project_id).await {
        Ok(t) => {
            println!("{t:?}")
        }
        Err(e) => {
            println!("{e:?}");
        }
    };
}
