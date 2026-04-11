/// Zero-cost DSL types for SELECT and WHERE builders
/// All field names are &'static str (no allocations)
/// All data structures use compile-time constants where possible

pub mod builders;

/// Tracks selected fields using &'static str for field names
/// This is built at compile-time, so no allocations
#[derive(Debug, Clone, Copy)]
pub struct SelectionTracker {
    /// Sorted array of selected field names (compile-time determined)
    /// We use a fixed-size array with Option to avoid Vec allocation
    pub fields: &'static [&'static str],

    /// Nested selections for relations (compile-time determined)
    pub nested: &'static [(&'static str, SelectionTracker)],
}

impl SelectionTracker {
    /// Create a selection tracker from compile-time field arrays
    pub const fn new(
        fields: &'static [&'static str],
        nested: &'static [(&'static str, SelectionTracker)],
    ) -> Self {
        SelectionTracker { fields, nested }
    }

    /// Check if a field is selected
    #[inline]
    pub fn is_selected(&self, field: &str) -> bool {
        self.fields.iter().any(|f| *f == field)
    }

    /// Get nested selection for a relation
    #[inline]
    pub fn get_nested(&self, relation: &str) -> Option<SelectionTracker> {
        self.nested
            .iter()
            .find(|(r, _)| *r == relation)
            .map(|(_, sel)| *sel)
    }
}

// ============================================================

/// WHERE condition operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhereOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
    Contains,
    NotContains,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    Between,
    StartsWith,
    EndsWith,
}

/// Filter value - using static references where possible
#[derive(Debug, Clone, Copy)]
pub enum FilterValue {
    String(&'static str),
    Int(i64),
    Float(f64),
    Bool(bool),
    StringList(&'static [&'static str]),
    IntList(&'static [i64]),
    Null,
}

/// A single WHERE condition (zero-allocation)
#[derive(Debug, Clone, Copy)]
pub struct WhereCondition {
    pub field: &'static str,
    pub operator: WhereOperator,
    pub value: FilterValue,
}

impl WhereCondition {
    /// Create a new condition with static references
    pub const fn new(field: &'static str, operator: WhereOperator, value: FilterValue) -> Self {
        WhereCondition {
            field,
            operator,
            value,
        }
    }
}

/// Tracks WHERE conditions - compile-time determined
#[derive(Debug, Clone, Copy)]
pub struct FilterTracker {
    /// Top-level conditions (AND)
    pub conditions: &'static [WhereCondition],

    /// OR groups
    pub or_groups: &'static [&'static [WhereCondition]],

    /// Nested filters for relations
    /// Using BTreeMap for O(log n) lookup, but with static str keys (no allocations)
    pub relation_filters: &'static [(&'static str, FilterTracker)],
}

impl FilterTracker {
    /// Create a filter tracker from compile-time arrays
    pub const fn new(
        conditions: &'static [WhereCondition],
        or_groups: &'static [&'static [WhereCondition]],
        relation_filters: &'static [(&'static str, FilterTracker)],
    ) -> Self {
        FilterTracker {
            conditions,
            or_groups,
            relation_filters,
        }
    }

    /// Get nested filter for a relation
    #[inline]
    pub fn relation_filter(&self, relation: &str) -> Option<FilterTracker> {
        self.relation_filters
            .iter()
            .find(|(r, _)| *r == relation)
            .map(|(_, filter)| *filter)
    }
}

// ============================================================
// Builder marker types - these are zero-cost, just type tags

/// Marker type for SelectBuilder state tracking
/// These are used to encode selection state at compile-time
pub struct SelectBuilderMarker;

/// Marker type for WhereBuilder state tracking
pub struct WhereBuilderMarker;

// ============================================================
// Compile-time constants for common selections and filters

/// Empty selection (no fields selected yet)
pub const EMPTY_SELECTION: SelectionTracker = SelectionTracker {
    fields: &[],
    nested: &[],
};

/// Empty filter (no conditions)
pub const EMPTY_FILTER: FilterTracker = FilterTracker {
    conditions: &[],
    or_groups: &[],
    relation_filters: &[],
};

/// Helper macro to create compile-time field arrays
/// Usage: fields!["id", "name", "email"]
#[macro_export]
macro_rules! fields {
    ($($field:expr),* $(,)?) => {
        &[$($field),*] as &[&'static str]
    };
}

/// Helper macro to create compile-time conditions
/// Usage: conditions![eq("id", "123"), contains("email", "@gmail")]
#[macro_export]
macro_rules! conditions {
    ($($cond:expr),* $(,)?) => {
        &[$($cond),*] as &[WhereCondition]
    };
}

// ============================================================
// Const helper functions for building conditions

pub const fn eq(field: &'static str, value: &'static str) -> WhereCondition {
    WhereCondition {
        field,
        operator: WhereOperator::Equals,
        value: FilterValue::String(value),
    }
}

pub const fn int_eq(field: &'static str, value: i64) -> WhereCondition {
    WhereCondition {
        field,
        operator: WhereOperator::Equals,
        value: FilterValue::Int(value),
    }
}

pub const fn contains(field: &'static str, value: &'static str) -> WhereCondition {
    WhereCondition {
        field,
        operator: WhereOperator::Contains,
        value: FilterValue::String(value),
    }
}

pub const fn is_null(field: &'static str) -> WhereCondition {
    WhereCondition {
        field,
        operator: WhereOperator::IsNull,
        value: FilterValue::Null,
    }
}

pub const fn in_list(field: &'static str, values: &'static [&'static str]) -> WhereCondition {
    WhereCondition {
        field,
        operator: WhereOperator::In,
        value: FilterValue::StringList(values),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_tracker_const() {
        const SELECTION: SelectionTracker = SelectionTracker::new(&["id", "name", "email"], &[]);

        assert!(SELECTION.is_selected("id"));
        assert!(SELECTION.is_selected("name"));
        assert!(!SELECTION.is_selected("password"));
    }

    #[test]
    fn test_nested_selection_const() {
        const POST_SELECTION: SelectionTracker = SelectionTracker::new(&["id", "title"], &[]);
        const USER_SELECTION: SelectionTracker =
            SelectionTracker::new(&["id", "name"], &[("posts", POST_SELECTION)]);

        assert!(USER_SELECTION.is_selected("id"));
        let nested = USER_SELECTION.get_nested("posts");
        assert!(nested.is_some());
        assert!(nested.unwrap().is_selected("title"));
    }

    #[test]
    fn test_condition_const() {
        const COND: WhereCondition = eq("email", "test@example.com");

        assert_eq!(COND.field, "email");
        assert_eq!(COND.operator, WhereOperator::Equals);
    }

    #[test]
    fn test_filter_tracker_const() {
        const CONDITIONS: &[WhereCondition] = &[eq("status", "active")];
        const FILTER: FilterTracker = FilterTracker::new(CONDITIONS, &[], &[]);

        assert_eq!(FILTER.conditions.len(), 1);
    }

    #[test]
    fn test_filter_by_relation() {
        const POST_FILTER: FilterTracker = FilterTracker::new(&[eq("published", "true")], &[], &[]);
        const USER_FILTER: FilterTracker =
            FilterTracker::new(&[eq("id", "123")], &[], &[("posts", POST_FILTER)]);

        let nested = USER_FILTER.relation_filter("posts");
        assert!(nested.is_some());
    }
}
