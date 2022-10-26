mod finish_task;
mod landing_page;
mod modify_weight;
mod report_use_rate;
mod tick_tock;
mod time_span;

pub use finish_task::FinishTaskPage;
pub use landing_page::*;
pub use modify_weight::ModifyWeightPage;
pub use report_use_rate::ReportUseRatePage;
pub use tick_tock::TickTockPage;
pub use time_span::TimeSpanPage;

pub trait Page {
    fn display(&self) {
        println!("{}", self.title());
        let mut seq = 'a';
        for op in self.options() {
            println!("\t{seq}. {}", op);
            seq = (seq as u8 + 1) as char;
        }
    }

    fn title(&self) -> &String;

    fn options(&self) -> &Vec<String>;

    fn eval_choice(&self) -> char {
        crate::util::eval_choice(self.options().len() as i32, false)
    }
}
