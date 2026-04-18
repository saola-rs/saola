/// SaolaClient - Main entry point for database operations
use psl::parser_database::NoExtensionTypes;
use query_core::QueryExecutor;
#[cfg(any(feature = "postgresql", feature = "mysql", feature = "sqlite", feature = "mssql", feature = "mongodb"))]
use query_core::executor::InterpretingExecutor;
#[cfg(any(feature = "postgresql", feature = "mysql", feature = "sqlite", feature = "mssql"))]
use sql_query_connector::FromSource;
#[cfg(feature = "postgresql")]
use sql_query_connector::PostgreSql;
#[cfg(feature = "mysql")]
use sql_query_connector::Mysql;
#[cfg(feature = "sqlite")]
use sql_query_connector::Sqlite;
#[cfg(feature = "mongodb")]
use mongodb_query_connector::MongoDb;
use std::sync::Arc;

/// Main database client - initializes connection and schema
pub struct SaolaClient {
    pub executor: Arc<dyn QueryExecutor + Send + Sync>,
    pub query_schema: Arc<schema::QuerySchema>,
}

impl SaolaClient {
    /// Create a new Saola client with schema and database URL
    pub async fn new(schema_str: &str, url: &str) -> crate::Result<Self> {
        let source_file = psl::SourceFile::from(schema_str);
        let validated = psl::validate(source_file, &NoExtensionTypes);

        if !validated.diagnostics.errors().is_empty() {
            anyhow::bail!("Schema validation failed: {:?}", validated.diagnostics.errors());
        }

        let validated = Arc::new(validated);
        let datasource = validated
            .configuration
            .datasources
            .first()
            .ok_or_else(|| anyhow::anyhow!("No datasource found in schema"))?;

        let query_schema = Arc::new(schema::build(validated.clone(), true));

        #[cfg(not(any(feature = "postgresql", feature = "mysql", feature = "sqlite", feature = "mssql", feature = "mongodb")))]
        {
            let _ = (url, query_schema, datasource);
            anyhow::bail!("No database provider feature enabled. Please enable one of: postgresql, mysql, sqlite, mssql, mongodb");
        }

        #[cfg(any(feature = "postgresql", feature = "mysql", feature = "sqlite", feature = "mssql", feature = "mongodb"))]
        {
            let executor: Arc<dyn QueryExecutor + Send + Sync> = match datasource.active_provider {
                #[cfg(feature = "postgresql")]
                "postgresql" | "postgres" => {
                    let connector = PostgreSql::from_source(datasource, url, psl::PreviewFeatures::empty(), false).await?;
                    Arc::new(InterpretingExecutor::new(connector, false))
                }
                #[cfg(feature = "mysql")]
                "mysql" => {
                    let connector = Mysql::from_source(datasource, url, psl::PreviewFeatures::empty(), false).await?;
                    Arc::new(InterpretingExecutor::new(connector, false))
                }
                #[cfg(feature = "sqlite")]
                "sqlite" => {
                    let connector = Sqlite::from_source(datasource, url, psl::PreviewFeatures::empty(), false).await?;
                    Arc::new(InterpretingExecutor::new(connector, false))
                }
                #[cfg(feature = "mssql")]
                "sqlserver" => {
                    let connector = sql_query_connector::Mssql::from_source(datasource, url, psl::PreviewFeatures::empty(), false).await?;
                    Arc::new(InterpretingExecutor::new(connector, false))
                }
                #[cfg(feature = "mongodb")]
                "mongodb" => {
                    let connector = MongoDb::new(datasource, url).await?;
                    Arc::new(InterpretingExecutor::new(connector, false))
                }
                _ => anyhow::bail!("Unsupported or disabled provider: {}. Check your saola-core features.", datasource.active_provider),
            };

            Ok(SaolaClient { executor, query_schema })
        }
    }

    /// Get a reference to the query executor
    pub fn executor(&self) -> Arc<dyn QueryExecutor + Send + Sync> {
        self.executor.clone()
    }

    /// Get a reference to the query schema
    pub fn query_schema(&self) -> Arc<schema::QuerySchema> {
        self.query_schema.clone()
    }

    /// Execute a closure within a transaction with default configuration
    ///
    /// # Simple API - Use the `tx!` macro for cleanest syntax
    ///
    /// # Example:
    /// ```ignore
    /// client.transaction(|tx| async move {
    ///     user().create("alice@example.com".to_string()).exec(&tx).await?;
    ///     Ok(())
    /// }).await?;
    /// ```
    pub async fn transaction<F, Fut, T>(&self, callback: F) -> crate::Result<T>
    where
        F: FnOnce(crate::transaction::Transaction) -> Fut,
        Fut: std::future::Future<Output = crate::Result<T>>,
    {
        self.transaction_with_config(crate::transaction::TransactionConfig::default(), callback)
            .await
    }

    /// Execute a closure within a transaction with custom configuration
    pub async fn transaction_with_config<F, Fut, T>(
        &self,
        config: crate::transaction::TransactionConfig,
        callback: F,
    ) -> crate::Result<T>
    where
        F: FnOnce(crate::transaction::Transaction) -> Fut,
        Fut: std::future::Future<Output = crate::Result<T>>,
    {
        let tx = crate::transaction::Transaction::begin_with_config(self, config).await?;
        match callback(tx.clone()).await {
            Ok(result) => {
                tx.commit().await?;
                Ok(result)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                Err(e)
            }
        }
    }
}
