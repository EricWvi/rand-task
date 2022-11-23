use crate::cli::page::{Page, TimeSpanPage};
use crate::cli::util;
use crate::record::flush_todo;
use crate::{PROJECT, TASK};
use rtdb::projects::{ProjectStatus, ProjectType};
use rtdb::tasks::TaskStatus;
use rtdb::{Project, Task};
use std::sync::Mutex;

pub async fn impromptu_project() {
    println!("ProjectName:");
    let name = util::get_dialog_answer("ProjectName", "")
        .trim()
        .to_string();
    println!("{name}\n");
    println!("ProjectType:  a.Today  b.Focus another thing  c.En  d.Take a break  e.Tired");
    let choice = util::eval_choice(5, false);
    let project_type = match choice as char {
        'a' => ProjectType::Today,
        'b' => ProjectType::FocusAnotherThing,
        'c' => ProjectType::En,
        'd' => ProjectType::TakeABreak,
        'e' => ProjectType::Tired,
        _ => unreachable!(),
    };

    PROJECT
        .set(Project {
            id: 0,
            name: name.clone(),
            md_link: None,
            r#type: project_type,
            weight: 1,
            status: ProjectStatus::Unfinished,
        })
        .expect("failed to set global PROJECT");

    TASK.set(Mutex::new(Task {
        id: 0,
        name,
        file_link: None,
        project_id: 0,
        status: TaskStatus::Unfinished,
        seq: 0,
    }))
    .expect("failed to set global TASK");

    let project = PROJECT.get().unwrap();
    tracing::info!(?project);

    let time_span = TimeSpanPage::new();
    time_span.display();
    time_span.eval().await;

    flush_todo(project_type);
}
