use crate::record::ToDo;

pub async fn today(todo: ToDo) {
    println!("{}", todo);
}
