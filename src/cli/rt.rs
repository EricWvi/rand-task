use crate::cli::page::*;
use crate::cli::util;
use crate::cli::util::rand_task;
use crate::record;
use crate::record::ToDo;
use rtdb::projects::ProjectType;

pub async fn rt(mut todo: ToDo) {
    let old: String = todo.clone().into();
    let project_type = todo.next();
    if project_type.is_none() {
        let landing_page = LandingPage::new();
        landing_page.display();
        landing_page.eval().await;
    } else {
        let projects = match project_type.unwrap() {
            ProjectType::FocusAnotherThing => focus_another_thing_projects().await,
            ProjectType::TakeABreak => take_a_break_projects().await,
            ProjectType::Tired => tired_projects().await,
            ProjectType::Today => work_projects().await,
            ProjectType::Inbox => unreachable!(),
            ProjectType::En => en_projects().await,
        };
        if projects.len() == 0 {
            println!(
                "You have done all the projects of ProjectType::{:?}. ✅",
                project_type.unwrap()
            );
            return;
        }
        let project = rand_task(&projects).unwrap();
        tracing::info!(?project);
        util::set_global_project(project);
        util::set_global_task(project).await;

        let time_span = TimeSpanPage::new();
        time_span.display();
        time_span.eval().await;

        record::flush_todo(old, todo.into());
    }
}
