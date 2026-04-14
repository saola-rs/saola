/// PrismaClient - Main entry point for database operations
use psl::parser_database::NoExtensionTypes;
use query_core::{QueryExecutor, executor::InterpretingExecutor};
use sql_query_connector::{FromSource, Mysql, PostgreSql, Sqlite};
use std::sync::Arc;

/// Main database client - initializes connection and schema
pub struct PrismaClient {
    executor: Arc<dyn QueryExecutor + Send + Sync>,
    query_schema: Arc<schema::QuerySchema>,
}

impl PrismaClient {
    /// Create a new Prisma client with schema and database URL
    pub async fn new(schema_str: &str, url: &str) -> crate::Result<Self> {
        let source_file = psl::SourceFile::from(schema_str);
        let validated = psl::validate(source_file, &NoExtensionTypes);

        if !validated.diagnostics.errors().is_empty() {
            anyhow::bail!("Schema validation failed");
        }

        let validated = Arc::new(validated);
        let datasource = validated
            .configuration
            .datasources
            .first()
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

        Ok(PrismaClient { executor, query_schema })
    }

    /// Get a reference to the query executor
    pub fn executor(&self) -> Arc<dyn QueryExecutor + Send + Sync> {
        self.executor.clone()
    }

    /// Get a reference to the query schema
    pub fn query_schema(&self) -> Arc<schema::QuerySchema> {
        self.query_schema.clone()
    }
}
