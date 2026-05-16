// Public re-exports of third-party crates
pub use anyhow;
pub use chrono;
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
pub mod error;
pub mod filter_builders;
pub mod filters;
pub mod prelude;
pub mod read;
pub mod transaction;
pub mod write;

// Re-export main types
pub use aggregate::{AggregateBuilder, CountBuilder, GroupByBuilder};
pub use builder::{
    Create, CreateMany, Delete, DeleteMany, Executable, FilterBuilder, Filterable, FindFirst, FindFirstOrThrow,
    FindMany, FindUnique, FindUniqueOrThrow, ModelMarker, Query, RelCompatible, Selectable, SelectionField,
    SelectionRelField, Update, UpdateMany, Upsert,
};
pub use client::SaolaClient;
pub use error::{Error, Result};
pub use filter_builders::{
    BoolFilter, DateTimeFilter, EnumFilter, FloatFilter, IntFilter, RelationFilter, StringFilter,
};
pub use filters::{
    BoolField, DateTimeField, EnumField, Field, FloatField, IntField, RelationFilterOps, SelectField,
    StringField,
};
pub use read::ReadBuilder;
pub use transaction::{QueryExecutorProvider, Transaction, TransactionConfig};
pub use write::{
    CreateManyAndReturnBuilder, CreateManyBuilder, DeleteManyBuilder, UpdateManyAndReturnBuilder, UpdateManyBuilder,
    WriteBuilder,
};

pub mod macros;

/// Marker type for relations that are not loaded.
/// Implements Deserialize by ignoring any value, and Serialize as unit.
#[derive(Debug, Clone, Copy, Default)]
pub struct Unloaded;

impl<'de> serde::Deserialize<'de> for Unloaded {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = serde::de::IgnoredAny::deserialize(deserializer)?;
        Ok(Unloaded)
    }
}

impl serde::Serialize for Unloaded {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_unit()
    }
}

impl crate::builder::FromResponseIr for Unloaded {
    fn from_ir(_item: query_core::response_ir::Item) -> crate::Result<Self> {
        Ok(Unloaded)
    }
}

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
// (Removed anyhow::Result in favor of custom Result in error.rs)

// Re-export commonly used types from query_core
pub use query_core::{ArgumentValue, Operation, Selection};

// Macros
