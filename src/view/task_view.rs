use crate::tasks::{TaskStatus, TaskType};
use crate::Task;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ListView {
    pub id: i32,
    pub name: String,
    pub r#type: TaskType,
    pub weight: i32,
    pub status: TaskStatus,
}

impl Display for ListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:30} {{id: {}, weight: {}, {:?}, {:?}}}",
            self.name, self.id, self.weight, self.r#type, self.status
        )
    }
}

impl From<Task> for ListView {
    fn from(t: Task) -> Self {
        ListView {
            id: t.id,
            name: t.name.clone(),
            r#type: t.r#type,
            weight: t.weight,
            status: t.status,
        }
    }
}
