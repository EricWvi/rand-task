//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use super::tasks::TaskType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub date: DateTime,
    pub schedule_time: i32,
    pub actual_time: i32,
    pub use_rate: String,
    pub task_type: TaskType,
    pub task_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
