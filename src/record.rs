use chrono::Datelike;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn init() -> String {
    let mut path = PathBuf::from(rtdb::config::task_dir());
    let date = chrono::Local::today();
    let file_name = format!("{}.md", date.year());
    path.push("review/autogen");
    path.push(file_name);

    let mut file = match fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
    {
        Ok(f) => f,
        Err(e) => panic!("{e:?}"),
    };
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read autogen");
    let mut lines = content.trim_end().split("\n").collect::<Vec<_>>();
    let line = lines.last();
    let title = format!("{}-{}", date.month(), date.day());
    let bit_vector = if line.is_none() || !line.unwrap().contains(title.as_str()) {
        let pattern = format!("{0}-{1} 000000000000\n", date.month(), date.day());
        file.write(pattern.as_bytes())
            .expect("failed to write to autogen");
        "000000000000"
    } else {
        line.unwrap()
            .split(" ")
            .skip(1)
            .next()
            .expect("autogen is corrupted")
    };

    bit_vector.to_string()
}

pub fn flush_todo(old: String, new: String) {
    let mut path = PathBuf::from(rtdb::config::task_dir());
    let date = chrono::Local::today();
    let file_name = format!("{}.md", date.year());
    path.push("review/autogen");
    path.push(file_name);

    let mut content = fs::read_to_string(&path).expect("failed to read autogen");
    let today = format!("{}-{} ", date.month(), date.day());
    let content = content.replace(
        (today.clone() + old.as_str()).as_str(),
        (today + new.as_str()).as_str(),
    );
    fs::write(&path, content).expect("failed to write to autogen");
}
