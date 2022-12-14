use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub file_link: Option<String>,
    pub project_id: i32,
    pub status: TaskStatus,
    pub seq: i32,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            id: 0,
            name: "".to_string(),
            file_link: None,
            project_id: 0,
            status: TaskStatus::Unfinished,
            seq: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum TaskStatus {
    #[sea_orm(num_value = 0)]
    Unfinished,
    #[sea_orm(num_value = 1)]
    Completed,
    #[sea_orm(num_value = 2)]
    Discarded,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
