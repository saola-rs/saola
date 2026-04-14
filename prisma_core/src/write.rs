/// Write query builders - create, update, delete operations
use crate::builder::{BuilderState, Executable, Filterable, Selectable, execute};
use query_core::{ArgumentValue, Selection};

/// Generic Write builder - base for all create/update/delete operations
pub struct WriteBuilder {
    pub state: BuilderState,
    is_write: bool,
}

impl WriteBuilder {
    /// Create a new write builder for the given operation
    pub fn new(model_name: String, operation: &str, default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::write(model_name, operation, default_selections),
            is_write: true,
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

    /// Create a createMany operation
    pub fn create_many(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "createMany", default_selections)
    }

    /// Create an updateMany operation
    pub fn update_many(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "updateMany", default_selections)
    }

    /// Create a deleteMany operation
    pub fn delete_many(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "deleteMany", default_selections)
    }

    /// Create an upsert operation
    pub fn upsert(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "upsert", default_selections)
    }
}

impl Filterable for WriteBuilder {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.selection.push_argument(name, value);
    }
}

impl Selectable for WriteBuilder {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
}

impl Executable for WriteBuilder {
    async fn exec<T: serde::de::DeserializeOwned>(self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        let operation = self.state.into_operation(true);
        execute(operation, client).await
    }
}
