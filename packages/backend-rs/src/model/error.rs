#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Failed to parse string: {0}")]
    ParseError(#[from] parse_display::ParseError),
    #[error("Database error: {0}")]
    DbError(#[from] sea_orm::DbErr),
    #[error("Requested entity not found")]
    NotFound,
}
