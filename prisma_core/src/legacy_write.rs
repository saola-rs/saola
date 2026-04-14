/// Legacy write builders - kept for backward compatibility with generated macros

use query_core::{ArgumentValue, Selection};

/// Legacy Create builder
pub struct CreateBuilder {
    pub model_name: String,
    pub selection: Selection,
    pub default_selections: Vec<String>,
}

impl CreateBuilder {
    pub fn new(model_name: String, default_selections: Vec<String>) -> Self {
        let name = format!("createOne{}", model_name);
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
        let operation = query_core::Operation::Write(self.selection);
        crate::builder::execute(operation, client).await
    }
}

/// Legacy Update builder
pub struct UpdateBuilder {
    pub model_name: String,
    pub selection: Selection,
    pub default_selections: Vec<String>,
}

impl UpdateBuilder {
    pub fn new(model_name: String, default_selections: Vec<String>) -> Self {
        let name = format!("updateOne{}", model_name);
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
        let operation = query_core::Operation::Write(self.selection);
        crate::builder::execute(operation, client).await
    }
}

/// Legacy Delete builder
pub struct DeleteBuilder {
    pub model_name: String,
    pub selection: Selection,
    pub default_selections: Vec<String>,
}

impl DeleteBuilder {
    pub fn new(model_name: String, default_selections: Vec<String>) -> Self {
        let name = format!("deleteOne{}", model_name);
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
        let operation = query_core::Operation::Write(self.selection);
        crate::builder::execute(operation, client).await
    }
}
