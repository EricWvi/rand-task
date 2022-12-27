use super::*;
use crate::cli::util;
use crate::cli::util::rand_task;
use rtdb::projects::ProjectType;
use rtdb::{project_dao, Project};

pub struct LandingPage {
    title: String,
    options: Vec<String>,
}

impl LandingPage {
    pub fn new() -> LandingPage {
        LandingPage {
            title: "Which type will you choose?".to_string(),
            options: vec![
                "💻 Today".to_string(),
                "💬 En".to_string(),
                "💪 Focus another thing".to_string(),
                "🎧 Take a break".to_string(),
                "🪫 Tired".to_string(),
            ],
        }
    }

    pub async fn eval(&self) {
        let c = self.eval_choice();
        let projects = match c {
            'a' => work_projects().await,
            'b' => en_projects().await,
            'c' => focus_another_thing_projects().await,
            'd' => take_a_break_projects().await,
            'e' => tired_projects().await,
            _ => vec![],
        };
        if projects.len() == 0 {
            println!("You have done all the projects of this type. ✅");
            return;
        }
        let project = rand_task(&projects).unwrap();
        util::set_global_project(project);
        util::set_global_task(project).await;

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

pub async fn work_projects() -> Vec<Project> {
    let db = rtdb::db();
    project_dao::find_projects_by_type(db, ProjectType::Today, false, false)
        .await
        .expect("failed to find projects by ProjectType::Today")
}

pub async fn en_projects() -> Vec<Project> {
    let db = rtdb::db();
    project_dao::find_projects_by_type(db, ProjectType::En, false, false)
        .await
        .expect("failed to find projects by ProjectType::En")
}

pub async fn focus_another_thing_projects() -> Vec<Project> {
    let db = rtdb::db();
    project_dao::find_projects_by_type(db, ProjectType::FocusAnotherThing, false, false)
        .await
        .expect("failed to find projects by ProjectType::FocusAnotherThing")
}

pub async fn take_a_break_projects() -> Vec<Project> {
    let db = rtdb::db();
    project_dao::find_projects_by_type(db, ProjectType::TakeABreak, false, false)
        .await
        .expect("failed to find projects by ProjectType::TakeABreak")
}

pub async fn tired_projects() -> Vec<Project> {
    let db = rtdb::db();
    project_dao::find_projects_by_type(db, ProjectType::Tired, false, false)
        .await
        .expect("failed to find projects by ProjectType::Tired")
}
