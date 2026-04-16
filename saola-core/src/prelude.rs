//! Prisma prelude module
//!
//! This module re-exports the most common types and traits needed for building queries.
//! Import this with `use saola_core::prelude::*;` to make filter operators available
//! in your code closures.

// Re-export all filter operator traits
pub use crate::filters::{
    BoolFieldOps, DateTimeFieldOps, EnumFieldOps, FloatFieldOps, IntFieldOps, RelationFilterOps, StringFieldOps,
};

// Re-export filter builder types
pub use crate::filter_builders::{BoolFilter, DateTimeFilter, EnumFilter, FloatFilter, IntFilter, StringFilter};

// Re-export core builder types
pub use crate::builder::{Executable, FilterBuilder, Filterable, Selectable};

// Re-export common operation types
pub use crate::{AggregateBuilder, CountBuilder, GroupByBuilder, ReadBuilder, WriteBuilder, SortOrder};

// Re-export result type
pub use crate::Result;

pub use crate::transaction::IsolationLevel;
