// Public re-exports of third-party crates
pub use anyhow;
pub use indexmap::IndexMap;
pub use query_core;
pub use query_structure;
pub use schema;
pub use serde;
pub use serde_json;

// Internal modules - exported for use by macros
pub mod aggregate;
pub mod builder;
pub mod client;
pub mod filter_builders;
pub mod filters;
pub mod prelude;
pub mod read;
pub mod transaction;
pub mod write;

// Re-export main types
pub use aggregate::{AggregateBuilder, CountBuilder, GroupByBuilder};
pub use builder::{Executable, FilterBuilder, Filterable, Selectable};
pub use client::PrismaClient;
pub use filter_builders::{
    BoolFilter, DateTimeFilter, EnumFilter, FloatFilter, IntFilter, RelationFilter, StringFilter,
};
pub use filters::{
    BoolFieldOps, DateTimeFieldOps, EnumFieldOps, FloatFieldOps, IntFieldOps, RelationFilterOps, StringFieldOps,
};
pub use read::ReadBuilder;
pub use transaction::{QueryExecutorProvider, Transaction, TransactionConfig};
pub use write::{
    CreateManyAndReturnBuilder, CreateManyBuilder, DeleteManyBuilder, UpdateManyAndReturnBuilder, UpdateManyBuilder,
    WriteBuilder,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl From<SortOrder> for ArgumentValue {
    fn from(order: SortOrder) -> Self {
        ArgumentValue::Scalar(match order {
            SortOrder::Asc => query_structure::PrismaValue::Enum("asc".to_string()),
            SortOrder::Desc => query_structure::PrismaValue::Enum("desc".to_string()),
        })
    }
}

// Standard Result type using anyhow::Error
pub type Result<T> = anyhow::Result<T>;

// Re-export commonly used types from query_core
pub use query_core::{ArgumentValue, Operation, Selection};

// Macros
