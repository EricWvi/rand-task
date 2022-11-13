use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub md_link: Option<String>,
    pub r#type: TaskType,
    pub weight: i32,
    pub status: TaskStatus,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum TaskType {
    #[sea_orm(num_value = 0)]
    FocusAnotherThing,
    #[sea_orm(num_value = 1)]
    TakeABreak,
    #[sea_orm(num_value = 2)]
    Tired,
    #[sea_orm(num_value = 3)]
    Today,
    #[sea_orm(num_value = 4)]
    Inbox,
    #[sea_orm(num_value = 5)]
    En,
}

impl Into<&str> for TaskType {
    fn into(self) -> &'static str {
        match self {
            TaskType::FocusAnotherThing => "Focus another thing",
            TaskType::TakeABreak => "Take a break",
            TaskType::Tired => "Tired",
            TaskType::Today => "Today",
            TaskType::Inbox => "Inbox",
            TaskType::En => "En",
        }
    }
}

impl From<String> for TaskType {
    fn from(value: String) -> Self {
        match &*value {
            "Inbox" => TaskType::Inbox,
            "Today" => TaskType::Today,
            "En" => TaskType::En,
            "Focus another thing" => TaskType::FocusAnotherThing,
            "Take a break" => TaskType::TakeABreak,
            "Tired" => TaskType::Tired,
            _ => panic!("invalid task type"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum TaskStatus {
    #[sea_orm(num_value = 0)]
    Pending,
    #[sea_orm(num_value = 1)]
    Scheduled,
    #[sea_orm(num_value = 2)]
    Unfinished,
    #[sea_orm(num_value = 3)]
    Completed,
    #[sea_orm(num_value = 4)]
    Discarded,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
