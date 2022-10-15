use crate::cli::util;
use crate::Page;

pub struct FinishTaskPage {
    title: String,
    options: Vec<String>,
}

impl FinishTaskPage {
    pub fn new() -> FinishTaskPage {
        FinishTaskPage {
            title: "Finished.   [Enter]".to_string(),
            options: vec![],
        }
    }

    pub fn eval(&self) {
        util::get_input();
    }
}

impl Page for FinishTaskPage {
    fn title(&self) -> &String {
        &self.title
    }

    fn options(&self) -> &Vec<String> {
        &self.options
    }
}
