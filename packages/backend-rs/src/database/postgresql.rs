use crate::config::server::CONFIG;
use sea_orm::{Database, DbConn, DbErr};

static DB_CONN: once_cell::sync::OnceCell<DbConn> = once_cell::sync::OnceCell::new();

async fn init_database() -> Result<&'static DbConn, DbErr> {
    let database_uri = format!(
        "postgres://{}:{}@{}:{}/{}",
        CONFIG.db.user,
        urlencoding::encode(&CONFIG.db.pass),
        CONFIG.db.host,
        CONFIG.db.port,
        CONFIG.db.db,
    );
    let conn = Database::connect(database_uri).await?;
    Ok(DB_CONN.get_or_init(move || conn))
}

pub async fn db_conn() -> Result<&'static DbConn, DbErr> {
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
