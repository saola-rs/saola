/// Aggregation query builders - count, aggregate, groupBy operations
use crate::builder::{BuilderState, Executable, Filterable, FromResponseIr, Selectable, execute_raw};
use crate::transaction::QueryExecutorProvider;
use query_core::{ArgumentValue, Selection};
use std::marker::PhantomData;
use std::sync::Arc;

/// Count builder - counts records matching criteria
pub struct CountBuilder {
    pub state: BuilderState,
    pub provider: Option<Arc<dyn QueryExecutorProvider>>,
}

impl CountBuilder {
    /// Create a count operation
    pub fn new(model_name: String, _default_selections: Vec<String>) -> Self {
        let mut state = BuilderState::read(model_name, "aggregate", Vec::new());
        let mut count_sel = Selection::with_name("_count");
        count_sel.push_nested_selection(Selection::with_name("_all"));
        state.selection.push_nested_selection(count_sel);

        Self { state, provider: None }
    }

    pub fn with_provider(mut self, provider: Arc<dyn QueryExecutorProvider>) -> Self {
        self.provider = Some(provider);
        self
    }

    pub async fn exec(self) -> crate::Result<i64> {
        let provider = self.provider.clone().ok_or_else(|| {
            crate::Error::RuntimeError("Builder is not bound to a provider.".to_string())
        })?;
        self.exec_with(provider.as_ref()).await
    }

    pub async fn exec_with<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<i64> {
        let operation = self.state.into_operation(false);
        let item = execute_raw(operation, provider).await?;

        // Count operations return `{ "_count": { "_all": n } }` or just `{ "_count": n }`
        // We try to drill down to the actual count value if possible
        let count_item = match &item {
            query_core::response_ir::Item::Map(map) => {
                let c = map.get("_count").unwrap_or(&item);
                match c {
                    query_core::response_ir::Item::Map(inner_map) => {
                        inner_map.get("_all").unwrap_or(c).clone()
                    }
                    _ => c.clone(),
                }
            }
            _ => item,
        };

        i64::from_ir(count_item)
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
        self.exec_with(provider).await
    }
}

/// Aggregate builder - aggregates data (sum, avg, min, max, count per field)
pub struct AggregateBuilder<T> {
    pub state: BuilderState,
    pub provider: Option<Arc<dyn QueryExecutorProvider>>,
    _phantom: PhantomData<T>,
}

impl<T> AggregateBuilder<T> {
    /// Create an aggregate operation
    pub fn new(model_name: String, _default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::read(model_name, "aggregate", Vec::new()),
            provider: None,
            _phantom: PhantomData,
        }
    }

    pub fn with_provider(mut self, provider: Arc<dyn QueryExecutorProvider>) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Transition this builder to return a different type
    pub fn with_type<U>(self) -> AggregateBuilder<U> {
        AggregateBuilder {
            state: self.state,
            provider: self.provider,
            _phantom: PhantomData,
        }
    }

    pub async fn exec(self) -> crate::Result<T>
    where
        T: FromResponseIr,
    {
        let provider = self.provider.clone().ok_or_else(|| {
            crate::Error::RuntimeError("Builder is not bound to a provider.".to_string())
        })?;
        self.exec_with(provider.as_ref()).await
    }

    pub async fn exec_with<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<T>
    where
        T: FromResponseIr,
    {
        let operation = self.state.into_operation(false);
        let item = execute_raw(operation, provider).await?;
        T::from_ir(item)
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
        self.exec_with(provider).await
    }
}

/// GroupBy builder - groups records by field(s) with aggregation
pub struct GroupByBuilder<T> {
    pub state: BuilderState,
    pub provider: Option<Arc<dyn QueryExecutorProvider>>,
    _phantom: PhantomData<T>,
}

impl<T> GroupByBuilder<T> {
    /// Create a groupBy operation
    pub fn new(model_name: String, _default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::read(model_name, "groupBy", Vec::new()),
            provider: None,
            _phantom: PhantomData,
        }
    }

    pub fn with_provider(mut self, provider: Arc<dyn QueryExecutorProvider>) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Transition this builder to return a different type
    pub fn with_type<U>(self) -> GroupByBuilder<U> {
        GroupByBuilder {
            state: self.state,
            provider: self.provider,
            _phantom: PhantomData,
        }
    }

    pub async fn exec(self) -> crate::Result<Vec<T>>
    where
        T: FromResponseIr,
    {
        let provider = self.provider.clone().ok_or_else(|| {
            crate::Error::RuntimeError("Builder is not bound to a provider.".to_string())
        })?;
        self.exec_with(provider.as_ref()).await
    }

    pub async fn exec_with<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<Vec<T>>
    where
        T: FromResponseIr,
    {
        let operation = self.state.into_operation(false);
        let item = execute_raw(operation, provider).await?;
        Vec::<T>::from_ir(item)
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
        self.exec_with(provider).await
    }
}
