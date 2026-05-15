use crate::builder::{BuilderState, Executable, Filterable, FromResponseIr, Selectable, execute_raw};
use crate::transaction::QueryExecutorProvider;
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

    /// Create an upsert operation (upsertOne{Model})
    pub fn upsert(model_name: String, default_selections: Vec<String>) -> Self {
        Self::new(model_name, "upsertOne", default_selections)
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

/// Builder for bulk create operations (returns count)
pub struct CreateManyBuilder {
    pub state: BuilderState,
}

impl CreateManyBuilder {
    pub fn new(model_name: String) -> Self {
        let mut state = BuilderState::write(model_name, "createMany", Vec::new());
        state.selection.push_nested_selection(Selection::with_name("count"));
        Self { state }
    }

    pub fn skip_duplicates(mut self, skip: bool) -> Self {
        self.state.arguments.insert(
            "skipDuplicates".to_string(),
            ArgumentValue::Scalar(crate::query_structure::PrismaValue::Boolean(skip)),
        );
        self
    }

    pub async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<i64> {
        let operation = self.state.into_operation(true);
        let item = execute_raw(operation, provider).await?;

        if let query_core::response_ir::Item::Map(map) = item {
            map.get("count")
                .and_then(|i| {
                    if let query_core::response_ir::Item::Value(query_structure::PrismaValue::Int(n)) = i {
                        Some(*n)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| crate::Error::RuntimeError("Failed to extract count from createMany".to_string()))
        } else {
            Err(crate::Error::RuntimeError(
                "Unexpected response format from createMany".to_string(),
            ))
        }
    }
}

/// Builder for bulk create operations and return records
pub struct CreateManyAndReturnBuilder<T> {
    pub state: BuilderState,
    _phantom: PhantomData<T>,
}

impl<T> CreateManyAndReturnBuilder<T> {
    pub fn new(model_name: String, default_selections: Vec<String>) -> Self {
        let name = format!("createMany{}AndReturn", model_name);
        Self {
            state: BuilderState {
                model_name,
                selection: Selection::with_name(name),
                arguments: crate::IndexMap::new(),
                default_selections,
            },
            _phantom: PhantomData,
        }
    }

    pub fn skip_duplicates(mut self, skip: bool) -> Self {
        self.state.arguments.insert(
            "skipDuplicates".to_string(),
            ArgumentValue::Scalar(crate::query_structure::PrismaValue::Boolean(skip)),
        );
        self
    }
}

impl<T: FromResponseIr + Send + Sync> CreateManyAndReturnBuilder<T> {
    pub async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<Vec<T>> {
        let operation = self.state.into_operation(true);
        let item = execute_raw(operation, provider).await?;
        Vec::<T>::from_ir(item)
    }
}

/// Builder for bulk update operations
pub struct UpdateManyBuilder {
    pub state: BuilderState,
}

impl UpdateManyBuilder {
    pub fn new(model_name: String) -> Self {
        let mut state = BuilderState::write(model_name, "updateMany", Vec::new());
        state.selection.push_nested_selection(Selection::with_name("count"));
        Self { state }
    }

    pub async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<i64> {
        let operation = self.state.into_operation(true);
        let item = execute_raw(operation, provider).await?;

        if let query_core::response_ir::Item::Map(map) = item {
            map.get("count")
                .and_then(|i| {
                    if let query_core::response_ir::Item::Value(query_structure::PrismaValue::Int(n)) = i {
                        Some(*n)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| crate::Error::RuntimeError("Failed to extract count from updateMany".to_string()))
        } else {
            Err(crate::Error::RuntimeError(
                "Unexpected response format from updateMany".to_string(),
            ))
        }
    }
}

/// Builder for bulk update operations and return records
pub struct UpdateManyAndReturnBuilder<T> {
    pub state: BuilderState,
    _phantom: PhantomData<T>,
}

impl<T> UpdateManyAndReturnBuilder<T> {
    pub fn new(model_name: String, default_selections: Vec<String>) -> Self {
        let name = format!("updateMany{}AndReturn", model_name);
        Self {
            state: BuilderState {
                model_name,
                selection: Selection::with_name(name),
                arguments: crate::IndexMap::new(),
                default_selections,
            },
            _phantom: PhantomData,
        }
    }
}

impl<T: FromResponseIr + Send + Sync> UpdateManyAndReturnBuilder<T> {
    pub async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<Vec<T>> {
        let operation = self.state.into_operation(true);
        let item = execute_raw(operation, provider).await?;
        Vec::<T>::from_ir(item)
    }
}

impl<M> Filterable for UpdateManyAndReturnBuilder<M> {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.arguments.insert(name, value);
    }
}

impl<M> Selectable for UpdateManyAndReturnBuilder<M> {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
    fn into_selections(self) -> Vec<Selection> {
        self.state.selection.nested_selections().to_vec()
    }
}

/// Builder for bulk delete operations
pub struct DeleteManyBuilder {
    pub state: BuilderState,
}

impl DeleteManyBuilder {
    pub fn new(model_name: String) -> Self {
        let mut state = BuilderState::write(model_name, "deleteMany", Vec::new());
        state.selection.push_nested_selection(Selection::with_name("count"));
        Self { state }
    }

    pub async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<i64> {
        let operation = self.state.into_operation(true);
        let item = execute_raw(operation, provider).await?;

        if let query_core::response_ir::Item::Map(map) = item {
            map.get("count")
                .and_then(|i| {
                    if let query_core::response_ir::Item::Value(query_structure::PrismaValue::Int(n)) = i {
                        Some(*n)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| crate::Error::RuntimeError("Failed to extract count from deleteMany".to_string()))
        } else {
            Err(crate::Error::RuntimeError(
                "Unexpected response format from deleteMany".to_string(),
            ))
        }
    }
}

impl Filterable for CreateManyBuilder {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.arguments.insert(name, value);
    }
}

impl<M> Filterable for CreateManyAndReturnBuilder<M> {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.arguments.insert(name, value);
    }
}

impl<M> Selectable for CreateManyAndReturnBuilder<M> {
    fn add_nested_selection(&mut self, selection: Selection) {
        self.state.selection.push_nested_selection(selection);
    }
    fn into_selections(self) -> Vec<Selection> {
        self.state.selection.nested_selections().to_vec()
    }
}

impl Filterable for UpdateManyBuilder {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.arguments.insert(name, value);
    }
}

impl Filterable for DeleteManyBuilder {
    fn add_filter_arg(&mut self, name: String, value: ArgumentValue) {
        self.state.arguments.insert(name, value);
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
    fn into_selections(self) -> Vec<Selection> {
        self.state.selection.nested_selections().to_vec()
    }
}

impl<T: FromResponseIr> Executable for WriteBuilder<T> {
    type Output = T;

    async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<T> {
        let operation = self.state.into_operation(true);
        let item = execute_raw(operation, provider).await?;
        T::from_ir(item)
    }
}

impl<T: FromResponseIr + Send + Sync> WriteBuilder<T> {
    pub async fn exec_inferred<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<T> {
        self.exec(provider).await
    }
}
