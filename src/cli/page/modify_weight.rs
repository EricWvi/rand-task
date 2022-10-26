use crate::cli::util;
use crate::Page;
use rtdb::task_dao;

pub struct ModifyWeightPage {
    title: String,
    options: Vec<String>,
}

impl ModifyWeightPage {
    pub fn new() -> ModifyWeightPage {
        let task = crate::TASK.get().unwrap();
        ModifyWeightPage {
            title: "Modify weight.".to_string(),
            options: vec![
                format!("unchanged({})", task.weight),
                "1".to_string(),
                "5".to_string(),
                "10".to_string(),
                "custom".to_string(),
            ],
        }
    }

    pub async fn eval(&self) {
        let c = self.eval_choice();
        match c {
            'a' => (),
            'b' => modify_weight(1).await,
            'c' => modify_weight(5).await,
            'd' => modify_weight(10).await,
            'e' => {
                println!("Weight:");
                let input = util::get_input();
                let weight = input.trim().parse::<i32>().expect("invalid number");
                modify_weight(weight).await
            }
            _ => (),
        };
    }
}

impl Page for ModifyWeightPage {
    fn title(&self) -> &String {
        &self.title
    }

    fn options(&self) -> &Vec<String> {
        &self.options
    }
}

async fn modify_weight(weight: i32) {
    let db = rtdb::db();
    let task = crate::TASK.get().unwrap();
    match task_dao::update_weight(db, task, weight).await {
        Ok(_) => {}
        Err(e) => {
            println!("{e:?}");
        }
    };
}
