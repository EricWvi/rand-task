#![feature(async_closure)]

use crate::cli::page::*;
use crate::cli::{util, Cli, Commands};
use crate::util::rand_task;
use clap::Parser;
use rtdb::tasks::{TaskStatus, TaskType};
use rtdb::Task;
use tokio::sync::OnceCell;

mod cli;
mod record;

static TASK: OnceCell<Task> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    rtdb::init().await.expect("failed to connect db");
    let mut todo = record::init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Impromptu) => {
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

            let time_span = TimeSpanPage::new();
            time_span.display();
            time_span.eval().await;
        }
        Some(Commands::Select { id }) => {
            let old: String = todo.clone().into();
            let task = match rtdb::task_dao::find_tasks_by_id(rtdb::db(), *id).await {
                Ok(task) => task,
                Err(e) => panic!("{e:?}"),
            };
            todo.select_type(task.r#type);
            TASK.set(task).expect("failed to set global TASK");
            println!("Task: {}", TASK.get().unwrap().name);

            let time_span = TimeSpanPage::new();
            time_span.display();
            time_span.eval().await;

            record::flush_todo(old, todo.into());
        }
        None => {
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
                    println!("You have done all the tasks of TaskType::{task_type:?}. ✅");
                    return;
                }
                let task = rand_task(&tasks).unwrap();
                println!("Task: {}", task.name);

                TASK.set(task.clone()).expect("failed to set global TASK");

                let time_span = TimeSpanPage::new();
                time_span.display();
                time_span.eval().await;

                record::flush_todo(old, todo.into());
            }
        }
    }
}
