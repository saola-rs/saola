# Saola ORM

Saola is a Rust ORM that provides a Prisma-like developer experience with compile-time type safety, minimal boilerplate, and macro-based code generation. It leverages Prisma's query execution engine while providing Rust-native macros for schema-driven code generation.

## Key Features

- **Zero Boilerplate**: Initialize your entire database client with a single `init!("schema.prisma")` call.
- **Type Safety**: All query operators and relations are validated at compile-time.
- **Schema-Driven**: The Prisma schema serves as the single source of truth for your data model.
- **Production Ready**: Supports transactions with isolation levels, bulk operations, and complex aggregations.
- **Field Projections**: Advanced field selection using the `select_as!` macro.
- **Battle-Tested Engine**: Uses the official Prisma query engine for reliable database interactions.

## Quick Start

### 1. Database Schema

Define your models in a `schema.prisma` file:

```prisma
datasource db {
  provider = "sqlite"
  url      = "file:./dev.db"
}

model User {
  id       String    @id @default(cuid())
  email    String    @unique
  name     String
  posts    Post[]
}

model Post {
  id        String    @id @default(cuid())
  title     String
  published Boolean   @default(false)
  userId    String
  user      User      @relation(fields: [userId], references: [id])
}
```

### 2. Implementation

Add the dependencies to your `Cargo.toml`:

```toml
[dependencies]
saola-core = { version = "0.1", features = ["sqlite"] }
saola-macros = { version = "0.1", features = ["sqlite"] }
tokio = { version = "1", features = ["full"] }
```

Initialize the client and perform queries:

```rust
use saola_macros::init;
use saola_core::prelude::*;

init!("schema.prisma");

#[tokio::main]
async fn main() -> Result<()> {
    let client = saola::client().await?;

    // Create a user with nested relations
    let user = saola::user()
        .create("alice@example.com".to_string(), "Alice".into())
        .data(|d| {
            d.posts(|posts| {
                posts.create("My First Post".to_string(), |post| {
                    post.published(true);
                });
            });
        })
        .include(|u| u.posts())
        .exec(&client)
        .await?;

    // Query with filtering and relations
    let active_users = saola::user()
        .find_many()
        .where_clause(|w| {
            w.posts().some(|p| { p.published().eq(true); });
        })
        .exec(&client)
        .await?;

    Ok(())
}
```

### 3. Migrations

Use the Prisma CLI to manage your database schema:

```bash
npx prisma migrate dev --name init --skip-generate
```

## Architecture

Saola utilizes a two-level macro system to bridge the gap between the Prisma schema and Rust's type system.

### Level 1: Global Initialization
The `init!()` macro parses the `schema.prisma` file using the official PSL (Prisma Schema Language) parser. It generates the base Rust types for every model and enum, and sets up the database client based on the defined datasource.

### Level 2: Model-Specific Generation
The `#[saola_model]` attribute macro (internally invoked by `init!`) generates:
- **Builders**: Type-safe structures for `Where`, `Select`, `Include`, and `Data` clauses.
- **Query Factories**: Methods like `find_many()`, `create()`, and `aggregate()` that return specialized builders for database operations.

## Supported Operations

Saola supports the full range of Prisma operations:

- **Read**: `find_many`, `find_unique`, `find_first`, and their `_or_throw` variants.
- **Write**: `create`, `update`, `upsert`, `delete`.
- **Bulk**: `create_many`, `update_many`, `delete_many`.
- **Aggregation**: `count`, `aggregate`, `group_by`.
- **Transactions**: Full support for interactive transactions with configurable isolation levels (ReadUncommitted, ReadCommitted, RepeatableRead, Serializable, Snapshot).

## Advanced Querying

### Filter System
Field-specific filters are validated at compile-time:
- **String**: `contains`, `starts_with`, `ends_with`, `equals`, etc.
- **Numeric**: `gt`, `gte`, `lt`, `lte`, `in_list`.
- **Relations**: `some`, `every`, `none`, `is`, `is_not`.

### Projections and Includes
- **Basic Include**: Fetch all fields of a related model.
- **Filtered Include**: Apply where clauses to included relations (e.g., `posts_with`).
- **Custom Projections**: Use `select_as!` for zero-boilerplate selection into custom structures, including nested relations.

## Project Structure

- `saola-core`: Runtime logic, traits, and the type-safe filter system.
- `saola-macros`: The core code generation engine (parsing, model analysis, and builder generation).
- `psl-official`: Integration with the official Prisma Schema Language parser.
- `query-engine`: Integration with the Prisma query execution engine.

## License

Apache License 2.0. See [LICENSE](LICENSE) for details.
