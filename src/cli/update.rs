use rtdb::projects::{ProjectStatus, ProjectType};
use rtdb::{project_dao, util, Project};
use sea_orm::DatabaseConnection;
use std::fs;
use std::path::PathBuf;

pub async fn update_project(db: &DatabaseConnection, id: i32) {
    let project = match project_dao::find_projects_by_id(db, id).await {
        Ok(t) => t,
        Err(e) => panic!("{e:?}"),
    };

    let prev_name = match project.md_link.as_ref() {
        None => "null",
        Some(link) => link,
    };
    let name = util::get_dialog_answer("Name", &*project.name);
    println!("Name: {name}\n");
    let md_link = match util::get_dialog_answer(
        "Md Link",
        &*project.md_link.as_ref().unwrap_or(&"null".to_string()),
    )
    .as_str()
    {
        "null" => None,
        s => Some(s.to_string()),
    };
    println!(
        "Md Link: {}\n",
        md_link.as_ref().unwrap_or(&"null".to_string())
    );

    let prev_status = project.status;
    let prev_type = project.r#type;
    println!(
        "ProjectType:  a.Inbox  b.Today  c.En  d.Focus another thing  e.Take a break  f.Tired  [{:?}]",
        project.r#type
    );
    let r#type = match util::eval_choice(6, true) {
        'a' => ProjectType::Inbox,
        'b' => ProjectType::Today,
        'c' => ProjectType::En,
        'd' => ProjectType::FocusAnotherThing,
        'e' => ProjectType::TakeABreak,
        'f' => ProjectType::Tired,
        '\n' => project.r#type,
        _ => unreachable!(),
    };

    println!("Weight: [{}]", project.weight);
    let input = util::get_input();
    let weight = if input.trim().len() == 0 {
        project.weight
    } else {
        input.trim().parse::<i32>().expect("invalid number")
    };

    println!(
        "ProjectStatus:  a.Pending  b.Scheduled  c.Unfinished  d.Completed  e.Discarded  [{:?}]",
        project.status
    );
    let status = match util::eval_choice(4, true) {
        'a' => ProjectStatus::Pending,
        'b' => ProjectStatus::Scheduled,
        'c' => ProjectStatus::Completed,
        'd' => ProjectStatus::Discarded,
        '\n' => project.status,
        _ => unreachable!(),
    };
    let project_new = Project {
        id,
        name,
        md_link,
        r#type,
        weight,
        status,
    };
    let project = project_dao::update_project(db, &project, &project_new)
        .await
        .expect(&*format!("failed to update project[id={}]", project.id));

    if util::is_rt_md(Some(&prev_name.to_string())) {
        let dir = rtdb::config::project_dir();
        let mut prev = PathBuf::from(dir);
        let prev_dir =
            if prev_status == ProjectStatus::Completed || prev_status == ProjectStatus::Discarded {
                "completed"
            } else {
                match prev_type {
                    ProjectType::FocusAnotherThing => "focus-another-thing",
                    ProjectType::TakeABreak => "take-a-break",
                    ProjectType::Tired => "tired",
                    ProjectType::Today => "current-work",
                    ProjectType::Inbox => "inbox",
                    ProjectType::En => "en",
                }
            };
        prev.push(prev_dir);
        prev.push(prev_name);
        let mut curr = PathBuf::from(dir);
        let curr_dir = if project.status == ProjectStatus::Completed
            || project.status == ProjectStatus::Discarded
        {
            "completed"
        } else {
            match project.r#type {
                ProjectType::FocusAnotherThing => "focus-another-thing",
                ProjectType::TakeABreak => "take-a-break",
                ProjectType::Tired => "tired",
                ProjectType::Today => "current-work",
                ProjectType::Inbox => "inbox",
                ProjectType::En => "en",
            }
        };
        curr.push(curr_dir);
        let file_name = project.md_link.as_ref().unwrap();
        curr.push(file_name);
        if util::is_rt_md(project.md_link.as_ref()) && prev != curr {
            if prev.exists() {
                fs::rename(prev, curr).expect("failed to move md file");
                println!("Moving {prev_dir}/{prev_name} to {curr_dir}/{file_name}");
            }
        }
    }
}
