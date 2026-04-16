/// Base traits and types for all query builders
/// This module provides the foundation for type-safe, composable query building
use query_core::{ArgumentValue, Operation, Selection, protocol::EngineProtocol};

/// Trait for any object that can execute a query and return typed results
pub trait Executable: Sized {
    /// Execute the query and deserialize result to type T
    #[allow(async_fn_in_trait)]
    async fn exec<T: serde::de::DeserializeOwned>(self, client: &crate::client::PrismaClient) -> crate::Result<T>;
}

/// Trait for builders that support where clauses (filtering)
pub trait Filterable {
    /// Add a filter argument to the query
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue);
}

/// Trait for builders that support selection (field picking)
pub trait Selectable {
    /// Add a field to the selection
    fn add_nested_selection(&mut self, selection: Selection);
}

/// Backward compatibility trait - alias for FilterBuilder used by generated macros
pub trait FilterBuilder: Sized {
    fn add_arg(&mut self, name: String, value: ArgumentValue);
    fn build(self) -> crate::IndexMap<String, ArgumentValue>;
}

/// Base builder state - common to all operations
pub struct BuilderState {
    pub model_name: String,
    pub selection: Selection,
    pub arguments: crate::IndexMap<String, ArgumentValue>,
    pub default_selections: Vec<String>,
}

impl BuilderState {
    /// Create a new builder state for a read operation
    pub fn read(model_name: String, operation: &str, default_selections: Vec<String>) -> Self {
        let name = format!("{}{}", operation, model_name);
        Self {
            model_name,
            selection: Selection::with_name(name),
            arguments: crate::IndexMap::new(),
            default_selections,
        }
    }

    /// Create a new builder state for a write operation
    pub fn write(model_name: String, operation: &str, default_selections: Vec<String>) -> Self {
        let name = format!("{}{}", operation, model_name);
        Self {
            model_name,
            selection: Selection::with_name(name),
            arguments: crate::IndexMap::new(),
            default_selections,
        }
    }

    /// Apply default selections if none were explicitly selected
    pub fn apply_defaults(&mut self) {
        if self.selection.nested_selections().is_empty() {
            for field in &self.default_selections {
                self.selection
                    .push_nested_selection(Selection::with_name(field.clone()));
            }
        }
    }

    /// Consume this state and create an operation
    pub fn into_operation(mut self, is_write: bool) -> Operation {
        self.apply_defaults();
        for (k, v) in std::mem::take(&mut self.arguments) {
            self.selection.push_argument(k, v);
        }
        if is_write {
            Operation::Write(self.selection)
        } else {
            Operation::Read(self.selection)
        }
    }
}

pub async fn execute_raw(
    operation: Operation,
    client: &crate::client::PrismaClient,
) -> crate::Result<serde_json::Value> {
    let response = client
        .executor()
        .execute(None, operation, client.query_schema(), None, EngineProtocol::Json)
        .await?;

    Ok(serde_json::to_value(&response.data)?)
}

/// Common implementation of execution logic across all builders
pub async fn execute<T: serde::de::DeserializeOwned>(
    operation: Operation,
    client: &crate::client::PrismaClient,
) -> crate::Result<T> {
    let json = execute_raw(operation, client).await?;
    Ok(serde_json::from_value(json)?)
}
