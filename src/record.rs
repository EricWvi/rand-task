use chrono::Datelike;
use rtdb::tasks::TaskType;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{fs, iter};

pub fn init() -> ToDo {
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
    let lines = content.trim_end().split("\n").collect::<Vec<_>>();
    let line = lines.last();
    let title = format!("{}-{}", date.month(), date.day());
    let bit_vector = if line.is_none() || !line.unwrap().contains(title.as_str()) {
        let mut s = String::with_capacity(TASK_SEQ.len());
        for c in iter::repeat('0').take(TASK_SEQ.len()) {
            s.push(c);
        }
        let pattern = format!("{0}-{1} {2}\n", date.month(), date.day(), s);
        file.write(pattern.as_bytes())
            .expect("failed to write to autogen");
        s
    } else {
        line.unwrap()
            .split(" ")
            .skip(1)
            .next()
            .expect("autogen is corrupted")
            .into()
    };

    bit_vector.into()
}

pub fn flush_todo(old: String, new: String) {
    let mut path = PathBuf::from(rtdb::config::task_dir());
    let date = chrono::Local::today();
    let file_name = format!("{}.md", date.year());
    path.push("review/autogen");
    path.push(file_name);

    let content = fs::read_to_string(&path).expect("failed to read autogen");
    let today = format!("{}-{} ", date.month(), date.day());
    let content = content.replace(
        (today.clone() + old.as_str()).as_str(),
        (today + new.as_str()).as_str(),
    );
    fs::write(&path, content).expect("failed to write to autogen");
}

const TASK_SEQ: [TaskType; 19] = [
    TaskType::Today,
    TaskType::En,
    TaskType::FocusAnotherThing,
    TaskType::Today,
    TaskType::En,
    TaskType::Today,
    TaskType::FocusAnotherThing,
    TaskType::Today,
    TaskType::En,
    TaskType::TakeABreak,
    TaskType::Today,
    TaskType::En,
    TaskType::TakeABreak,
    TaskType::Today,
    TaskType::En,
    TaskType::Tired,
    TaskType::Today,
    TaskType::En,
    TaskType::Tired,
];

#[derive(Clone, PartialEq, Debug)]
pub struct ToDo {
    inner: Vec<(bool, TaskType)>,
}

impl ToDo {
    pub fn next(&mut self) -> Option<TaskType> {
        let mut index = 0;
        let mut task_type: Option<TaskType> = None;
        for (i, (b, t)) in self.inner.iter().enumerate() {
            if *b {
                continue;
            } else {
                index = i;
                task_type = Some(*t);
                break;
            }
        }
        if task_type.is_some() {
            self.inner[index].0 = true;
        }
        task_type
    }

    pub fn select_type(&mut self, task_type: TaskType) {
        let mut index = -1;
        for (i, (b, t)) in self.inner.iter().enumerate() {
            if !*b && *t == task_type {
                index = i as i32;
                break;
            }
        }
        if index != -1 {
            self.inner[index as usize].0 = true;
        }
    }
}

impl Into<String> for ToDo {
    fn into(self) -> String {
        let mut s = String::with_capacity(TASK_SEQ.len());
        for (b, _) in self.inner {
            if b {
                s.push('1')
            } else {
                s.push('0')
            }
        }
        s
    }
}

impl From<String> for ToDo {
    fn from(value: String) -> Self {
        assert_eq!(value.len(), TASK_SEQ.len());
        let mut todo = ToDo {
            inner: Vec::with_capacity(TASK_SEQ.len()),
        };
        for (i, bit) in value.as_bytes().iter().enumerate() {
            match *bit as char {
                '1' => todo.inner.push((true, TASK_SEQ[i])),
                '0' => todo.inner.push((false, TASK_SEQ[i])),
                _ => panic!("the bit vector is invalid"),
            }
        }
        todo
    }
}

#[cfg(test)]
mod test {
    use crate::record::ToDo;
    use rtdb::tasks::TaskType;

    #[test]
    fn test_next() {
        let mut todo: ToDo = "0000000000000000000".to_string().into();
        assert_eq!(todo.next().unwrap(), TaskType::Today);
        assert_eq!(todo, "1000000000000000000".to_string().into());
        assert_eq!(todo.next().unwrap(), TaskType::En);
        assert_eq!(todo, "1100000000000000000".to_string().into());

        let mut todo: ToDo = "1111111111111111100".to_string().into();
        assert_eq!(todo.next().unwrap(), TaskType::En);
        assert_eq!(todo, "1111111111111111110".to_string().into());
        assert_eq!(todo.next().unwrap(), TaskType::Tired);
        assert_eq!(todo, "1111111111111111111".to_string().into());

        let mut todo: ToDo = "1110111111111111100".to_string().into();
        assert_eq!(todo.next().unwrap(), TaskType::Today);
    }

    #[test]
    fn test_select_type() {
        let mut todo: ToDo = "0000000000000000000".to_string().into();
        todo.select_type(TaskType::En);
        assert_eq!(todo, "0100000000000000000".to_string().into());
        todo.select_type(TaskType::Today);
        assert_eq!(todo, "1100000000000000000".to_string().into());
        let mut todo: ToDo = "1101010100100100100".to_string().into();
        todo.select_type(TaskType::Today);
        todo.select_type(TaskType::Today);
        assert_eq!(todo, "1101010100100100100".to_string().into());
    }
}
