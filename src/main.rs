#![feature(async_closure)]

use crate::cli::page::*;
use crate::cli::*;
use clap::Parser;
use rtdb::{Project, Task};
use std::fs::OpenOptions;
use std::sync::Mutex;
use time::macros::format_description;
use time::UtcOffset;
use tokio::sync::OnceCell;
use tracing_subscriber::fmt::time::OffsetTime;

mod cli;
mod record;

static PROJECT: OnceCell<Project> = OnceCell::const_new();
static TASK: OnceCell<Task> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("rand-task.log");
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(Mutex::new(file))
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            ),
        ))
        .init();

    let db = rtdb::init().await.expect("failed to connect db");
    let todo = record::init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add) => add_project(db).await,
        Some(Commands::Classify) => classify_projects(db).await,
        Some(Commands::Complete { ids }) => complete_project(db, ids).await,
        Some(Commands::Deschedule { ids }) => deschedule_project(db, ids).await,
        Some(Commands::Get { id }) => get_project(db, *id).await,
        Some(Commands::Impromptu) => impromptu_project().await,
        Some(Commands::List { all }) => list_projects(db, *all).await,
        Some(Commands::Schedule { ids }) => schedule_project(db, ids).await,
        Some(Commands::Search { q }) => search_project(db, q).await,
        Some(Commands::Select { id }) => select_project(db, *id, todo).await,
        Some(Commands::Task { command }) => match command {
            TaskCommand::Add { pid } => add_task(db, *pid).await,
        },
        Some(Commands::Today) => today(todo).await,
        Some(Commands::Update { id }) => update_project(db, *id).await,
        None => rt(todo).await,
    }
}
