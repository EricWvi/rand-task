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
static TASK: OnceCell<Mutex<Task>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    let db = rtdb::init().await.expect("failed to connect db");
    init_log();
    record::init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { pname }) => add_project(db, pname).await,
        Some(Commands::Classify) => classify_projects(db).await,
        Some(Commands::Complete { ids }) => complete_project(db, ids).await,
        Some(Commands::Deschedule { ids }) => deschedule_project(db, ids).await,
        Some(Commands::Get { id }) => get_project(db, *id).await,
        Some(Commands::Impromptu) => impromptu_project().await,
        Some(Commands::List { all }) => list_projects(db, *all).await,
        Some(Commands::Schedule { ids }) => schedule_project(db, ids).await,
        Some(Commands::Search { q }) => search_project(db, q).await,
        Some(Commands::Select { id }) => select_project(db, id).await,
        Some(Commands::Task { command }) => match command {
            TaskCommand::Add { pid } => add_task(db, *pid).await,
            TaskCommand::Complete { tids } => complete_tasks(db, tids).await,
            TaskCommand::Top { tid } => top_task(db, *tid).await,
            TaskCommand::Update { tid } => update_task(db, *tid).await,
        },
        Some(Commands::Today) => today().await,
        Some(Commands::Update { id }) => update_project(db, *id).await,
        None => rt().await,
    }
}

fn init_log() {
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
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .with_writer(Mutex::new(file))
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            ),
        ))
        .init();
}
