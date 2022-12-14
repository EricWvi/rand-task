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
            options: vec![],
        }
    }

    pub async fn eval(&self) {
        let c = rtdb::util::yes_or_no();
        match c {
            'n' => (),
            'y' => complete_task().await,
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

pub async fn complete_task() {
    let task = crate::TASK.get();
    if task.is_some() {
        let task = task.unwrap().lock().unwrap();
        if task.id != 0 {
            let msg = format!("failed to complete task[id={}]", task.id);
            task_dao::update_status(rtdb::db(), task.clone(), TaskStatus::Completed)
                .await
                .expect(&*msg);
        }
    }
}
