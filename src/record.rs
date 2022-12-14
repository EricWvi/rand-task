use chrono::Datelike;
use rtdb::projects::ProjectType;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{fs, iter};
use tokio::sync::OnceCell;

static TODO_PATH: OnceCell<PathBuf> = OnceCell::const_new();

pub fn init() {
    let mut path = PathBuf::from(rtdb::config::project_dir());
    let date = chrono::Local::today();
    let file_name = format!("{}.md", date.year());
    path.push("review/autogen");
    path.push(file_name);
    TODO_PATH.set(path).expect("TODO_PATH can not be set");
}

pub fn next_todo() -> Option<ProjectType> {
    get_todo().next()
}

pub fn get_todo() -> ToDo {
    let mut file = match fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(TODO_PATH.get().unwrap())
    {
        Ok(f) => f,
        Err(e) => panic!("{e:?}"),
    };
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read autogen");
    let lines = content.trim_end().split("\n").collect::<Vec<_>>();
    let line = lines.last();
    let date = chrono::Local::today();
    let title = format!("{}-{}", date.month(), date.day());
    let bit_vector = if line.is_none() || !line.unwrap().contains(title.as_str()) {
        let mut s = String::with_capacity(PROJECT_SEQ.len());
        for c in iter::repeat('0').take(PROJECT_SEQ.len()) {
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

pub fn flush_todo(project_type: ProjectType) -> bool {
    // TODO maybe need file lock
    let mut todo: ToDo = get_todo();
    let content = fs::read_to_string(TODO_PATH.get().unwrap()).expect("failed to read autogen");
    let lines = content.trim_end().split("\n").collect::<Vec<_>>();
    let line = lines.last();
    let has = todo.select_type(project_type);
    let date = chrono::Local::today();
    let today = format!("{}-{} ", date.month(), date.day());
    let str: String = todo.into();
    let content = content.replace(line.unwrap(), (today + str.as_str()).as_str());
    fs::write(TODO_PATH.get().unwrap(), content).expect("failed to write to autogen");
    has
}

const PROJECT_SEQ: [ProjectType; 20] = [
    ProjectType::Today,
    ProjectType::Today,
    ProjectType::FocusAnotherThing,
    ProjectType::Today,
    ProjectType::En,
    ProjectType::Tired,
    ProjectType::Today,
    ProjectType::FocusAnotherThing,
    ProjectType::Today,
    ProjectType::En,
    ProjectType::TakeABreak,
    ProjectType::Today,
    ProjectType::En,
    ProjectType::TakeABreak,
    ProjectType::Today,
    ProjectType::En,
    ProjectType::Tired,
    ProjectType::Today,
    ProjectType::En,
    ProjectType::Tired,
];

#[derive(Clone, PartialEq, Debug)]
pub struct ToDo {
    inner: Vec<(bool, ProjectType)>,
}

impl ToDo {
    pub fn next(&mut self) -> Option<ProjectType> {
        let mut index = 0;
        let mut project_type: Option<ProjectType> = None;
        for (i, (b, t)) in self.inner.iter().enumerate() {
            if *b {
                continue;
            } else {
                index = i;
                project_type = Some(*t);
                break;
            }
        }
        if project_type.is_some() {
            self.inner[index].0 = true;
        }
        project_type
    }

    pub fn select_type(&mut self, project_type: ProjectType) -> bool {
        let mut index = -1;
        let mut has = false;
        for (i, (b, t)) in self.inner.iter().enumerate() {
            if !*b && *t == project_type {
                index = i as i32;
                break;
            }
        }
        if index != -1 {
            self.inner[index as usize].0 = true;
            has = true;
        }
        has
    }
}

impl Into<String> for ToDo {
    fn into(self) -> String {
        let mut s = String::with_capacity(PROJECT_SEQ.len());
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

impl From<&str> for ToDo {
    fn from(value: &str) -> Self {
        assert_eq!(value.len(), PROJECT_SEQ.len());
        let mut todo = ToDo {
            inner: Vec::with_capacity(PROJECT_SEQ.len()),
        };
        for (i, bit) in value.as_bytes().iter().enumerate() {
            match *bit as char {
                '1' => todo.inner.push((true, PROJECT_SEQ[i])),
                '0' => todo.inner.push((false, PROJECT_SEQ[i])),
                _ => panic!("the bit vector is invalid"),
            }
        }
        todo
    }
}

impl From<String> for ToDo {
    fn from(value: String) -> Self {
        (&*value).into()
    }
}

impl Display for ToDo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for (status, project_type) in &self.inner {
            if !*status {
                continue;
            }
            str += Into::<&str>::into(*project_type);
            str += " ???\n";
        }
        write!(f, "{}", str)?;
        let mut todo = self.clone();
        let mut tail = String::new();
        for _ in 0..3 {
            let project_type = todo.next();
            if project_type.is_some() {
                tail += Into::<&str>::into(project_type.unwrap());
                tail += "\n";
            }
        }
        if todo.next().is_some() {
            tail += "[...]\n";
        }
        write!(f, "{}", tail)
    }
}

#[cfg(test)]
mod test {
    use crate::record::ToDo;
    use rtdb::projects::ProjectType;

    #[test]
    fn test_next() {
        let mut todo = ToDo::from("00000000000000000000");
        assert_eq!(todo.next().unwrap(), ProjectType::Today);
        assert_eq!(todo, "10000000000000000000".into());
        assert_eq!(todo.next().unwrap(), ProjectType::En);
        assert_eq!(todo, "11000000000000000000".into());

        let mut todo = ToDo::from("11111111111111111100");
        assert_eq!(todo.next().unwrap(), ProjectType::En);
        assert_eq!(todo, "11111111111111111110".into());
        assert_eq!(todo.next().unwrap(), ProjectType::Tired);
        assert_eq!(todo, "11111111111111111111".into());

        let mut todo = ToDo::from("11101111111111111100");
        assert_eq!(todo.next().unwrap(), ProjectType::Today);
    }

    #[test]
    fn test_select_type() {
        let mut todo = ToDo::from("00000000000000000000");
        todo.select_type(ProjectType::En);
        assert_eq!(todo, "01000000000000000000".into());
        todo.select_type(ProjectType::Today);
        assert_eq!(todo, "11000000000000000000".into());
        let mut todo = ToDo::from("11010010100100100100");
        todo.select_type(ProjectType::Today);
        todo.select_type(ProjectType::Today);
        assert_eq!(todo, "11010010100100100100".into());
        let mut todo = ToDo::from("00100010000000000000");
        todo.select_type(ProjectType::FocusAnotherThing);
        assert_eq!(todo, "00100001000000000000".into());
    }
}
