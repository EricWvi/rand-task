use crate::cli::page::complete_task::complete_task;
use crate::cli::util;
use crate::cli::util::set_global_task;
use crate::Page;
use std::process::exit;

pub struct CtrlCPage {
    title: String,
    options: Vec<String>,
}

impl CtrlCPage {
    pub fn new() -> CtrlCPage {
        CtrlCPage {
            title: "\nWhich action do you want?".to_string(),
            options: vec![
                "Stop".to_string(),
                "Next task".to_string(),
                "Continue".to_string(),
            ],
        }
    }

    pub async fn eval(&self) {
        let c = self.eval_choice();
        match c {
            'a' => exit(0),
            'b' => {
                complete_task().await;
                set_global_task(crate::PROJECT.get().unwrap()).await;
                let task = crate::TASK.get();
                if task.is_some() {
                    if let Some(link) = task.unwrap().lock().unwrap().file_link.as_ref() {
                        util::open_link(link);
                    }
                }
            }
            'c' => (),
            _ => (),
        }
    }
}

impl Page for CtrlCPage {
    fn title(&self) -> &String {
        &self.title
    }

    fn options(&self) -> &Vec<String> {
        &self.options
    }
}
