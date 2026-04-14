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
pub mod legacy_read;
pub mod legacy_write;

// Re-export main types
pub use builder::{Executable, Filterable, Selectable, FilterBuilder};
pub use client::PrismaClient;
pub use read::{ReadBuilder, FindFirstBuilder, FindFirstOrThrowBuilder, FindUniqueOrThrowBuilder};
pub use write::{WriteBuilder, CreateManyBuilder, UpdateManyBuilder, DeleteManyBuilder, UpsertBuilder};
pub use aggregate::{CountBuilder, AggregateBuilder, GroupByBuilder};

// Re-export legacy builders for backward compatibility with generated macros
pub use legacy_read::{FindManyBuilder, FindUniqueBuilder};
pub use legacy_write::{CreateBuilder, UpdateBuilder, DeleteBuilder};

// Standard Result type using anyhow::Error
pub type Result<T> = anyhow::Result<T>;

// Re-export commonly used types from query_core
pub use query_core::{Operation, ArgumentValue, Selection};
