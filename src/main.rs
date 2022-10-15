#![feature(async_closure)]

use crate::cli::page::{LandingPage, Page};
use rtdb::Task;
use tokio::sync::OnceCell;

mod cli;

static TASK: OnceCell<Task> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    rtdb::init().await.expect("failed to connect db");

    let landing_page = LandingPage::new();
    landing_page.display();
    landing_page.eval().await;
}
