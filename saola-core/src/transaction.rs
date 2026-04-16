/// Transaction support for prisma-core
use query_core::{QueryExecutor, TxId, executor::TransactionOptions, protocol::EngineProtocol};
use std::sync::Arc;

/// Supported transaction isolation levels
///
/// Database support:
/// - PostgreSQL: ReadUncommitted, ReadCommitted, RepeatableRead, Serializable
/// - MySQL: ReadUncommitted, ReadCommitted, RepeatableRead, Serializable
/// - SQL Server: All levels
/// - CockroachDB: Serializable only
/// - SQLite: Serializable only
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum IsolationLevel {
    /// Read Uncommitted - allows dirty reads
    ReadUncommitted,
    /// Read Committed - default for PostgreSQL and SQL Server (prevents dirty reads)
    ReadCommitted,
    /// Repeatable Read - default for MySQL (prevents non-repeatable reads)
    RepeatableRead,
    /// Snapshot - SQL Server only (MVCC-like isolation)
    Snapshot,
    /// Serializable - strictest level, default for CockroachDB and SQLite
    Serializable,
}

impl IsolationLevel {
    /// Convert isolation level to the string format expected by databases
    pub fn as_str(&self) -> &'static str {
        match self {
            IsolationLevel::ReadUncommitted => "ReadUncommitted",
            IsolationLevel::ReadCommitted => "ReadCommitted",
            IsolationLevel::RepeatableRead => "RepeatableRead",
            IsolationLevel::Snapshot => "Snapshot",
            IsolationLevel::Serializable => "Serializable",
        }
    }
}

impl std::fmt::Display for IsolationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<IsolationLevel> for String {
    fn from(level: IsolationLevel) -> Self {
        level.as_str().to_string()
    }
}

/// Configuration for transaction behavior
#[derive(Clone, Debug)]
pub struct TransactionConfig {
    /// Maximum time to wait to acquire a transaction in milliseconds (default: 5000)
    pub max_wait_ms: u64,

    /// Transaction timeout in milliseconds after which it auto-rollbacks (default: 60000)
    pub timeout_ms: u64,

    /// Isolation level (optional, database default used if not specified)
    pub isolation_level: Option<IsolationLevel>,
}

impl Default for TransactionConfig {
    fn default() -> Self {
        Self {
            max_wait_ms: 5000,
            timeout_ms: 60000,
            isolation_level: None,
        }
    }
}

impl TransactionConfig {
    /// Create a new transaction config with custom timeout
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            timeout_ms,
            ..Default::default()
        }
    }

    /// Set max wait time (in ms)
    pub fn max_wait(mut self, ms: u64) -> Self {
        self.max_wait_ms = ms;
        self
    }

    /// Set timeout (in ms)
    pub fn timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    /// Set isolation level
    pub fn isolation_level(mut self, level: IsolationLevel) -> Self {
        self.isolation_level = Some(level);
        self
    }
}

/// Represents an active database transaction
#[derive(Clone)]
pub struct Transaction {
    pub tx_id: TxId,
    pub executor: Arc<dyn QueryExecutor + Send + Sync>,
    pub query_schema: Arc<crate::schema::QuerySchema>,
}

impl Transaction {
    /// Begin a new transaction with default configuration
    pub async fn begin(client: &crate::client::SaolaClient) -> crate::Result<Self> {
        Self::begin_with_config(client, TransactionConfig::default()).await
    }

    /// Begin a new transaction with custom configuration
    pub async fn begin_with_config(
        client: &crate::client::SaolaClient,
        config: TransactionConfig,
    ) -> crate::Result<Self> {
        let tx_id = client
            .executor()
            .start_tx(
                client.query_schema(),
                EngineProtocol::Json,
                TransactionOptions::new(
                    config.max_wait_ms,
                    config.timeout_ms,
                    config.isolation_level.map(|i| i.to_string()),
                )
                .with_new_transaction_id(),
            )
            .await?;

        Ok(Transaction {
            tx_id,
            executor: client.executor().clone(),
            query_schema: client.query_schema().clone(),
        })
    }

    /// Commit the transaction
    pub async fn commit(self) -> crate::Result<()> {
        self.executor
            .commit_tx(self.tx_id)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to commit transaction: {}", e))
    }

    /// Rollback the transaction
    pub async fn rollback(self) -> crate::Result<()> {
        self.executor
            .rollback_tx(self.tx_id)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to rollback transaction: {}", e))
    }
}

/// Trait to make both SaolaClient and Transaction work with execute functions
pub trait QueryExecutorProvider {
    fn executor(&self) -> Arc<dyn QueryExecutor + Send + Sync>;
    fn query_schema(&self) -> Arc<crate::schema::QuerySchema>;
    fn tx_id(&self) -> Option<&TxId>;
}

impl QueryExecutorProvider for crate::client::SaolaClient {
    fn executor(&self) -> Arc<dyn QueryExecutor + Send + Sync> {
        self.executor.clone()
    }

    fn query_schema(&self) -> Arc<crate::schema::QuerySchema> {
        self.query_schema.clone()
    }

    fn tx_id(&self) -> Option<&TxId> {
        None
    }
}

impl QueryExecutorProvider for Transaction {
    fn executor(&self) -> Arc<dyn QueryExecutor + Send + Sync> {
        self.executor.clone()
    }

    fn query_schema(&self) -> Arc<crate::schema::QuerySchema> {
        self.query_schema.clone()
    }

    fn tx_id(&self) -> Option<&TxId> {
        Some(&self.tx_id)
    }
}

/// Helper macro for cleaner transaction syntax without Box::pin
///
/// # Example
/// ```ignore
/// tx!(client, tx, {
///     user().create("alice@example.com".to_string()).exec(tx).await?;
///     Ok(())
/// })?;
///
/// // With custom config:
/// tx_config!(client, tx, TransactionConfig::default().timeout_ms(30000), {
///     user().create("alice@example.com".to_string()).exec(tx).await?;
///     Ok(())
/// })?;
/// ```
#[macro_export]
macro_rules! tx {
    ($client:expr, $tx:ident, $body:expr) => {
        $client.transaction(|$tx| async move { $body }).await
    };
}

/// Helper macro for transactions with custom configuration
#[macro_export]
macro_rules! tx_config {
    ($client:expr, $tx:ident, $config:expr, $body:expr) => {
        $client
            .transaction_with_config($config, |$tx| async move { $body })
            .await
    };
}
