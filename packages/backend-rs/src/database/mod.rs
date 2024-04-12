pub mod error;

use crate::config::server::SERVER_CONFIG;
use error::Error;
use sea_orm::{Database, DbConn};

static DB_CONN: once_cell::sync::OnceCell<DbConn> = once_cell::sync::OnceCell::new();

async fn init_database() -> Result<&'static DbConn, Error> {
    let database_uri = format!(
        "postgres://{}:{}@{}:{}/{}",
        SERVER_CONFIG.db.user,
        urlencoding::encode(&SERVER_CONFIG.db.pass),
        SERVER_CONFIG.db.host,
        SERVER_CONFIG.db.port,
        SERVER_CONFIG.db.db,
    );
    let conn = Database::connect(database_uri).await?;
    Ok(DB_CONN.get_or_init(move || conn))
}

pub async fn db_conn() -> Result<&'static DbConn, Error> {
    match DB_CONN.get() {
        Some(conn) => Ok(conn),
        None => init_database().await,
    }
}

#[cfg(test)]
mod unit_test {
    use super::db_conn;

    #[tokio::test]
    async fn connect_test() {
        assert!(db_conn().await.is_ok());
    }
}
