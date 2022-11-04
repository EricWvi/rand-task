pub mod config;
mod dao;
mod entity;
pub mod util;
mod view;

pub use dao::*;
pub use entity::*;
pub use view::*;

use sea_orm::{Database, DatabaseConnection, DbErr};
use tokio::sync::OnceCell;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

#[inline]
pub fn db() -> &'static DatabaseConnection {
    DB.get().unwrap()
}

#[cfg(not(test))]
pub async fn init() -> Result<&'static DatabaseConnection, DbErr> {
    config::init();

    DB.set(Database::connect(config::db_url()).await?)
        .expect("db is not initialized");
    Ok(DB.get().unwrap())
}

#[cfg(test)]
pub async fn init() -> DatabaseConnection {
    config::init();

    Database::connect(config::db_url())
        .await
        .expect("can not connect to db")
}
