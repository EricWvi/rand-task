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
    let todo = record::init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Impromptu) => {
            println!("TaskName:");
            let name = util::get_dialog_answer("TaskName", "").trim().to_string();
            println!("{name}\n");
            println!("TaskType:  a.Today  b.Focus another thing  c.Take a break  d.Tired");
            let choice = util::eval_choice(4, false);
            let task_type = match choice as char {
                'a' => TaskType::Today,
                'b' => TaskType::FocusAnotherThing,
                'c' => TaskType::TakeABreak,
                'd' => TaskType::Tired,
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
            let task = match rtdb::task_dao::find_tasks_by_id(rtdb::db(), *id).await {
                Ok(task) => task,
                Err(e) => panic!("{e:?}"),
            };
            TASK.set(task).expect("failed to set global TASK");
            println!("Task: {}", TASK.get().unwrap().name);

            let time_span = TimeSpanPage::new();
            time_span.display();
            time_span.eval().await;
        }
        None => {
            if todo == "111111111111" {
                let landing_page = LandingPage::new();
                landing_page.display();
                landing_page.eval().await;
            } else {
                let task_type = match todo.as_str() {
                    "000000000000" => TaskType::Today,
                    "100000000000" => TaskType::FocusAnotherThing,
                    "110000000000" => TaskType::Today,
                    "111000000000" => TaskType::FocusAnotherThing,
                    "111100000000" => TaskType::Today,
                    "111110000000" => TaskType::TakeABreak,
                    "111111000000" => TaskType::Today,
                    "111111100000" => TaskType::TakeABreak,
                    "111111110000" => TaskType::Today,
                    "111111111000" => TaskType::Tired,
                    "111111111100" => TaskType::Today,
                    "111111111110" => TaskType::Tired,
                    _ => unreachable!(),
                };
                let tasks = match task_type {
                    TaskType::FocusAnotherThing => focus_another_thing_tasks().await,
                    TaskType::TakeABreak => take_a_break_tasks().await,
                    TaskType::Tired => tired_tasks().await,
                    TaskType::Today => work_tasks().await,
                    TaskType::Inbox => unreachable!(),
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

                let mut new = todo.clone();
                let count = new.bytes().filter(|c| *c == '1' as u8).count();
                unsafe {
                    let bytes = new.as_bytes_mut();
                    bytes[count] = '1' as u8;
                }
                record::flush_todo(todo.clone(), new);
            }
        }
    }
}
