#![feature(async_closure)]

use crate::cli::page::{LandingPage, Page, TimeSpanPage};
use crate::cli::{util, Cli, Commands};
use clap::Parser;
use rtdb::tasks::{TaskStatus, TaskType};
use rtdb::Task;
use tokio::sync::OnceCell;

mod cli;

static TASK: OnceCell<Task> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    rtdb::init().await.expect("failed to connect db");

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Impromptu) => {
            println!("TaskName:");
            let name = util::get_dialog_answer("TaskName", "").trim().to_string();
            println!("{name}\n");
            println!("TaskType:  a.Today  b.Focus another thing  c.Take a break  d.Tired");
            let choice = util::eval_choice(4);
            let task_type = match choice as char {
                'a' => TaskType::Today,
                'b' => TaskType::FocusAnotherThing,
                'c' => TaskType::TakeABreak,
                'd' => TaskType::Tired,
                _ => TaskType::Today,
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
            let task = match rtdb::task_dao::find_tasks_by_id(rtdb::DB.get().unwrap(), *id).await {
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
            let landing_page = LandingPage::new();
            landing_page.display();
            landing_page.eval().await;
        }
    }
}
