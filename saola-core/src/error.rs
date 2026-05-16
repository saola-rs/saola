pub use query_core::CoreError;
use thiserror::Error;
pub use user_facing_errors::KnownError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabaseError(#[from] CoreError),

    #[error("Prisma error {}: {}", .0.error_code, .0.message)]
    PrismaError(KnownError),

    #[error("Record not found: {model_name}.{argument_name} = {argument_value}")]
    RecordNotFound {
        model_name: String,
        argument_name: String,
        argument_value: String,
    },

    #[error("Unique constraint failed on the {constraint}")]
    UniqueConstraintViolation { constraint: String },

    #[error("Foreign key constraint violated on the {constraint}")]
    ForeignKeyConstraintViolation { constraint: String },

    #[error("Constraint failed on the database: {database_error}")]
    DatabaseConstraintViolation { database_error: String },

    #[error("Null constraint violation on the {constraint}")]
    NullConstraintViolation { constraint: String },

    #[error("Value too long for column: {column_name}")]
    ValueTooLong { column_name: String },

    #[error("Inconsistent column data: {message}")]
    InconsistentColumnData { message: String },

    #[error("Data query invalid: {message}")]
    DataQueryInvalid { message: String },

    #[error("Query parameter limit exceeded: {message}")]
    QueryParameterLimitExceeded { message: String },

    #[error("Raw query error: {message}")]
    RawQueryError { message: String },

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
            match known.error_code.as_ref() {
                "P2001" => {
                    let model_name = known.meta["model_name"].as_str().unwrap_or_default().to_string();
                    let argument_name = known.meta["argument_name"].as_str().unwrap_or_default().to_string();
                    let argument_value = known.meta["argument_value"].as_str().unwrap_or_default().to_string();
                    Error::RecordNotFound {
                        model_name,
                        argument_name,
                        argument_value,
                    }
                }
                "P2025" => {
                    // P2025 is also often "Record not found" in modern Prisma
                    let cause = known.meta["cause"].as_str().unwrap_or("Record not found").to_string();
                    Error::RecordNotFound {
                        model_name: "Unknown".to_string(),
                        argument_name: "cause".to_string(),
                        argument_value: cause,
                    }
                }
                "P2002" => {
                    let constraint = known.meta["target"].to_string();
                    Error::UniqueConstraintViolation { constraint }
                }
                "P2003" => {
                    let constraint = known.meta["field_name"]
                        .as_str()
                        .or(known.meta["target"].as_str())
                        .unwrap_or_default()
                        .to_string();
                    Error::ForeignKeyConstraintViolation { constraint }
                }
                "P2004" => {
                    let database_error = known.meta["database_error"].as_str().unwrap_or_default().to_string();
                    Error::DatabaseConstraintViolation { database_error }
                }
                "P2011" => {
                    let constraint = known.meta["constraint"].as_str().unwrap_or_default().to_string();
                    Error::NullConstraintViolation { constraint }
                }
                "P2000" => {
                    let column_name = known.meta["column_name"].as_str().unwrap_or_default().to_string();
                    Error::ValueTooLong { column_name }
                }
                "P2023" => {
                    let message = known.meta["message"].as_str().unwrap_or_default().to_string();
                    Error::InconsistentColumnData { message }
                }
                "P2016" => {
                    let message = known.meta["details"].as_str().unwrap_or_default().to_string();
                    Error::DataQueryInvalid { message }
                }
                "P2029" => {
                    let message = known.meta["message"].as_str().unwrap_or_default().to_string();
                    Error::QueryParameterLimitExceeded { message }
                }
                "P2010" => {
                    let message = known.meta["message"].as_str().unwrap_or_default().to_string();
                    Error::RawQueryError { message }
                }
                _ => Error::PrismaError(known.clone()),
            }
        } else {
            Error::UnknownPrismaError(user_error.message().to_string())
        }
    }
}
