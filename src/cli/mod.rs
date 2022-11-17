mod add;
mod classify;
mod complete;
mod get;
mod impromptu;
mod list;
pub mod page;
mod rt;
mod schedule;
mod search;
mod select;
mod task;
mod today;
mod update;
pub mod util;

pub use add::*;
pub use classify::*;
pub use complete::*;
pub use get::*;
pub use impromptu::*;
pub use list::*;
pub use rt::*;
pub use schedule::*;
pub use search::*;
pub use select::*;
pub use task::*;
pub use today::*;
pub use update::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// add project to db
    Add,
    /// move projects in Inbox to other ProjectType
    Classify,
    /// change project status to `Completed`
    Complete { ids: Vec<i32> },
    /// change project status from `Scheduled` to `Pending`
    Deschedule { ids: Vec<i32> },
    /// get info of a specific project
    Get { id: i32 },
    /// take an unscheduled project
    Impromptu,
    /// list projects from db
    List {
        #[arg(short, long)]
        all: bool,
    },
    /// change project status from `Pending` to `Scheduled`
    Schedule { ids: Vec<i32> },
    /// search project info from db
    Search { q: String },
    /// select a specific project
    Select { id: i32 },
    /// task-related commands
    Task {
        #[command(subcommand)]
        command: TaskCommand,
    },
    /// show today's todo
    Today,
    /// update project info
    Update { id: i32 },
}

#[derive(Subcommand)]
pub enum TaskCommand {
    /// add task to a project
    Add { pid: i32 },
    /// complete a task
    Complete { tids: Vec<i32> },
    /// update task info
    Update { tid: i32 },
}
