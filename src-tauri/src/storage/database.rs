use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection failed: {0}")]
    ConnectionError(String),
    #[error("Query failed: {0}")]
    QueryError(String),
}

pub fn init_database() -> Result<(), DatabaseError> {
    Ok(())
}
