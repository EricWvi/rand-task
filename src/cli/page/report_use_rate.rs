use crate::cli::util;
use crate::Page;

pub struct ReportUseRatePage {
    title: String,
    options: Vec<String>,
}

impl ReportUseRatePage {
    pub fn new() -> ReportUseRatePage {
        ReportUseRatePage {
            title: "Report use rate:   [0-100%]".to_string(),
            options: vec![],
        }
    }

    pub fn eval(&self) -> String {
        let input = util::get_input();
        input.trim().parse::<i32>().expect("invalid number");
        input.trim().to_string() + "%"
    }
}

impl Page for ReportUseRatePage {
    fn title(&self) -> &String {
        &self.title
    }

    fn options(&self) -> &Vec<String> {
        &self.options
    }
}
