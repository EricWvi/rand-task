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
    /// add task to db
    Add,
    /// move tasks in Inbox to other TaskType
    Classify,
    /// change task status to `Completed`
    Complete { ids: Vec<i32> },
    /// change task status from `Scheduled` to `Pending`
    Deschedule { ids: Vec<i32> },
    /// get info of a specific task
    Get { id: i32 },
    /// take an unscheduled task
    Impromptu,
    /// list tasks from db
    List {
        #[arg(short, long)]
        all: bool,
    },
    /// change task status from `Pending` to `Scheduled`
    Schedule { ids: Vec<i32> },
    /// search task info from db
    Search { q: String },
    /// select a specific task
    Select { id: i32 },
    /// update task info
    Update { id: i32 },
}
