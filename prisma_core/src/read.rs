use crate::builder::{BuilderState, Executable, Filterable, Selectable, execute};
use query_core::{ArgumentValue, Selection};
use std::marker::PhantomData;

/// Generic Read builder - base for all find operations
pub struct ReadBuilder<T> {
    pub state: BuilderState,
    _phantom: PhantomData<T>,
}

impl<T> ReadBuilder<T> {
    /// Create a new read builder for the given operation
    pub fn new(model_name: String, operation: &str, default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::read(model_name, operation, default_selections),
            _phantom: PhantomData,
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

    /// Create a findUniqueOrThrow operation
    pub fn find_unique_or_throw(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "findUniqueOrThrow", default_selections)
    }

    /// Create a findFirstOrThrow operation
    pub fn find_first_or_throw(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "findFirstOrThrow", default_selections)
    }

    /// Transition this builder to return a different type
    pub fn with_type<U>(self) -> ReadBuilder<U> {
        ReadBuilder {
            state: self.state,
            _phantom: PhantomData,
        }
    }
}

impl<T> Filterable for ReadBuilder<T> {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.selection.push_argument(name, value);
    }
}

impl<T> Selectable for ReadBuilder<T> {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
}

impl<T: serde::de::DeserializeOwned + Send + Sync> Executable for ReadBuilder<T> {
    async fn exec<Ret: serde::de::DeserializeOwned>(self, client: &crate::client::PrismaClient) -> crate::Result<Ret> {
        let operation = self.state.into_operation(false);
        execute(operation, client).await
    }
}

impl<T: serde::de::DeserializeOwned + Send + Sync> ReadBuilder<T> {
    pub async fn exec_inferred(self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        let operation = self.state.into_operation(false);
        execute(operation, client).await
    }
}
