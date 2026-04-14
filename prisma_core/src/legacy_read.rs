/// Legacy read builders - kept for backward compatibility with generated macros
/// These wrap the generic ReadBuilder for convenience

use crate::builder::{BuilderState, Executable, Filterable, Selectable, execute};
use query_core::{ArgumentValue, Selection};

/// Legacy FindMany builder - wraps ReadBuilder
pub struct FindManyBuilder {
    pub model_name: String,
    pub selection: Selection,
    pub default_selections: Vec<String>,
}

impl FindManyBuilder {
    pub fn new(model_name: String, default_selections: Vec<String>) -> Self {
        let name = format!("findMany{}", model_name);
        Self {
            model_name,
            selection: Selection::with_name(name),
            default_selections,
        }
    }

    pub fn add_arg(&mut self, name: String, val: ArgumentValue) {
        self.selection.push_argument(name, val);
    }

    pub fn add_selection(&mut self, name: String) {
        self.selection.push_nested_selection(Selection::with_name(name));
    }

    pub async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        if self.selection.nested_selections().is_empty() {
            for field in self.default_selections {
                self.selection.push_nested_selection(Selection::with_name(field));
            }
        }
        let operation = query_core::Operation::Read(self.selection);
        execute(operation, client).await
    }
}

/// Legacy FindUnique builder - wraps ReadBuilder
pub struct FindUniqueBuilder {
    pub model_name: String,
    pub selection: Selection,
    pub default_selections: Vec<String>,
}

impl FindUniqueBuilder {
    pub fn new(model_name: String, default_selections: Vec<String>) -> Self {
        let name = format!("findUnique{}", model_name);
        Self {
            model_name,
            selection: Selection::with_name(name),
            default_selections,
        }
    }

    pub fn add_arg(&mut self, name: String, val: ArgumentValue) {
        self.selection.push_argument(name, val);
    }

    pub fn add_selection(&mut self, name: String) {
        self.selection.push_nested_selection(Selection::with_name(name));
    }

    pub async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        if self.selection.nested_selections().is_empty() {
            for field in self.default_selections {
                self.selection.push_nested_selection(Selection::with_name(field));
            }
        }
        let operation = query_core::Operation::Read(self.selection);
        execute(operation, client).await
    }
}
