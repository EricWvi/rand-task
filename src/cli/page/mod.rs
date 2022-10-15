use std::future::Future;
use std::pin::Pin;

mod finish_task;
mod landing_page;
mod modify_weight;
mod tick_tock;
mod time_span;

pub use finish_task::FinishTaskPage;
pub use landing_page::LandingPage;
pub use modify_weight::ModifyWeightPage;
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
        let mut choice = 'a';
        loop {
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(n) => {
                    if input.trim_end().len() != 1 {
                        println!("invalid input");
                        continue;
                    }
                    match input.as_bytes()[0] as char {
                        c @ 'a'..='z' => {
                            let i = input.as_bytes()[0] - 'a' as u8;
                            assert!((i as usize) < self.options().len(), "option out of range");
                            choice = c;
                            break;
                        }
                        _ => {
                            println!("invalid input");
                            continue;
                        }
                    };
                }
                Err(error) => println!("error: {error}"),
            }
        }
        choice
    }
}
