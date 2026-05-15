pub use query_core::CoreError;
use thiserror::Error;
pub use user_facing_errors::KnownError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(#[from] CoreError),

    #[error("Prisma error {}: {}", .0.error_code, .0.message)]
    PrismaError(KnownError),

    #[error("Prisma unknown error: {0}")]
    UnknownPrismaError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Query validation error: {0}")]
    ValidationError(String),

    #[error("Query builder error: {0}")]
    QueryBuilderError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn from_core(err: CoreError) -> Self {
        let user_error = user_facing_errors::Error::from(err);
        if let Some(known) = user_error.as_known() {
            Error::PrismaError(known.clone())
        } else {
            Error::UnknownPrismaError(user_error.message().to_string())
        }
    }
}
