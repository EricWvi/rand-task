use crate::cli::page::*;
use crate::cli::util::rand_task;
use crate::record::ToDo;
use crate::{record, TASK};
use rtdb::tasks::TaskType;

pub async fn rt(mut todo: ToDo) {
    let old: String = todo.clone().into();
    let task_type = todo.next();
    if task_type.is_none() {
        let landing_page = LandingPage::new();
        landing_page.display();
        landing_page.eval().await;
    } else {
        let tasks = match task_type.unwrap() {
            TaskType::FocusAnotherThing => focus_another_thing_tasks().await,
            TaskType::TakeABreak => take_a_break_tasks().await,
            TaskType::Tired => tired_tasks().await,
            TaskType::Today => work_tasks().await,
            TaskType::Inbox => unreachable!(),
            TaskType::En => en_tasks().await,
        };
        if tasks.len() == 0 {
            println!(
                "You have done all the tasks of TaskType::{:?}. ✅",
                task_type.unwrap()
            );
            return;
        }
        let task = rand_task(&tasks).unwrap();
        tracing::info!(?task);
        println!("Task: {}", task.name);

        TASK.set(task.clone()).expect("failed to set global TASK");

        let time_span = TimeSpanPage::new();
        time_span.display();
        time_span.eval().await;

        record::flush_todo(old, todo.into());
    }
}
