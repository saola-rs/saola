use crate::builder::{BuilderState, Executable, Filterable, Selectable, execute};
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

    pub async fn exec(self, client: &crate::client::PrismaClient) -> crate::Result<i64> {
        let operation = self.state.into_operation(true);
        let res = crate::builder::execute_raw(operation, client).await?;

        // For bulk operations, count is at the top level, not nested under operation name
        let count = res.get("count").and_then(|v| v.as_i64()).unwrap_or(0);
        Ok(count)
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

impl<T: serde::de::DeserializeOwned + Send + Sync> CreateManyAndReturnBuilder<T> {
    pub async fn exec(self, client: &crate::client::PrismaClient) -> crate::Result<Vec<T>> {
        let operation = self.state.into_operation(true);
        let op_name = match &operation {
            query_core::Operation::Write(s) => s.name().to_string(),
            _ => String::new(),
        };
        let res = crate::builder::execute_raw(operation, client).await?;

        // For bulk CreateManyAndReturn operations, the result is either directly an array
        // or nested under the operation name
        let list = if let serde_json::Value::Array(_) = res {
            res
        } else {
            res.get(&op_name)
                .cloned()
                .unwrap_or(serde_json::Value::Array(Vec::new()))
        };
        Ok(serde_json::from_value(list)?)
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

    pub async fn exec(self, client: &crate::client::PrismaClient) -> crate::Result<i64> {
        let operation = self.state.into_operation(true);
        let res = crate::builder::execute_raw(operation, client).await?;

        // For bulk operations, count is at the top level, not nested under operation name
        let count = res.get("count").and_then(|v| v.as_i64()).unwrap_or(0);
        Ok(count)
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

impl<T: serde::de::DeserializeOwned + Send + Sync> UpdateManyAndReturnBuilder<T> {
    pub async fn exec(self, client: &crate::client::PrismaClient) -> crate::Result<Vec<T>> {
        let operation = self.state.into_operation(true);
        let op_name = match &operation {
            query_core::Operation::Write(s) => s.name().to_string(),
            _ => String::new(),
        };
        let res = crate::builder::execute_raw(operation, client).await?;

        // For bulk UpdateManyAndReturn operations, the result is either directly an array
        // or nested under the operation name
        let list = if let serde_json::Value::Array(_) = res {
            res
        } else {
            res.get(&op_name)
                .cloned()
                .unwrap_or(serde_json::Value::Array(Vec::new()))
        };
        Ok(serde_json::from_value(list)?)
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

    pub async fn exec(self, client: &crate::client::PrismaClient) -> crate::Result<i64> {
        let operation = self.state.into_operation(true);
        let res = crate::builder::execute_raw(operation, client).await?;

        // For bulk operations, count is at the top level, not nested under operation name
        let count = res.get("count").and_then(|v| v.as_i64()).unwrap_or(0);
        Ok(count)
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
}

impl<T: serde::de::DeserializeOwned + Send + Sync> Executable for WriteBuilder<T> {
    async fn exec<Ret: serde::de::DeserializeOwned>(self, client: &crate::client::PrismaClient) -> crate::Result<Ret> {
        let operation = self.state.into_operation(true);
        execute(operation, client).await
    }
}

impl<T: serde::de::DeserializeOwned + Send + Sync> WriteBuilder<T> {
    pub async fn exec_inferred(self, client: &crate::client::PrismaClient) -> crate::Result<T> {
        let operation = self.state.into_operation(true);
        execute(operation, client).await
    }
}
