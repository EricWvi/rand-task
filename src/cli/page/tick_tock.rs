use crate::cli::page::ctrlc_page::CtrlCPage;
use crate::cli::util;
use crate::cli::util::ASCmd;
use crate::Page;
use std::io::Write;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use wait_timeout::ChildExt;

pub struct TickTockPage {
    title: String,
    options: Vec<String>,
}

impl TickTockPage {
    pub fn new() -> TickTockPage {
        TickTockPage {
            title: "tick-tock".to_string(),
            options: vec![],
        }
    }

    pub async fn eval(&self, total: i32) {
        let mut min = total;
        let mut sec = 0;

        let running = Arc::new(AtomicBool::new(true));
        let normal = Arc::new(AtomicBool::new(false));
        let r = running.clone();
        let n = normal.clone();
        ctrlc::set_handler(move || {
            if n.load(Ordering::SeqCst) {
                process::exit(1);
            }
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        while sec >= 0 && min >= 0 {
            if sec == 0 && min == 5 {
                tokio::spawn((async || {
                    let mut child = util::alert(ASCmd::AlertFiveMinutes).spawn().unwrap();
                    let thirty_sec = Duration::from_secs(30);
                    match child.wait_timeout(thirty_sec).unwrap() {
                        Some(_) => (),
                        None => {
                            child.kill().unwrap();
                            util::send_msg("RandTask: 5 minutes remaining.").await;
                        }
                    };
                })());
            }

            util::progressing_bar(min, sec, total).expect("failed to plot progressing bar");
            match std::io::stdout().flush() {
                Ok(_) => {}
                Err(e) => {
                    println!("{e:?}");
                }
            };
            sleep(Duration::from_secs(1));
            if sec > 0 {
                sec -= 1;
            } else {
                min -= 1;
                sec = 59;
            }

            // Ctrl + C
            if !running.load(Ordering::SeqCst) {
                let page = CtrlCPage::new();
                page.display();
                page.eval().await;
                running.store(true, Ordering::SeqCst);
            }
        }
        normal.store(true, Ordering::SeqCst);

        println!("\n⏰ Time up!");
        tokio::spawn((async || {
            let mut child = util::alert(ASCmd::AlertFinished).spawn().unwrap();
            let thirty_sec = Duration::from_secs(30);
            match child.wait_timeout(thirty_sec).unwrap() {
                Some(_) => (),
                None => {
                    child.kill().unwrap();
                    util::send_msg("RandTask: Time up!").await;
                }
            };
        })());
    }
}

impl Page for TickTockPage {
    fn title(&self) -> &String {
        &self.title
    }

    fn options(&self) -> &Vec<String> {
        &self.options
    }
}
