use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub md_link: Option<String>,
    pub r#type: ProjectType,
    pub weight: i32,
    pub status: ProjectStatus,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ProjectType {
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

impl Into<&str> for ProjectType {
    fn into(self) -> &'static str {
        match self {
            ProjectType::FocusAnotherThing => "Focus another thing",
            ProjectType::TakeABreak => "Take a break",
            ProjectType::Tired => "Tired",
            ProjectType::Today => "Today",
            ProjectType::Inbox => "Inbox",
            ProjectType::En => "En",
        }
    }
}

impl From<String> for ProjectType {
    fn from(value: String) -> Self {
        match &*value {
            "Inbox" => ProjectType::Inbox,
            "Today" => ProjectType::Today,
            "En" => ProjectType::En,
            "Focus another thing" => ProjectType::FocusAnotherThing,
            "Take a break" => ProjectType::TakeABreak,
            "Tired" => ProjectType::Tired,
            _ => panic!("invalid project type"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ProjectStatus {
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
