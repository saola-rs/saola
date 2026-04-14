// Public re-exports of third-party crates
pub use anyhow;
pub use indexmap::IndexMap;
pub use query_core;
pub use query_structure;
pub use schema;
pub use serde;
pub use serde_json;

// Internal modules - exported for use by macros
pub mod builder;
pub mod client;
pub mod read;
pub mod write;
pub mod aggregate;
pub mod filters;
pub mod filter_builders;
pub mod prelude;

// Re-export main types
pub use builder::{Executable, Filterable, Selectable, FilterBuilder};
pub use client::PrismaClient;
pub use read::ReadBuilder;
pub use write::WriteBuilder;
pub use aggregate::{CountBuilder, AggregateBuilder, GroupByBuilder};
pub use filters::{StringFieldOps, IntFieldOps, BoolFieldOps, EnumFieldOps, FloatFieldOps, DateTimeFieldOps, RelationFilterOps};
pub use filter_builders::{StringFilter, IntFilter, BoolFilter, EnumFilter, FloatFilter, DateTimeFilter};

// Standard Result type using anyhow::Error
pub type Result<T> = anyhow::Result<T>;

// Re-export commonly used types from query_core
pub use query_core::{Operation, ArgumentValue, Selection};
