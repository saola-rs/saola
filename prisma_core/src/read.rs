/// Read/Find query builders - findMany, findUnique, findFirst, etc.
use crate::builder::{BuilderState, Executable, Filterable, Selectable, execute};
use query_core::{ArgumentValue, Selection};

/// Generic Read builder - base for all find operations
pub struct ReadBuilder {
    pub state: BuilderState,
    is_write: bool,
}

impl ReadBuilder {
    /// Create a new read builder for the given operation
    pub fn new(model_name: String, operation: &str, default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::read(model_name, operation, default_selections),
            is_write: false,
        }
    }

    /// Create a findMany operation
    pub fn find_many(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "findMany", default_selections)
    }

    /// Create a findUnique operation
    pub fn find_unique(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "findUnique", default_selections)
    }

    /// Create a findFirst operation
    pub fn find_first(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "findFirst", default_selections)
    }

    /// Create a findFirstOrThrow operation
    pub fn find_first_or_throw(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "findFirstOrThrow", default_selections)
    }

    /// Create a findUniqueOrThrow operation
    pub fn find_unique_or_throw(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "findUniqueOrThrow", default_selections)
    }

    /// Builder method - do not use directly, use through generated builders with where_clause closures
    pub fn _add_where_from_map(&mut self, where_map: indexmap::IndexMap<String, ArgumentValue>) -> &mut Self {
        self.state.selection.push_argument("where".to_string(), ArgumentValue::Object(where_map));
        self
    }

    /// Builder method - do not use directly, use through generated builders with select closures
    pub fn _add_select_fields(&mut self, fields: Vec<String>) -> &mut Self {
        self.state.selection.clear_nested_selections();
        for field in fields {
            self.state.selection.push_nested_selection(Selection::with_name(field));
        }
        self
    }

    /// Builder method - do not use directly, use through generated builders with include closures
    pub fn _add_includes(&mut self, selections: Vec<Selection>) -> &mut Self {
        for sel in selections {
            self.state.selection.push_nested_selection(sel);
        }
        self
    }
}

impl Filterable for ReadBuilder {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.selection.push_argument(name, value);
    }
}

impl Selectable for ReadBuilder {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
}

impl Executable for ReadBuilder {
    async fn exec<T: serde::de::DeserializeOwned>(self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        let operation = self.state.into_operation(false);
        execute(operation, client).await
    }
}
