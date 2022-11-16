use crate::Page;
use rtdb::task_dao;
use rtdb::tasks::TaskStatus;

pub struct CompleteTaskPage {
    title: String,
    options: Vec<String>,
}

impl CompleteTaskPage {
    pub fn new() -> CompleteTaskPage {
        CompleteTaskPage {
            title: "Complete the task?  [y/n]".to_string(),
            options: vec!["1".to_string(), "5".to_string()],
        }
    }

    pub async fn eval(&self) {
        let c = rtdb::util::yes_or_no();
        match c {
            'n' => (),
            'y' => {
                let task = crate::TASK.get();
                if task.is_some() {
                    let msg = format!("failed to complete task[id={}]", task.unwrap().id);
                    task_dao::update_status(
                        rtdb::db(),
                        task.unwrap().clone(),
                        TaskStatus::Completed,
                    )
                    .await
                    .expect(&*msg);
                }
            }
            _ => (),
        };
    }
}

impl Page for CompleteTaskPage {
    fn title(&self) -> &String {
        &self.title
    }

    fn options(&self) -> &Vec<String> {
        &self.options
    }
}
