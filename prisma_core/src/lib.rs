pub use query_structure;
pub use query_core;
pub use schema;
pub use indexmap::IndexMap;
pub use serde_json;
pub use serde;
pub use anyhow;

use query_core::{QueryExecutor, executor::InterpretingExecutor, protocol::EngineProtocol, Operation, Selection};
use sql_query_connector::{FromSource, PostgreSql, Mysql, Sqlite};
use std::sync::Arc;
use psl::parser_database::NoExtensionTypes;

pub struct PrismaClient {
    executor: Arc<dyn QueryExecutor + Send + Sync>,
    query_schema: Arc<schema::QuerySchema>,
}

pub trait FilterBuilder {
    fn add_arg(&mut self, name: String, value: query_core::ArgumentValue);
}

impl PrismaClient {
    pub async fn new(schema_str: &str, url: &str) -> Result<Self, anyhow::Error> {
        let source_file = psl::SourceFile::from(schema_str);
        let validated = psl::validate(source_file, &NoExtensionTypes);
        
        if !validated.diagnostics.errors().is_empty() {
            anyhow::bail!("Schema validation failed");
        }

        let validated = Arc::new(validated);
        let datasource = validated.configuration.datasources.first()
            .ok_or_else(|| anyhow::anyhow!("No datasource found in schema"))?;

        let query_schema = Arc::new(schema::build(validated.clone(), true));

        let executor: Arc<dyn QueryExecutor + Send + Sync> = match datasource.active_provider {
            "postgresql" | "postgres" => {
                let connector = PostgreSql::from_source(datasource, url, psl::PreviewFeatures::empty(), false).await?;
                Arc::new(InterpretingExecutor::new(connector, false))
            }
            "mysql" => {
                let connector = Mysql::from_source(datasource, url, psl::PreviewFeatures::empty(), false).await?;
                Arc::new(InterpretingExecutor::new(connector, false))
            }
            "sqlite" => {
                let connector = Sqlite::from_source(datasource, url, psl::PreviewFeatures::empty(), false).await?;
                Arc::new(InterpretingExecutor::new(connector, false))
            }
            _ => anyhow::bail!("Unsupported provider: {}", datasource.active_provider),
        };

        Ok(PrismaClient {
            executor,
            query_schema,
        })
    }

    pub fn executor(&self) -> Arc<dyn QueryExecutor + Send + Sync> {
        self.executor.clone()
    }

    pub fn query_schema(&self) -> Arc<schema::QuerySchema> {
        self.query_schema.clone()
    }
}

pub struct FindManyBuilder {
    pub model_name: String,
    pub selection: Selection,
    pub default_selections: Vec<String>,
}

impl FindManyBuilder {
    pub fn new(model_name: String, default_selections: Vec<String>) -> Self {
        let name = format!("findMany{}", model_name);
        Self {
            model_name,
            selection: Selection::with_name(name),
            default_selections,
        }
    }

    pub fn add_arg(&mut self, name: String, val: query_core::ArgumentValue) {
        self.selection.push_argument(name, val);
    }

    pub fn add_selection(&mut self, name: String) {
        self.selection.push_nested_selection(Selection::with_name(name));
    }

    pub async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &PrismaClient) -> Result<T, anyhow::Error> {
        if self.selection.nested_selections().is_empty() {
            for field in self.default_selections {
                self.selection.push_nested_selection(Selection::with_name(field));
            }
        }
        let operation = Operation::Read(self.selection);
        
        let response = client.executor().execute(
            None,
            operation,
            client.query_schema(),
            None,
            EngineProtocol::Json,
        ).await?;

        Ok(serde_json::from_value(serde_json::to_value(&response.data)?)?)
    }
}

pub struct FindUniqueBuilder {
    pub model_name: String,
    pub selection: Selection,
    pub default_selections: Vec<String>,
}

impl FindUniqueBuilder {
    pub fn new(model_name: String, default_selections: Vec<String>) -> Self {
        let name = format!("findUnique{}", model_name);
        Self {
            model_name,
            selection: Selection::with_name(name),
            default_selections,
        }
    }

    pub fn add_arg(&mut self, name: String, val: query_core::ArgumentValue) {
        self.selection.push_argument(name, val);
    }

    pub fn add_selection(&mut self, name: String) {
        self.selection.push_nested_selection(Selection::with_name(name));
    }

    pub async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &PrismaClient) -> Result<T, anyhow::Error> {
        if self.selection.nested_selections().is_empty() {
            for field in self.default_selections {
                self.selection.push_nested_selection(Selection::with_name(field));
            }
        }
        let operation = Operation::Read(self.selection);
        
        let response = client.executor().execute(
            None,
            operation,
            client.query_schema(),
            None,
            EngineProtocol::Json,
        ).await?;

        Ok(serde_json::from_value(serde_json::to_value(&response.data)?)?)
    }
}

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

    pub fn add_arg(&mut self, name: String, val: query_core::ArgumentValue) {
        self.selection.push_argument(name, val);
    }

    pub fn add_selection(&mut self, name: String) {
        self.selection.push_nested_selection(Selection::with_name(name));
    }

    pub async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &PrismaClient) -> Result<T, anyhow::Error> {
        if self.selection.nested_selections().is_empty() {
            for field in self.default_selections {
                self.selection.push_nested_selection(Selection::with_name(field));
            }
        }
        let operation = Operation::Write(self.selection);
        
        let response = client.executor().execute(
            None,
            operation,
            client.query_schema(),
            None,
            EngineProtocol::Json,
        ).await?;

        Ok(serde_json::from_value(serde_json::to_value(&response.data)?)?)
    }
}

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

    pub fn add_arg(&mut self, name: String, val: query_core::ArgumentValue) {
        self.selection.push_argument(name, val);
    }

    pub fn add_selection(&mut self, name: String) {
        self.selection.push_nested_selection(Selection::with_name(name));
    }

    pub async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &PrismaClient) -> Result<T, anyhow::Error> {
        if self.selection.nested_selections().is_empty() {
            for field in self.default_selections {
                self.selection.push_nested_selection(Selection::with_name(field));
            }
        }
        let operation = Operation::Write(self.selection);
        
        let response = client.executor().execute(
            None,
            operation,
            client.query_schema(),
            None,
            EngineProtocol::Json,
        ).await?;

        Ok(serde_json::from_value(serde_json::to_value(&response.data)?)?)
    }
}

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

    pub fn add_arg(&mut self, name: String, val: query_core::ArgumentValue) {
        self.selection.push_argument(name, val);
    }

    pub fn add_selection(&mut self, name: String) {
        self.selection.push_nested_selection(Selection::with_name(name));
    }

    pub async fn exec<T: serde::de::DeserializeOwned>(mut self, client: &PrismaClient) -> Result<T, anyhow::Error> {
        if self.selection.nested_selections().is_empty() {
            for field in self.default_selections {
                self.selection.push_nested_selection(Selection::with_name(field));
            }
        }
        let operation = Operation::Write(self.selection);
        
        let response = client.executor().execute(
            None,
            operation,
            client.query_schema(),
            None,
            EngineProtocol::Json,
        ).await?;

        Ok(serde_json::from_value(serde_json::to_value(&response.data)?)?)
    }
}
