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

// Re-export main types
pub use builder::{Executable, Filterable, Selectable};
pub use client::PrismaClient;
pub use read::{ReadBuilder, FindManyBuilder, FindUniqueBuilder, FindFirstBuilder, FindFirstOrThrowBuilder, FindUniqueOrThrowBuilder};
pub use write::{WriteBuilder, CreateBuilder, UpdateBuilder, DeleteBuilder, CreateManyBuilder, UpdateManyBuilder, DeleteManyBuilder, UpsertBuilder};
pub use aggregate::{CountBuilder, AggregateBuilder, GroupByBuilder};

// Standard Result type using anyhow::Error
pub type Result<T> = anyhow::Result<T>;

// Re-export commonly used types from query_core
pub use query_core::{Operation, ArgumentValue, Selection};
