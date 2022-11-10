use rtdb::tasks::{TaskStatus, TaskType};
use rtdb::{task_dao, util, Task};
use sea_orm::DatabaseConnection;
use std::fs;
use std::path::PathBuf;

pub async fn update_task(db: &DatabaseConnection, id: i32) {
    let task = match task_dao::find_tasks_by_id(db, id).await {
        Ok(t) => t,
        Err(e) => panic!("{e:?}"),
    };

    let prev_name = match task.md_link.as_ref() {
        None => "null",
        Some(link) => link,
    };
    let name = util::get_dialog_answer("Name", &*task.name);
    println!("Name: {name}\n");
    let md_link = match util::get_dialog_answer(
        "Md Link",
        &*task.md_link.as_ref().unwrap_or(&"null".to_string()),
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

    let prev_status = task.status;
    let prev_type = task.r#type;
    println!(
        "TaskType:  a.Inbox  b.Today  c.En  d.Focus another thing  e.Take a break  f.Tired  [{:?}]",
        task.r#type
    );
    let r#type = match util::eval_choice(6, true) {
        'a' => TaskType::Inbox,
        'b' => TaskType::Today,
        'c' => TaskType::En,
        'd' => TaskType::FocusAnotherThing,
        'e' => TaskType::TakeABreak,
        'f' => TaskType::Tired,
        '\n' => task.r#type,
        _ => unreachable!(),
    };

    println!("Weight: [{}]", task.weight);
    let input = util::get_input();
    let weight = if input.trim().len() == 0 {
        task.weight
    } else {
        input.trim().parse::<i32>().expect("invalid number")
    };

    println!(
        "TaskStatus:  a.Pending  b.Scheduled  c.Unfinished  d.Completed  [{:?}]",
        task.status
    );
    let status = match util::eval_choice(4, true) {
        'a' => TaskStatus::Pending,
        'b' => TaskStatus::Scheduled,
        'c' => TaskStatus::Unfinished,
        'd' => TaskStatus::Completed,
        '\n' => task.status,
        _ => unreachable!(),
    };
    let task_new = Task {
        id,
        name,
        md_link,
        r#type,
        weight,
        status,
    };
    let task = task_dao::update_task(db, &task, &task_new)
        .await
        .expect(&*format!("failed to update task[id={}]", task.id));

    if prev_name != "null" {
        let dir = rtdb::config::task_dir();
        let mut prev = PathBuf::from(dir);
        let prev_dir = if prev_status == TaskStatus::Completed {
            "completed"
        } else {
            match prev_type {
                TaskType::FocusAnotherThing => "focus-another-thing",
                TaskType::TakeABreak => "take-a-break",
                TaskType::Tired => "tired",
                TaskType::Today => "current-work",
                TaskType::Inbox => "inbox",
                TaskType::En => "en",
            }
        };
        prev.push(prev_dir);
        prev.push(prev_name);
        let mut curr = PathBuf::from(dir);
        let curr_dir = if task.status == TaskStatus::Completed {
            "completed"
        } else {
            match task.r#type {
                TaskType::FocusAnotherThing => "focus-another-thing",
                TaskType::TakeABreak => "take-a-break",
                TaskType::Tired => "tired",
                TaskType::Today => "current-work",
                TaskType::Inbox => "inbox",
                TaskType::En => "en",
            }
        };
        curr.push(curr_dir);
        let file_name = task.md_link.as_ref().unwrap();
        curr.push(file_name);
        if util::is_rt_md(task.md_link.as_ref()) && prev != curr {
            if prev.exists() {
                fs::rename(prev, curr).expect("failed to move md file");
                println!("Moving {prev_dir}/{prev_name} to {curr_dir}/{file_name}");
            }
        }
    }
}
