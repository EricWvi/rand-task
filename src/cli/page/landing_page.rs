use super::*;
use crate::cli::util::rand_task;
use rtdb::tasks::TaskType;
use rtdb::{task_dao, Task};

pub struct LandingPage {
    title: String,
    options: Vec<String>,
}

impl LandingPage {
    pub fn new() -> LandingPage {
        LandingPage {
            title: "How tired are you now?".to_string(),
            options: vec![
                "💻 I am going to work hard.".to_string(),
                "💪 I want to focus on another thing.".to_string(),
                "🎧 I need to take a break.".to_string(),
                "🪫 I am very tired.".to_string(),
            ],
        }
    }

    pub async fn eval(&self) {
        let c = self.eval_choice();
        let tasks = match c {
            'a' => work_tasks().await,
            'b' => focus_another_thing_tasks().await,
            'c' => take_a_break_tasks().await,
            'd' => tired_tasks().await,
            _ => vec![],
        };
        if tasks.len() == 0 {
            println!("You have done all the tasks of this type. ✅");
            return;
        }
        let task = rand_task(&tasks).unwrap();
        println!("Task: {}", task.name);

        crate::TASK
            .set(task.clone())
            .expect("failed to set global TASK");

        let time_span = TimeSpanPage::new();
        time_span.display();
        time_span.eval().await;
    }
}

impl Page for LandingPage {
    fn title(&self) -> &String {
        &self.title
    }

    fn options(&self) -> &Vec<String> {
        &self.options
    }
}

pub async fn work_tasks() -> Vec<Task> {
    let db = rtdb::db();
    task_dao::find_tasks_by_type(db, TaskType::Today, false, false)
        .await
        .expect("failed to find tasks by TaskType::Today")
}

pub async fn en_tasks() -> Vec<Task> {
    let db = rtdb::db();
    task_dao::find_tasks_by_type(db, TaskType::En, false, false)
        .await
        .expect("failed to find tasks by TaskType::En")
}

pub async fn focus_another_thing_tasks() -> Vec<Task> {
    let db = rtdb::db();
    task_dao::find_tasks_by_type(db, TaskType::FocusAnotherThing, false, false)
        .await
        .expect("failed to find tasks by TaskType::FocusAnotherThing")
}

pub async fn take_a_break_tasks() -> Vec<Task> {
    let db = rtdb::db();
    task_dao::find_tasks_by_type(db, TaskType::TakeABreak, false, false)
        .await
        .expect("failed to find tasks by TaskType::TakeABreak")
}

pub async fn tired_tasks() -> Vec<Task> {
    let db = rtdb::db();
    task_dao::find_tasks_by_type(db, TaskType::Tired, false, false)
        .await
        .expect("failed to find tasks by TaskType::Tired")
}
