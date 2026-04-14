/// Aggregation query builders - count, aggregate, groupBy operations
use crate::builder::{BuilderState, Executable, Filterable, Selectable, execute};
use query_core::{ArgumentValue, Selection};

/// Count builder - counts records matching criteria
pub struct CountBuilder {
    state: BuilderState,
}

impl CountBuilder {
    /// Create a count operation
    pub fn new(model_name: String, _default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::read(model_name, "count", Vec::new()),
        }
    }
}

impl Filterable for CountBuilder {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.selection.push_argument(name, value);
    }
}

impl Selectable for CountBuilder {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
}

impl Executable for CountBuilder {
    async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        let operation = self.state.into_operation(false);
        execute(operation, client).await
    }
}

/// Aggregate builder - aggregates data (sum, avg, min, max, count per field)
pub struct AggregateBuilder {
    state: BuilderState,
}

impl AggregateBuilder {
    /// Create an aggregate operation
    pub fn new(model_name: String, _default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::read(model_name, "aggregate", Vec::new()),
        }
    }
}

impl Filterable for AggregateBuilder {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.selection.push_argument(name, value);
    }
}

impl Selectable for AggregateBuilder {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
}

impl Executable for AggregateBuilder {
    async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        let operation = self.state.into_operation(false);
        execute(operation, client).await
    }
}

/// GroupBy builder - groups records by field(s) with aggregation
pub struct GroupByBuilder {
    state: BuilderState,
}

impl GroupByBuilder {
    /// Create a groupBy operation
    pub fn new(model_name: String, _default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::read(model_name, "groupBy", Vec::new()),
        }
    }
}

impl Filterable for GroupByBuilder {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.selection.push_argument(name, value);
    }
}

impl Selectable for GroupByBuilder {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
}

impl Executable for GroupByBuilder {
    async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        let operation = self.state.into_operation(false);
        execute(operation, client).await
    }
}
