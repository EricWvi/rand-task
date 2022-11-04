use rtdb::Task;
use rtdb::tasks::{TaskStatus, TaskType};
use crate::cli::page::{Page, TimeSpanPage};
use crate::cli::util;
use crate::TASK;

pub async fn impromptu_task() {
    println!("TaskName:");
    let name = util::get_dialog_answer("TaskName", "").trim().to_string();
    println!("{name}\n");
    println!("TaskType:  a.Today  b.Focus another thing  c.En  d.Take a break  e.Tired");
    let choice = util::eval_choice(4, false);
    let task_type = match choice as char {
        'a' => TaskType::Today,
        'b' => TaskType::En,
        'c' => TaskType::FocusAnotherThing,
        'd' => TaskType::TakeABreak,
        'e' => TaskType::Tired,
        _ => unreachable!(),
    };

    TASK.set(Task {
        id: 0,
        name,
        md_link: None,
        r#type: task_type,
        weight: 1,
        status: TaskStatus::Unfinished,
    })
        .expect("failed to set global TASK");

    let task = TASK.get().unwrap();
    tracing::info!(?task);

    let time_span = TimeSpanPage::new();
    time_span.display();
    time_span.eval().await;
}
