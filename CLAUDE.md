# Prisma for Rust - Project Guide

## Project Overview

This is a **Prisma ORM implementation for Rust** - a work-in-progress project building a type-safe database client generator for Rust. It leverages code from the official Prisma engines repository (kept in `/prisma-engines` folder) but integrates copies of needed components directly into the root workspace.

**Key Philosophy:** Maximize reuse of Prisma engines code while building Rust-native abstractions and client generators.

**Status:** Active development with incremental phases (Phase 3 in progress)

---

## Workspace Structure

### Core Architecture
```
Root Workspace Members:
├── psl-official/              # Official PSL (Prisma Schema Language) parser
├── prisma-schema/             # Prisma schema parsing layer
├── prisma-macros/             # Proc-macro for generating Prisma structs
├── prisma_core/               # Core PrismaClient implementation
├── prisma-cli/                # CLI tool for code generation
├── example-app/               # Example usage demonstrating the full flow
├── quaint/                    # Database abstraction layer (from engines)
├── query-engine/              # Query execution engine
│   ├── core/                  # Query execution core
│   ├── dmmf/                  # Data Model Meta Format
│   ├── schema/                # Query schema building
│   ├── query-structure/       # Query operation structures
│   ├── query-builders/        # SQL/MongoDB query builders
│   └── connectors/            # Database connectors (SQL, MongoDB)
├── schema-engine/             # Schema management engine
│   ├── core/                  # Core schema operations
│   ├── connectors/            # Database connectors for schema
│   ├── commands/              # Schema commands (migrate, introspect)
│   ├── json-rpc-api/          # JSON-RPC API
│   ├── datamodel-renderer/    # Schema formatting
│   ├── sql-schema-describer/  # SQL schema introspection
│   └── mongodb-schema-describer/  # MongoDB schema introspection
└── libs/                      # Utility libraries
    ├── prisma-value/          # PrismaValue type system
    ├── query-engine-common/   # Shared query engine utilities
    ├── metrics/               # Observability/metrics
    ├── telemetry/             # Tracing & telemetry
    ├── user-facing-errors/    # User-friendly error messages
    └── [other utilities]      # Build, testing, MongoDB client libs
```

---

## Key Components & Their Responsibilities

### 1. **prisma_core** - Main Client Entry Point
**File:** `prisma_core/src/lib.rs`

Core data structures and client initialization:
- **`PrismaClient`**: Main struct that holds executor and query schema, initialized with schema string and database URL
- **Builder Pattern Structs**: `FindManyBuilder`, `FindUniqueBuilder`, `CreateBuilder`, `UpdateBuilder`, `DeleteBuilder`
- **Database Support**: PostgreSQL, MySQL, SQLite (connectors auto-selected based on provider)

```rust
pub struct PrismaClient {
    executor: Arc<dyn QueryExecutor + Send + Sync>,
    query_schema: Arc<schema::QuerySchema>,
}
```

### 2. **prisma-cli** - Code Generation Tool
**File:** `prisma-cli/src/main.rs`

Generates Rust client code from Prisma schemas:
- Reads `schema.prisma` file
- Validates schema using PSL parser
- Generates type-safe Rust code with builders for each model
- Outputs to `src/prisma.rs` (default)

```bash
prisma-cli --schema schema.prisma --output src/prisma.rs
```

### 3. **prisma-macros** - Compile-Time Code Generation
**File:** `prisma-macros/src/lib.rs`

Proc-macro `#[prisma_model]` for attribute-based code generation:
- Analyzes struct fields for `#[prisma(...)]` attributes (name, relation, id, unique)
- Generates filter methods, select builders, include builders, data builders
- Creates builder structs for all query operations
- Extracts relation metadata and scalar field names

Example usage:
```rust
#[prisma_model]
struct User {
    #[prisma(id)]
    id: String,
    email: String,
    #[prisma(name = "displayName")]
    display_name: Option<String>,
    #[prisma(relation)]
    posts: Vec<Post>,
}
```

### 4. **query-engine/** - Query Execution Layer
Adapted from Prisma engines, handles query interpretation and execution:

- **core**: Query executor trait (`QueryExecutor`), `Operation` enum (Read/Write), `EngineProtocol` (Json/Graphql)
- **schema**: Builds `QuerySchema` from validated PSL schema
- **connectors**: Database-specific connectors (SQL, MongoDB)
- **query-builders**: Converts high-level queries to database-specific SQL/query syntax
- **dmmf**: Data Model Meta Format for schema introspection

### 5. **schema-engine/** - Schema Management
Handles schema operations (introspection, migration validation):

- **core**: Schema core operations
- **connectors**: Database connectors for schema operations
- **datamodel-renderer**: Formats/renders Prisma schema
- **sql-schema-describer / mongodb-schema-describer**: Database introspection

### 6. **quaint** - Database Abstraction
Low-level database access layer with connection pooling and query execution for all supported databases.

### 7. **psl-official/** - Prisma Schema Language Parser
Official PSL parser (from Prisma engines), located in `psl-official/`:
- **psl-core**: Core parser logic with database feature flags
- **schema-ast**: Abstract Syntax Tree definitions
- **diagnostics**: Parser diagnostics/errors
- **parser-database**: Database model abstraction

---

## Development Workflow

### Example Application Flow
**File:** `example-app/src/main.rs` demonstrates:

1. **Initialize Client**
   ```rust
   let schema_str = include_str!("../schema.prisma");
   let client = PrismaClient::new(schema_str, "file:./dev.db").await?;
   ```

2. **CRUD Operations** using generated code or builders
   ```rust
   let user = client::user().create(...).exec(&client).await?;
   let users = client::user().find_many().where_clause(...).exec(&client).await?;
   ```

3. **Type Safety**: Generated code ensures compile-time type checking

### Recent Development Phases
- **Phase 1**: End-to-end code generation infrastructure
- **Phase 2**: Type-aware code generation with proper Select/Include separation
- **Phase 3**: Enum-aware compile-time type safety + modular codegen refactoring
- **Phase 3 part 2**: Return Type Generation with Serde Support

Current focus: Improving code generation to handle complex scenarios (enums, relations, return types).

---

## Key Patterns & Conventions

### Schema Attributes
```rust
#[prisma(name = "displayName")]      // Map to Prisma schema field name
#[prisma(id)]                         // Mark as ID (implies unique)
#[prisma(unique)]                     // Mark as unique field
#[prisma(relation)]                   // Mark as relation field
#[prisma(name = "fieldName", ...)]    // Can combine attributes
```

### Builder Pattern Usage
All operations use builder pattern for composability:
```rust
client::user()
    .find_many()
    .where_clause(|w| { w.email().eq("..."); })
    .select(|s| { s.id().email(); })
    .include(|i| { i.posts(...); })
    .exec(&client)
    .await?
```

### Selection System
- Uses `Selection` struct from query-engine with nested selections
- Differentiates between fields to return (`select`) and relations to include (`include`)
- Default selections include all scalar fields if not explicitly specified

---

## Important Notes

### prisma-engines Folder
- ⚠️ **Separate workspace** - don't explore or modify directly
- Used as reference only; needed code is copied to root
- When copying components from engines:
  1. Copy the necessary crate to root
  2. Update workspace `Cargo.toml` members
  3. Add workspace dependencies
  4. Remove prisma-engines path reference

### Database Support
Currently supports:
- PostgreSQL (with native driver via `postgres-native-tls`)
- MySQL (with native driver via `mysql_async`)
- SQLite (with rusqlite)
- MongoDB (partial support)

### Key Dependencies
- **psl**: Official PSL parser
- **tokio**: Async runtime
- **serde/serde_json**: Serialization
- **sqlx, tiberius, postgres**: Database drivers
- **hyper, reqwest**: HTTP clients
- **tracing**: Observability

---

## Common Development Tasks

### Add a New Schema Feature
1. Update PSL parser in `psl-official/` if needed
2. Modify `query-engine/schema/` to handle new schema constructs
3. Update code generator in `prisma-cli/` to generate for new features
4. Add integration tests in `example-app/`

### Fix Database Connectivity
Check `query-engine/connectors/$DB-query-connector` for database-specific logic.

### Improve Code Generation
1. Modify `prisma-cli/src/codegen` for overall structure
2. Adjust `prisma-macros/src/lib.rs` for attribute handling
3. Test with `example-app/`

### Debug Query Execution
1. Check `query-engine/core/` for execution logic
2. Verify `Selection` building in operation code
3. Check connector implementation in `query-engine/connectors/`

---

## Build & Test

```bash
# Full workspace build
cargo build

# Run example app
cd example-app && cargo run

# Generate code via CLI
cargo run --bin prisma-cli -- --schema schema.prisma --output src/prisma.rs

# Test workspace
cargo test --all
```

---

## File Locations & Conventions

- **Workspace config**: `/Cargo.toml`
- **Example schema**: `/example-app/schema.prisma`
- **Generated client code**: `/example-app/src/prisma.rs` (auto-generated)
- **Main client lib**: `/prisma_core/src/lib.rs`
- **Test database**: `/dev.db` (SQLite)

---

## Current Focus Areas
- Refining code generation for complex types and relations
- Improving type safety and compile-time guarantees
- Enhancing error messages for better developer experience
- Supporting more database providers seamlessly
