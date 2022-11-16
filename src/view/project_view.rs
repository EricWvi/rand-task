use crate::projects::{ProjectStatus, ProjectType};
use crate::Project;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ListView {
    pub id: i32,
    pub name: String,
    pub r#type: ProjectType,
    pub weight: i32,
    pub status: ProjectStatus,
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

impl From<Project> for ListView {
    fn from(t: Project) -> Self {
        ListView {
            id: t.id,
            name: t.name.clone(),
            r#type: t.r#type,
            weight: t.weight,
            status: t.status,
        }
    }
}
