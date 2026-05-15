use crate::Error;
use crate::transaction::QueryExecutorProvider;
/// Base traits and types for all query builders
/// This module provides the foundation for type-safe, composable query building
use query_core::{ArgumentValue, Operation, Selection, protocol::EngineProtocol};

/// Trait for any object that can execute a query and return typed results
pub trait Executable: Sized {
    type Output: FromResponseIr;

    /// Execute the query and return the inferred result type
    /// Can accept either &PrismaClient or &Transaction (both implement QueryExecutorProvider)
    #[allow(async_fn_in_trait)]
    async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<Self::Output>;
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
    /// Finalize and return all selections
    fn into_selections(self) -> Vec<Selection>;

    fn push_selection(&mut self, name: &str, selections: Vec<Selection>) {
        let mut sel = Selection::with_name(name.to_string());
        for s in selections {
            sel.push_nested_selection(s);
        }
        self.add_nested_selection(sel);
    }
}

pub trait SelectTransition<Marker> {
    type Output;
}

impl<T: SelectTransition<Marker>, Marker> SelectTransition<Marker> for Option<T> {
    type Output = Option<T::Output>;
}

impl<T: SelectTransition<Marker>, Marker> SelectTransition<Marker> for Vec<T> {
    type Output = Vec<T::Output>;
}

pub trait SelectAsTransition<U> {
    type Output;
}

impl<T: SelectAsTransition<U>, U> SelectAsTransition<U> for Option<T> {
    type Output = Option<U>;
}

impl<T: SelectAsTransition<U>, U> SelectAsTransition<U> for Vec<T> {
    type Output = Vec<U>;
}

pub trait SelectStruct: serde::de::DeserializeOwned + Send + Sync {
    fn selections() -> Vec<Selection>;
}

pub trait FromResponseIr: Sized {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self>;
}

impl FromResponseIr for String {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        match item {
            query_core::response_ir::Item::Value(query_structure::PrismaValue::String(s)) => Ok(s),
            _ => Err(crate::Error::RuntimeError("Expected string in response".to_string())),
        }
    }
}

impl FromResponseIr for i32 {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        match item {
            query_core::response_ir::Item::Value(query_structure::PrismaValue::Int(i)) => Ok(i as i32),
            _ => Err(crate::Error::RuntimeError("Expected i32 in response".to_string())),
        }
    }
}

impl FromResponseIr for i64 {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        match item {
            query_core::response_ir::Item::Value(query_structure::PrismaValue::Int(i)) => Ok(i),
            query_core::response_ir::Item::Value(query_structure::PrismaValue::BigInt(i)) => Ok(i),
            _ => Err(crate::Error::RuntimeError("Expected i64 in response".to_string())),
        }
    }
}

impl FromResponseIr for f64 {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        match item {
            query_core::response_ir::Item::Value(query_structure::PrismaValue::Float(f)) => {
                use bigdecimal::ToPrimitive;
                Ok(f.to_f64().unwrap_or(0.0))
            }
            _ => Err(crate::Error::RuntimeError("Expected f64 in response".to_string())),
        }
    }
}

impl FromResponseIr for bool {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        match item {
            query_core::response_ir::Item::Value(query_structure::PrismaValue::Boolean(b)) => Ok(b),
            _ => Err(crate::Error::RuntimeError("Expected bool in response".to_string())),
        }
    }
}

impl FromResponseIr for chrono::DateTime<chrono::Utc> {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        match item {
            query_core::response_ir::Item::Value(query_structure::PrismaValue::DateTime(dt)) => Ok(dt.into()),
            query_core::response_ir::Item::Value(query_structure::PrismaValue::String(s)) => {
                // Try parsing from string as fallback
                s.parse::<chrono::DateTime<chrono::Utc>>().map_err(|e| {
                    crate::Error::RuntimeError(format!("Failed to parse datetime from string '{}': {}", s, e))
                })
            }
            _ => Err(crate::Error::RuntimeError(format!(
                "Expected datetime in response, got {:?}",
                item
            ))),
        }
    }
}

impl FromResponseIr for serde_json::Value {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        Ok(serde_json::to_value(item)?)
    }
}

impl FromResponseIr for ::query_structure::PrismaValue {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        match item {
            query_core::response_ir::Item::Value(pv) => Ok(pv),
            _ => Err(crate::Error::RuntimeError(
                "Expected PrismaValue in response".to_string(),
            )),
        }
    }
}

impl FromResponseIr for Vec<u8> {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        match item {
            query_core::response_ir::Item::Value(query_structure::PrismaValue::Bytes(b)) => Ok(b),
            _ => Err(crate::Error::RuntimeError("Expected bytes in response".to_string())),
        }
    }
}

impl<T: FromResponseIr> FromResponseIr for Vec<T> {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        match item {
            query_core::response_ir::Item::List(list) => list.into_iter().map(T::from_ir).collect(),
            _ => Err(crate::Error::RuntimeError("Expected list in response".to_string())),
        }
    }
}

impl<T: FromResponseIr> FromResponseIr for Option<T> {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        match item {
            query_core::response_ir::Item::Value(query_structure::PrismaValue::Null) => Ok(None),
            _ => T::from_ir(item).map(Some),
        }
    }
}

impl<T: FromResponseIr> FromResponseIr for Box<T> {
    fn from_ir(item: query_core::response_ir::Item) -> crate::Result<Self> {
        T::from_ir(item).map(Box::new)
    }
}

pub trait GetSelections {
    fn get_selections() -> Vec<Selection>;
}

impl<T: SelectStruct> GetSelections for Option<T> {
    fn get_selections() -> Vec<Selection> {
        T::selections()
    }
}

impl<T: SelectStruct> GetSelections for Vec<T> {
    fn get_selections() -> Vec<Selection> {
        T::selections()
    }
}

impl<T: SelectStruct> GetSelections for Box<T> {
    fn get_selections() -> Vec<Selection> {
        T::selections()
    }
}

macro_rules! impl_get_selections_scalar {
    ($($t:ty),*) => {
        $(
            impl GetSelections for $t {
                fn get_selections() -> Vec<Selection> {
                    vec![]
                }
            }
        )*
    };
}

impl_get_selections_scalar!(
    String,
    i32,
    i64,
    f32,
    f64,
    bool,
    chrono::DateTime<chrono::Utc>,
    serde_json::Value,
    ::query_structure::PrismaValue
);

pub fn push_selection_with_type<T: GetSelections>(name: &str) -> Selection {
    let mut sel = Selection::with_name(name.to_string());
    for s in T::get_selections() {
        sel.push_nested_selection(s);
    }
    sel
}

impl<M: ModelMarker, Op, T> Query<M, Op, T> {
    pub fn select_as<U: SelectStruct>(mut self) -> Query<M, Op, <T as SelectAsTransition<U>>::Output>
    where
        T: SelectAsTransition<U>,
    {
        let selections = U::selections();
        self.state.selection.clear_nested_selections();
        for sel in selections {
            self.state.selection.push_nested_selection(sel);
        }
        self.with_type()
    }

    pub fn select<U, F>(mut self, f: F) -> Query<M, Op, <T as SelectAsTransition<U>>::Output>
    where
        U: serde::de::DeserializeOwned + Send + Sync,
        F: FnOnce(&mut M::Select),
        T: SelectAsTransition<U>,
    {
        let mut builder = M::Select::default();
        f(&mut builder);
        let selections = builder.into_selections();
        self.state.selection.clear_nested_selections();
        for sel in selections {
            self.state.selection.push_nested_selection(sel);
        }
        self.with_type()
    }
}

/// Backward compatibility trait - alias for FilterBuilder used by generated macros
pub trait FilterBuilder: Sized {
    fn add_arg(&mut self, name: String, value: ArgumentValue);
    fn build(self) -> crate::IndexMap<String, ArgumentValue>;
}

/// Helper for compile-time type checking in selection macros
pub struct SelectionField<'a, T, B>(&'a mut B, std::marker::PhantomData<T>);

impl<'a, T, B> SelectionField<'a, T, B> {
    pub fn new(builder: &'a mut B) -> Self {
        Self(builder, std::marker::PhantomData)
    }

    /// Asserts that the field type exactly matches T
    pub fn assert_type(self, _: &T) -> &'a mut B {
        self.0
    }
}

impl<'a, T, B> std::ops::Deref for SelectionField<'a, T, B> {
    type Target = B;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T, B> std::ops::DerefMut for SelectionField<'a, T, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

/// Helper for compile-time type checking for relations in selection macros
pub struct SelectionRelField<'a, T, B>(&'a mut B, std::marker::PhantomData<T>);

impl<'a, T, B> SelectionRelField<'a, T, B> {
    pub fn new(builder: &'a mut B) -> Self {
        Self(builder, std::marker::PhantomData)
    }

    /// Asserts that the relation container type (Vec vs Option) is compatible
    pub fn assert_rel_type<U: RelCompatible<T>>(self, _: &U) -> &'a mut B {
        self.0
    }
}

impl<'a, T, B> std::ops::Deref for SelectionRelField<'a, T, B> {
    type Target = B;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T, B> std::ops::DerefMut for SelectionRelField<'a, T, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

pub trait RelCompatible<T> {}
impl<A, B> RelCompatible<Vec<B>> for Vec<A> {}
impl<A, B> RelCompatible<Option<B>> for Option<A> {}

pub trait ModelMarker {
    /// The default data struct (e.g. UserData)
    type Data: serde::de::DeserializeOwned + Send + Sync + Default;
    /// The where builder for this model
    type Where: FilterBuilder + Default;
    /// The unique where builder for this model
    type UniqueWhere: FilterBuilder + Default;
    /// The order by builder for this model
    type OrderBy: Default;
    /// The include builder for this model
    type Include: Default;
    /// The select builder for this model
    type Select: Selectable + Default;
    /// The data builder for this model
    type DataBuilder: DataBuilderTrait + Default;

    /// The name of the model in the Prisma schema
    const NAME: &'static str;
    /// The list of default scalar fields to select
    const SCALAR_FIELDS: &'static [&'static str];
}

use std::sync::Arc;

/// Generic query builder for all operations
pub struct Query<M: ModelMarker, Op, T> {
    pub state: BuilderState,
    pub provider: Option<Arc<dyn QueryExecutorProvider>>,
    pub _marker: std::marker::PhantomData<(M, Op, T)>,
}

impl<M: ModelMarker, Op, T> Query<M, Op, T> {
    pub fn new(operation: &str) -> Self {
        Self {
            state: BuilderState::read(
                M::NAME.to_string(),
                operation,
                M::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            ),
            provider: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_provider(mut self, provider: Arc<dyn QueryExecutorProvider>) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Transition return type
    pub fn with_type<U>(self) -> Query<M, Op, U> {
        Query {
            state: self.state,
            provider: self.provider,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<M: ModelMarker, Op, T: FromResponseIr> Query<M, Op, T> {
    pub async fn exec(self) -> crate::Result<T> {
        let provider = self.provider.as_ref().ok_or_else(|| {
            crate::Error::RuntimeError(
                "Query is not bound to a provider. Use .exec_with(client) or bind it earlier.".to_string(),
            )
        })?;

        let op_name = std::any::type_name::<Op>();
        let is_write = op_name.contains("Create")
            || op_name.contains("Update")
            || op_name.contains("Upsert")
            || op_name.contains("Delete");

        let operation = self.state.into_operation(is_write);
        let tx_id = provider.tx_id().cloned();
        let response = provider
            .executor()
            .execute(tx_id, operation, provider.query_schema(), None, EngineProtocol::Json)
            .await
            .map_err(Error::from_core)?;

        let item = match response.data {
            query_core::response_ir::Item::Map(mut map) => map
                .shift_remove_index(0)
                .map(|(_, v)| v)
                .ok_or_else(|| crate::Error::RuntimeError("Inconsistent query result: No data returned".to_string()))?,
            item => item,
        };

        T::from_ir(unwrap_prisma_item(item))
    }

    pub async fn exec_with<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<T> {
        let op_name = std::any::type_name::<Op>();
        let is_write = op_name.contains("Create")
            || op_name.contains("Update")
            || op_name.contains("Upsert")
            || op_name.contains("Delete");

        let operation = self.state.into_operation(is_write);
        let tx_id = provider.tx_id().cloned();
        let response = provider
            .executor()
            .execute(tx_id, operation, provider.query_schema(), None, EngineProtocol::Json)
            .await
            .map_err(Error::from_core)?;

        let item = match response.data {
            query_core::response_ir::Item::Map(mut map) => map
                .shift_remove_index(0)
                .map(|(_, v)| v)
                .ok_or_else(|| crate::Error::RuntimeError("Inconsistent query result: No data returned".to_string()))?,
            item => item,
        };

        T::from_ir(unwrap_prisma_item(item))
    }
}

/// Operation markers
pub struct FindMany;
pub struct FindUnique;
pub struct FindFirst;
pub struct FindUniqueOrThrow;
pub struct FindFirstOrThrow;

/// Write markers
pub struct Create;
pub struct Update;
pub struct Upsert;
pub struct Delete;
pub struct CreateMany;
pub struct UpdateMany;
pub struct DeleteMany;

fn unwrap_prisma_item(item: query_core::response_ir::Item) -> query_core::response_ir::Item {
    match item {
        query_core::response_ir::Item::Value(query_structure::PrismaValue::Object(map)) => {
            let mut has_type = false;
            let mut value = None;

            for (k, v) in &map {
                if k == "$type" || k == "prisma__type" {
                    has_type = true;
                }
                if k == "value" || k == "prisma__value" {
                    value = Some(v.clone());
                }
            }

            if has_type && value.is_some() {
                // Recurse in case the value itself is wrapped
                unwrap_prisma_item(query_core::response_ir::Item::Value(value.unwrap()))
            } else {
                query_core::response_ir::Item::Value(query_structure::PrismaValue::Object(map))
            }
        }
        query_core::response_ir::Item::Map(map) => {
            let mut new_map = query_core::response_ir::Map::default();
            for (k, v) in map {
                new_map.insert(k, unwrap_prisma_item(v));
            }
            query_core::response_ir::Item::Map(new_map)
        }
        query_core::response_ir::Item::List(list) => {
            let vec: Vec<_> = list.into_iter().map(unwrap_prisma_item).collect();
            query_core::response_ir::Item::List(query_core::response_ir::List::from(vec))
        }
        query_core::response_ir::Item::Ref(r) => {
            // Unwrap the reference and then unwrap the item it points to
            match std::sync::Arc::try_unwrap(r) {
                Ok(item) => unwrap_prisma_item(item),
                Err(arc) => unwrap_prisma_item((*arc).clone()),
            }
        }
        _ => item,
    }
}

impl<M: ModelMarker, Op, T: FromResponseIr> Executable for Query<M, Op, T> {
    type Output = T;

    async fn exec<P: QueryExecutorProvider + ?Sized>(self, provider: &P) -> crate::Result<T> {
        self.exec_with(provider).await
    }
}

// Common query methods
impl<M: ModelMarker, Op, T> Query<M, Op, T> {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut M::Where),
    {
        let mut builder = M::Where::default();
        f(&mut builder);
        let map = builder.build();
        if !map.is_empty() {
            self.state
                .arguments
                .insert("where".to_string(), ArgumentValue::Object(map));
        }
        self
    }

    pub fn where_unique<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut M::UniqueWhere),
    {
        let mut builder = M::UniqueWhere::default();
        f(&mut builder);
        let map = builder.build();
        if !map.is_empty() {
            self.state
                .arguments
                .insert("where".to_string(), ArgumentValue::Object(map));
        }
        self
    }

    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut M::DataBuilder),
        M::DataBuilder: crate::builder::DataBuilderTrait,
    {
        let mut builder = M::DataBuilder::default();
        f(&mut builder);
        let map = builder.build();
        if !map.is_empty() {
            if let Some(ArgumentValue::Object(existing_map)) = self.state.arguments.get_mut("data") {
                for (k, v) in map {
                    existing_map.insert(k, v);
                }
            } else {
                self.state
                    .arguments
                    .insert("data".to_string(), ArgumentValue::Object(map));
            }
        }
        self
    }
}

pub trait DataBuilderTrait {
    fn build(self) -> crate::IndexMap<String, ArgumentValue>;
}

pub async fn execute_raw<P: crate::transaction::QueryExecutorProvider + ?Sized>(
    operation: Operation,
    provider: &P,
) -> crate::Result<query_core::response_ir::Item> {
    let tx_id = provider.tx_id().cloned();
    let response = provider
        .executor()
        .execute(tx_id, operation, provider.query_schema(), None, EngineProtocol::Json)
        .await
        .map_err(Error::from_core)?;

    let item = match response.data {
        query_core::response_ir::Item::Map(mut map) => map
            .shift_remove_index(0)
            .map(|(_, v)| v)
            .ok_or_else(|| crate::Error::RuntimeError("Inconsistent query result: No data returned".to_string()))?,
        item => item,
    };

    Ok(unwrap_prisma_item(item))
}

// Pagination and Ordering (FindMany/FindFirst)
impl<M: ModelMarker, Op, T> Query<M, Op, T> {
    pub fn take(mut self, n: i64) -> Self {
        self.state.arguments.insert(
            "take".to_string(),
            ArgumentValue::Scalar(crate::query_structure::PrismaValue::Int(n)),
        );
        self
    }
    pub fn skip(mut self, n: i64) -> Self {
        self.state.arguments.insert(
            "skip".to_string(),
            ArgumentValue::Scalar(crate::query_structure::PrismaValue::Int(n)),
        );
        self
    }
    // Note: order_by needs a trait or specific impl because every model has its own OrderBy builder
}

/// Generic trait for relation loading transitions
pub trait IncludeTransition<Marker> {
    type Output;
}

impl<T: IncludeTransition<Marker>, Marker> IncludeTransition<Marker> for Option<T> {
    type Output = Option<T::Output>;
}

impl<T: IncludeTransition<Marker>, Marker> IncludeTransition<Marker> for Vec<T> {
    type Output = Vec<T::Output>;
}

pub trait IncludeMarker {
    fn into_selection(self) -> Option<Selection>;
}

// Marker for no inclusions
pub struct IncludeEmpty;
impl IncludeMarker for IncludeEmpty {
    fn into_selection(self) -> Option<Selection> {
        None
    }
}

impl<M: ModelMarker, Op, T> Query<M, Op, T> {
    pub fn include<IM, F>(mut self, f: F) -> Query<M, Op, <T as IncludeTransition<IM>>::Output>
    where
        F: FnOnce(&mut M::Include) -> IM,
        IM: IncludeMarker,
        T: IncludeTransition<IM>,
    {
        let mut builder = M::Include::default();
        let marker = f(&mut builder);

        // Apply scalar defaults if first inclusion
        if self.state.selection.nested_selections().is_empty() {
            for field in M::SCALAR_FIELDS {
                self.state
                    .selection
                    .push_nested_selection(Selection::with_name(field.to_string()));
            }
        }

        if let Some(nested) = marker.into_selection() {
            self.state.selection.push_nested_selection(nested);
        }

        // Note: Include builders can also have arguments (like where/take on nested relations)
        // We'll need a way to extract them if the builder populates them.

        self.with_type()
    }
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

/// Recursively unwrap Prisma JSON protocol objects (e.g. {"$type": "DateTime", "value": "..."})
fn _unwrap_prisma_value(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(mut map) => {
            if map.len() == 2 && map.contains_key("$type") && map.contains_key("value") {
                return map.remove("value").unwrap();
            }
            serde_json::Value::Object(map.into_iter().map(|(k, v)| (k, _unwrap_prisma_value(v))).collect())
        }
        serde_json::Value::Array(list) => {
            serde_json::Value::Array(list.into_iter().map(_unwrap_prisma_value).collect())
        }
        _ => value,
    }
}
