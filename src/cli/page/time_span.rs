use crate::cli::page::{FinishTaskPage, ModifyWeightPage, TickTockPage};
use crate::cli::util;
use crate::Page;
use chrono::{NaiveDate, NaiveDateTime};
use rtdb::{record_dao, DB};
use std::ops::Sub;
use std::str::FromStr;

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
    if let Some(link) = crate::TASK.get().unwrap().md_link.as_ref() {
        util::open_md(link);
    }

    let start = chrono::Local::now().naive_local();
    println!("{start}");

    let page = TickTockPage::new();
    page.eval(total);
    let page = FinishTaskPage::new();
    page.display();
    page.eval();
    let page = ModifyWeightPage::new();
    page.display();
    page.eval().await;

    let end = chrono::Local::now().naive_local();
    println!("{end}");

    let span = end.sub(start);
    let min = span.num_minutes();
    println!("Schedule: {total}min, Actual: {min}min");

    let db = DB.get().unwrap();
    let task = crate::TASK.get().unwrap();
    record_dao::add_record(db, task.name.clone(), start, total, min as i32).await;
}

#[test]
fn test_time() {
    let start = NaiveDate::from_ymd(2022, 10, 15).and_hms(9, 29, 38);
    let end = NaiveDate::from_ymd(2022, 10, 15).and_hms(10, 17, 24);
    let span = end.sub(start);
    dbg!(span.num_minutes());
    dbg!(span.num_seconds() % 60);
}
