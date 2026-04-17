use crate::transaction::QueryExecutorProvider;
/// Base traits and types for all query builders
/// This module provides the foundation for type-safe, composable query building
use query_core::{ArgumentValue, Operation, Selection, protocol::EngineProtocol};

/// Trait for any object that can execute a query and return typed results
pub trait Executable: Sized {
    /// Execute the query and deserialize result to type T
    /// Can accept either &PrismaClient or &Transaction (both implement QueryExecutorProvider)
    #[allow(async_fn_in_trait)]
    async fn exec<T: serde::de::DeserializeOwned, P: QueryExecutorProvider + ?Sized>(
        self,
        provider: &P,
    ) -> crate::Result<T>;
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

pub async fn execute_raw<P: crate::transaction::QueryExecutorProvider + ?Sized>(
    operation: Operation,
    provider: &P,
) -> crate::Result<serde_json::Value> {
    let tx_id = provider.tx_id().cloned();
    let response = provider
        .executor()
        .execute(tx_id, operation, provider.query_schema(), None, EngineProtocol::Json)
        .await?;

    Ok(serde_json::to_value(&response.data)?)
}

/// Recursively unwrap Prisma JSON protocol objects (e.g. {"$type": "DateTime", "value": "..."})
fn unwrap_prisma_value(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(mut map) => {
            if map.len() == 2 && map.contains_key("$type") && map.contains_key("value") {
                return map.remove("value").unwrap();
            }
            serde_json::Value::Object(map.into_iter().map(|(k, v)| (k, unwrap_prisma_value(v))).collect())
        }
        serde_json::Value::Array(list) => {
            serde_json::Value::Array(list.into_iter().map(unwrap_prisma_value).collect())
        }
        _ => value,
    }
}

/// Common implementation of execution logic across all builders
pub async fn execute<T: serde::de::DeserializeOwned, P: crate::transaction::QueryExecutorProvider + ?Sized>(
    operation: Operation,
    provider: &P,
) -> crate::Result<T> {
    let json = execute_raw(operation, provider).await?;
    let unwrapped = unwrap_prisma_value(json);
    Ok(serde_json::from_value(unwrapped)?)
}
