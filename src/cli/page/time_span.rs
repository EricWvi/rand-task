use crate::cli::page::{
    CompleteTaskPage, FinishTaskPage, ModifyWeightPage, ReportUseRatePage, TickTockPage,
};
use crate::cli::util;
use crate::Page;
use rtdb::record_dao;
use std::ops::Sub;

pub struct TimeSpanPage {
    title: String,
    options: Vec<String>,
}

impl TimeSpanPage {
    pub fn new() -> TimeSpanPage {
        TimeSpanPage {
            title: "How much time will you spend?".to_string(),
            options: vec![
                "20min".to_string(),
                "30min".to_string(),
                "40min".to_string(),
                "custom".to_string(),
            ],
        }
    }

    pub async fn eval(&self) {
        let c = self.eval_choice();
        match c {
            'a' => start_task(20).await,
            'b' => start_task(30).await,
            'c' => start_task(40).await,
            'd' => {
                println!("Minutes:");
                let input = util::get_input();
                let min = input.trim().parse::<i32>().expect("invalid number");
                start_task(min).await
            }
            _ => (),
        };
    }
}

impl Page for TimeSpanPage {
    fn title(&self) -> &String {
        &self.title
    }

    fn options(&self) -> &Vec<String> {
        &self.options
    }
}

async fn start_task(total: i32) {
    let project = crate::PROJECT.get().unwrap();
    let task = crate::TASK.get();
    if let Some(link) = project.md_link.as_ref() {
        util::open_link(link);
    }
    if task.is_some() {
        if let Some(link) = task.unwrap().lock().unwrap().file_link.as_ref() {
            util::open_link(link);
        }
    }

    let start = chrono::Local::now().naive_local();
    println!("{start}");

    let page = TickTockPage::new();
    page.eval(total).await;

    let stop = chrono::Local::now().naive_local();

    let page = FinishTaskPage::new();
    page.display();
    page.eval();
    if project.id != 0 {
        let page = ModifyWeightPage::new();
        page.display();
        page.eval().await;

        if task.is_some() {
            if task.unwrap().lock().unwrap().id != 0 {
                let page = CompleteTaskPage::new();
                page.display();
                page.eval().await;
            }
        }
    }
    let page = ReportUseRatePage::new();
    page.display();
    let use_rate = page.eval();

    let end = chrono::Local::now().naive_local();
    println!("{end}");

    let span = end.sub(stop);
    let min = span.num_minutes() + total as i64;
    println!("Schedule: {total}min, Actual: {min}min");

    let db = rtdb::db();
    let (task_name, task_id) = if task.is_some() {
        let task = task.unwrap().lock().unwrap();
        (format!("{} - {}", project.name, task.name), task.id)
    } else {
        (project.name.clone(), 0)
    };
    match record_dao::add_record(
        db,
        task_name,
        start,
        total,
        min as i32,
        use_rate,
        project.r#type,
        project.id,
        task_id,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            println!("{e:?}");
        }
    }
}
