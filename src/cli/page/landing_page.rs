use super::*;
use crate::cli::util::rand_task;
use rtdb::tasks::TaskType;
use rtdb::{task_dao, tasks, Task, DB};

pub struct LandingPage {
    title: String,
    options: Vec<String>,
}

impl LandingPage {
    pub fn new() -> LandingPage {
        LandingPage {
            title: "How tired are you now?".to_string(),
            options: vec![
                "I want to focus on another thing. 💪️".to_string(),
                "I need to take a break. 🎧".to_string(),
                "I am very tired. 🪫".to_string(),
            ],
        }
    }

    pub async fn eval(&self) {
        let c = self.eval_choice();
        let tasks = match c {
            'a' => focus_another_thing_tasks().await,
            'b' => take_a_break_tasks().await,
            'c' => tired_tasks().await,
            _ => vec![],
        };
        let task = rand_task(&tasks).unwrap();
        println!("Task: {}", task.name);

        crate::TASK.set(task.clone());

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

async fn focus_another_thing_tasks() -> Vec<Task> {
    let db = DB.get().unwrap();
    task_dao::find_tasks_by_type(db, TaskType::FocusAnotherThing)
        .await
        .expect("failed to find tasks by TaskType::FocusAnotherThing")
}

async fn take_a_break_tasks() -> Vec<Task> {
    let db = DB.get().unwrap();
    task_dao::find_tasks_by_type(db, TaskType::TakeABreak)
        .await
        .expect("failed to find tasks by TaskType::TakeABreak")
}

async fn tired_tasks() -> Vec<Task> {
    let db = DB.get().unwrap();
    task_dao::find_tasks_by_type(db, TaskType::Tired)
        .await
        .expect("failed to find tasks by TaskType::Tired")
}
