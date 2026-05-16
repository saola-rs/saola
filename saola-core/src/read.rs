use crate::builder::{BuilderState, Executable, Filterable, FromResponseIr, Selectable, execute_raw};
use crate::transaction::QueryExecutorProvider;
use query_core::{ArgumentValue, Selection};
use std::marker::PhantomData;
use std::sync::Arc;

/// Generic Read builder - base for all find operations
pub struct ReadBuilder<T> {
    pub state: BuilderState,
    pub provider: Option<Arc<dyn QueryExecutorProvider>>,
    _phantom: PhantomData<T>,
}

impl<T> ReadBuilder<T> {
    /// Create a new read builder for the given operation
    pub fn new(model_name: String, operation: &str, default_selections: Vec<String>) -> Self {
        Self {
            state: BuilderState::read(model_name, operation, default_selections),
            provider: None,
            _phantom: PhantomData,
        }
    }

    pub fn with_provider(mut self, provider: Arc<dyn QueryExecutorProvider>) -> Self {
        self.provider = Some(provider);
        self
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
            provider: self.provider,
            _phantom: PhantomData,
        }
    }

    pub fn select_as<U: crate::builder::SelectStruct, R>(mut self) -> ReadBuilder<R> {
        let selections = U::selections();
        self.state.selection.clear_nested_selections();
        for sel in selections {
            self.state.selection.push_nested_selection(sel);
        }
        ReadBuilder {
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

impl<T> Filterable for ReadBuilder<T> {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.arguments.insert(name, value);
    }
}

impl<T> Selectable for ReadBuilder<T> {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
    fn into_selections(self) -> Vec<Selection> {
        self.state.selection.nested_selections().to_vec()
    }
}

impl<T: FromResponseIr> Executable for ReadBuilder<T> {
    type Output = T;

    async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<T> {
        self.exec_with(provider).await
    }
}
