extern crate core;

pub mod config;
mod dao;
mod entity;
pub mod util;
mod view;

pub use dao::*;
pub use entity::*;
pub use view::*;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tokio::sync::OnceCell;
use tracing::log;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

#[inline]
pub fn db() -> &'static DatabaseConnection {
    DB.get().unwrap()
}

#[cfg(not(test))]
pub async fn init() -> Result<&'static DatabaseConnection, DbErr> {
    config::init();

    let mut opt = ConnectOptions::new(config::db_url().clone());
    opt.sqlx_logging(false) // Disabling SQLx log
        .sqlx_logging_level(log::LevelFilter::Info); // Setting SQLx log level

    DB.set(Database::connect(opt).await?)
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
