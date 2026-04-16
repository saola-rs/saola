/// Aggregation query builders - count, aggregate, groupBy operations
use crate::builder::{BuilderState, Executable, Filterable, Selectable, execute_raw};
use crate::transaction::QueryExecutorProvider;
use query_core::{ArgumentValue, Selection};
use std::marker::PhantomData;

/// Count builder - counts records matching criteria
pub struct CountBuilder {
    state: BuilderState,
}

impl CountBuilder {
    /// Create a count operation
    pub fn new(model_name: String, _default_selections: Vec<String>) -> Self {
        let mut state = BuilderState::read(model_name, "aggregate", Vec::new());
        let mut count_sel = Selection::with_name("_count");
        count_sel.push_nested_selection(Selection::with_name("_all"));
        state.selection.push_nested_selection(count_sel);

        Self { state }
    }
}

impl Filterable for CountBuilder {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.arguments.insert(name, value);
    }
}

impl Selectable for CountBuilder {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
}

impl Executable for CountBuilder {
    async fn exec<T: serde::de::DeserializeOwned, P: QueryExecutorProvider + ?Sized>(
        self,
        provider: &P,
    ) -> crate::Result<T> {
        let operation = self.state.into_operation(false);
        // Extract op_name BEFORE moving operation into execute_raw
        let op_name = match &operation {
            query_core::Operation::Read(s) => s.name().to_string(),
            _ => String::new(),
        };

        let res = execute_raw(operation, provider).await?;

        let data = res.get(&op_name).unwrap_or(&res);
        let count_val = data.get("_count").and_then(|c| c.get("_all")).unwrap_or(data);

        Ok(serde_json::from_value(count_val.clone())?)
    }
}

/// Aggregate builder - aggregates data (sum, avg, min, max, count per field)
pub struct AggregateBuilder<T> {
    state: BuilderState,
    _phantom: PhantomData<T>,
}

impl<T> AggregateBuilder<T> {
    /// Create an aggregate operation
    pub fn new(model_name: String, _default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::read(model_name, "aggregate", Vec::new()),
            _phantom: PhantomData,
        }
    }

    /// Transition this builder to return a different type
    pub fn with_type<U>(self) -> AggregateBuilder<U> {
        AggregateBuilder {
            state: self.state,
            _phantom: PhantomData,
        }
    }
}

impl<T> Filterable for AggregateBuilder<T> {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.arguments.insert(name, value);
    }
}

impl<T> Selectable for AggregateBuilder<T> {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
}

impl<T: serde::de::DeserializeOwned + Send + Sync> Executable for AggregateBuilder<T> {
    async fn exec<Ret: serde::de::DeserializeOwned, P: QueryExecutorProvider + ?Sized>(
        self,
        provider: &P,
    ) -> crate::Result<Ret> {
        let operation = self.state.into_operation(false);
        let op_name = match &operation {
            query_core::Operation::Read(s) => s.name().to_string(),
            _ => String::new(),
        };
        let res = execute_raw(operation, provider).await?;

        let data = res.get(&op_name).cloned().unwrap_or(res);
        Ok(serde_json::from_value(data)?)
    }
}

impl<T: serde::de::DeserializeOwned + Send + Sync> AggregateBuilder<T> {
    pub async fn exec_inferred<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<T> {
        let operation = self.state.into_operation(false);
        let op_name = match &operation {
            query_core::Operation::Read(s) => s.name().to_string(),
            _ => String::new(),
        };
        let res = execute_raw(operation, provider).await?;

        let data = res.get(&op_name).cloned().unwrap_or(res);
        Ok(serde_json::from_value(data)?)
    }
}

/// GroupBy builder - groups records by field(s) with aggregation
pub struct GroupByBuilder<T> {
    state: BuilderState,
    _phantom: PhantomData<T>,
}

impl<T> GroupByBuilder<T> {
    /// Create a groupBy operation
    pub fn new(model_name: String, _default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::read(model_name, "groupBy", Vec::new()),
            _phantom: PhantomData,
        }
    }

    /// Transition this builder to return a different type
    pub fn with_type<U>(self) -> GroupByBuilder<U> {
        GroupByBuilder {
            state: self.state,
            _phantom: PhantomData,
        }
    }
}

impl<T> Filterable for GroupByBuilder<T> {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.arguments.insert(name, value);
    }
}

impl<T> Selectable for GroupByBuilder<T> {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
}

impl<T: serde::de::DeserializeOwned + Send + Sync> Executable for GroupByBuilder<T> {
    async fn exec<Ret: serde::de::DeserializeOwned, P: QueryExecutorProvider + ?Sized>(
        self,
        provider: &P,
    ) -> crate::Result<Ret> {
        let operation = self.state.into_operation(false);
        let op_name = match &operation {
            query_core::Operation::Read(s) => s.name().to_string(),
            _ => String::new(),
        };
        let res = execute_raw(operation, provider).await?;

        let data = res.get(&op_name).cloned().unwrap_or(res);
        Ok(serde_json::from_value(data)?)
    }
}

impl<T: serde::de::DeserializeOwned + Send + Sync> GroupByBuilder<T> {
    pub async fn exec_inferred<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<Vec<T>> {
        let operation = self.state.into_operation(false);
        let op_name = match &operation {
            query_core::Operation::Read(s) => s.name().to_string(),
            _ => String::new(),
        };
        let res = execute_raw(operation, provider).await?;

        let list = res.get(&op_name).cloned().unwrap_or(res);
        Ok(serde_json::from_value(list)?)
    }
}
