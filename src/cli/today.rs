use crate::record;

pub async fn today() {
    println!("{}", record::get_todo());
}
