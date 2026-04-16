use crate::builder::{BuilderState, Executable, Filterable, Selectable, execute};
use query_core::{ArgumentValue, Selection};
use std::marker::PhantomData;

/// Generic Write builder - base for all create/update/delete operations
pub struct WriteBuilder<T> {
    pub state: BuilderState,
    _phantom: PhantomData<T>,
}

impl<T> WriteBuilder<T> {
    /// Create a new write builder for the given operation
    pub fn new(model_name: String, operation: &str, default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::write(model_name, operation, default_selections),
            _phantom: PhantomData,
        }
    }

    /// Create a create operation (createOne{Model})
    pub fn create(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "createOne", default_selections)
    }

    /// Create an update operation (updateOne{Model})
    pub fn update(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "updateOne", default_selections)
    }

    /// Create a delete operation (deleteOne{Model})
    pub fn delete(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "deleteOne", default_selections)
    }

    /// Transition this builder to return a different type
    pub fn with_type<U>(self) -> WriteBuilder<U> {
        WriteBuilder {
            state: self.state,
            _phantom: PhantomData,
        }
    }
}

impl<T> Filterable for WriteBuilder<T> {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.arguments.insert(name, value);
    }
}

impl<T> Selectable for WriteBuilder<T> {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
}

impl<T: serde::de::DeserializeOwned + Send + Sync> Executable for WriteBuilder<T> {
    async fn exec<Ret: serde::de::DeserializeOwned>(self, client: &crate::client::PrismaClient) -> crate::Result<Ret> {
        let operation = self.state.into_operation(true);
        execute(operation, client).await
    }
}

impl<T: serde::de::DeserializeOwned + Send + Sync> WriteBuilder<T> {
    pub async fn exec_inferred(self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        let operation = self.state.into_operation(true);
        execute(operation, client).await
    }
}
