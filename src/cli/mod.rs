pub mod page;
pub mod util;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// take an unscheduled task
    Impromptu,
    /// select a specific task
    Select { id: i32 },
}
