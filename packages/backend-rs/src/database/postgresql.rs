//! PostgreSQL interface

use crate::config::CONFIG;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DbConn, DbErr};
use tracing::log::LevelFilter;

static DB_CONN: OnceCell<DbConn> = OnceCell::new();

async fn init_conn() -> Result<&'static DbConn, DbErr> {
    let database_uri = format!(
        "postgres://{}:{}@{}:{}/{}",
        CONFIG.db.user,
        urlencoding::encode(&CONFIG.db.pass),
        CONFIG.db.host,
        CONFIG.db.port,
        CONFIG.db.db,
    );
    let option: ConnectOptions = ConnectOptions::new(database_uri)
        .sqlx_logging_level(LevelFilter::Trace)
        .to_owned();

    tracing::info!("initializing connection");

    let conn = Database::connect(option).await?;
    Ok(DB_CONN.get_or_init(move || conn))
}

/// Returns an async PostgreSQL connection that can be used with [sea_orm] utilities.
pub async fn db_conn() -> Result<&'static DbConn, DbErr> {
    match DB_CONN.get() {
        Some(conn) => Ok(conn),
        None => init_conn().await,
    }
}

#[cfg(test)]
mod unit_test {
    use super::db_conn;

    #[tokio::test]
    async fn connect() {
        assert!(db_conn().await.is_ok());
        assert!(db_conn().await.is_ok());
    }
}
