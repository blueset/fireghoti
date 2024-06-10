//! PostgreSQL interface

use crate::config::CONFIG;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DbConn, DbErr};
use std::time::Duration;
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
        .sqlx_slow_statements_logging_settings(LevelFilter::Warn, Duration::from_secs(3))
        .to_owned();

    tracing::info!("initializing connection");

    let conn = Database::connect(option).await?;
    Ok(DB_CONN.get_or_init(move || conn))
}

/// Returns an async PostgreSQL connection that can be used with [sea_orm] utilities.
pub async fn get_conn() -> Result<&'static DbConn, DbErr> {
    match DB_CONN.get() {
        Some(conn) => Ok(conn),
        None => init_conn().await,
    }
}

#[cfg(test)]
mod unit_test {
    use super::get_conn;

    #[tokio::test]
    async fn connect_sequential() {
        get_conn().await.unwrap();
        get_conn().await.unwrap();
        get_conn().await.unwrap();
        get_conn().await.unwrap();
        get_conn().await.unwrap();
    }

    #[tokio::test]
    async fn connect_concurrent() {
        let [c1, c2, c3, c4, c5] = [get_conn(), get_conn(), get_conn(), get_conn(), get_conn()];
        let _ = tokio::try_join!(c1, c2, c3, c4, c5).unwrap();
    }

    #[tokio::test]
    async fn connect_spawn() {
        let mut tasks = Vec::new();

        for _ in 0..5 {
            tasks.push(tokio::spawn(get_conn()));
        }
        for task in tasks {
            task.await.unwrap().unwrap();
        }
    }
}
