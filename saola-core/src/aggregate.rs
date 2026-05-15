/// Aggregation query builders - count, aggregate, groupBy operations
use crate::builder::{BuilderState, Executable, Filterable, FromResponseIr, Selectable, execute_raw};
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
    fn into_selections(self) -> Vec<Selection> {
        self.state.selection.nested_selections().to_vec()
    }
}

impl Executable for CountBuilder {
    type Output = i64;

    async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<i64> {
        let operation = self.state.into_operation(false);
        let item = execute_raw(operation, provider).await?;

        // Count operations return `{ "_count": { "_all": n } }` or just `{ "_count": n }`
        // We try to drill down to the actual count value if possible
        let count_item = if let query_core::response_ir::Item::Map(map) = &item {
            map.get("_count").cloned().unwrap_or(item)
        } else {
            item
        };

        i64::from_ir(count_item)
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
    fn into_selections(self) -> Vec<Selection> {
        self.state.selection.nested_selections().to_vec()
    }
}

impl<T: FromResponseIr> Executable for AggregateBuilder<T> {
    type Output = T;

    async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<T> {
        let operation = self.state.into_operation(false);
        let item = execute_raw(operation, provider).await?;
        T::from_ir(item)
    }
}

impl<T: FromResponseIr + Send + Sync> AggregateBuilder<T> {
    pub async fn exec_inferred<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<T> {
        self.exec(provider).await
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
    fn into_selections(self) -> Vec<Selection> {
        self.state.selection.nested_selections().to_vec()
    }
}

impl<T: FromResponseIr> Executable for GroupByBuilder<T> {
    type Output = Vec<T>;

    async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<Vec<T>> {
        let operation = self.state.into_operation(false);
        let item = execute_raw(operation, provider).await?;
        Vec::<T>::from_ir(item)
    }
}

impl<T: FromResponseIr + Send + Sync> GroupByBuilder<T> {
    pub async fn exec_inferred<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<Vec<T>> {
        self.exec(provider).await
    }
}
