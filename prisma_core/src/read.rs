/// Read/Find query builders - findMany, findUnique, findFirst, etc.
use crate::builder::{BuilderState, Executable, Filterable, Selectable, execute};
use query_core::{ArgumentValue, Selection};

/// Generic Read builder - base for all find operations
pub struct ReadBuilder {
    state: BuilderState,
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
    async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        let operation = self.state.into_operation(false);
        execute(operation, client).await
    }
}

// Public type aliases for convenience and backward compatibility
pub type FindFirstBuilder = ReadBuilder;
pub type FindFirstOrThrowBuilder = ReadBuilder;
pub type FindUniqueOrThrowBuilder = ReadBuilder;
