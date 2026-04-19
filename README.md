# Saola ORM 🦀

A **production-ready Rust ORM** that brings Prisma's developer experience to Rust with compile-time type safety, zero boilerplate, and macro-based code generation.

Reuses Prisma's query execution engine while providing Rust-native macros for code generation.

**Architecture**: Pure macro-based. Schema file → Rust types → Type-safe builders → Database queries.

## ✨ What Makes Saola Special?

- **Zero Boilerplate** - Just `init!("schema.prisma")` and start querying
- **100% Type Safe** - All query operators validated at compile-time
- **Production Features** - Transactions with isolation levels, bulk ops, aggregations
- **Schema-Driven** - Prisma schema is your single source of truth
- **Automatic Client** - Database client auto-initialized from schema datasource
- **Advanced Includes** - Include with filtering (`_with`), as custom types (`_as`)
- **Field Projections** - Zero-boilerplate `select_as!` macro with optional fields
- **Real Queries** - Uses Prisma's battle-tested query execution engine
- **Fluent API** - Clean, chainable builder syntax that feels natural

## 🚀 Quick Start

### 1. Create a Schema

```prisma
// schema.prisma
datasource db {
  provider = "sqlite"
  url      = "file:./dev.db"
}

model User {
  id       String    @id @default(cuid())
  email    String    @unique
  name     String
  isActive Boolean   @default(true)
  posts    Post[]
  profile  Profile?
}

model Post {
  id        String    @id @default(cuid())
  title     String
  published Boolean   @default(false)
  userId    String
  user      User      @relation(fields: [userId], references: [id])
  createdAt DateTime  @default(now())
}

model Profile {
  id     String @id @default(cuid())
  bio    String?
  userId String @unique
  user   User   @relation(fields: [userId], references: [id])
}
```

### 2. Initialize & Query
```bash
cargo add saola-core --features sqlite
cargo add saola-macros --features sqlite
```

```rust
use saola_macros::init;
use saola_core::prelude::*;

init!("schema.prisma");

#[tokio::main]
async fn main() -> Result<()> {
    let client = saola::client().await?;

    // Create user with profile and posts in one call
    let user = saola::user()
        .create("alice@example.com".to_string(), "Alice".into())
        .data(|d| {
            d.profile(|p| {
                p.create(|prof| {
                    prof.bio("Tech writer".to_string());
                });
            });
            d.posts(|posts| {
                posts.create("My First Post".to_string(), |post| {
                    post.published(true);
                });
            });
        })
        .include(|u| u.posts())
        .include(|u| u.profile())
        .exec(&client)
        .await?;

    println!("userId: {}", user.id);

    // Query with relation filtering
    let active_writers = saola::user()
        .find_many()
        .where_clause(|w| {
            w.is_active().eq(true);
            w.posts().some(|p| {
                p.published().eq(true);
            });
        })
        .include(|u| u.profile())
        .exec(&client)
        .await?;

    println!("active_writers: {:?}", active_writers);
    Ok(())
}
```
### 3. Run Migrations
```bash
# Using Prisma Migrate CLI
npx prisma@6.19 migrate dev --name init --skip-generate

# using bunx
bunx prisma@6.19 migrate dev --name init --skip-generate
```

### 4. Run

```bash
cargo run
```

## 🏗️ Architecture: Two-Level Macro System

### Level 1: `init!()` - Schema to Rust Types

Compile-time macro that:
1. **Reads** your `schema.prisma` file
2. **Parses** with official PSL parser
3. **Walks** models/enums (generates Rust types with `#[saola_model]`)
4. **Extracts** datasource → auto-generates `saola::client()` function
5. **Wraps** everything in `pub mod saola { ... }`

### Level 2: `#[saola_model]` - Builders & Query Factories

Attribute macro generating:
- **Builders**: `{Model}WhereBuilder`, `UniqueWhereBuilder`, `SelectBuilder`, `IncludeBuilder`, `DataBuilder`
- **Query Types**: `{Model}ReadBuilder`, `WriteBuilder`, `CountBuilder`, `AggregateBuilder`, `GroupByBuilder`
- **Query Factory**: `{model}()` function with all 17 operations

**Result**: `saola::user().create(...)` compiles to generated implementation.

## ✅ All Features (17 Operations + Advanced)

### CRUD Operations (14 Total)

**Read** (5 variants):
- `find_many()` → `Vec<T>`
- `find_unique()` → `Option<T>`
- `find_first()` → `Option<T>`
- `find_unique_or_throw()` → `T` (throws if not found)
- `find_first_or_throw()` → `T` (throws if not found)

**Write** (4 variants):
- `create()` - Single record with nested relations
- `update()` - Update with nested writes
- `delete()` - Delete record
- `upsert()` - Create or update

**Bulk** (5 variants):
- `create_many()` → count
- `create_many_and_return()` → `Vec<T>` ✅
- `update_many()` → count
- `update_many_and_return()` → `Vec<T>` ✅
- `delete_many()` → count

**Aggregation** (3 variants):
- `count()` - With filters
- `aggregate()` - Sum, avg, min, max
- `group_by()` - Group and aggregate

### Advanced Query Features

#### Type-Safe Filters by Field Type
- **StringFilter**: `contains`, `starts_with`, `ends_with`, `in_list`, `equals`, `gt/gte/lt/lte`
- **IntFilter**: `gt/gte/lt/lte`, `in_list`, `equals`
- **FloatFilter**: `gt/gte/lt/lte`, `in_list`, `equals`
- **BoolFilter**: `equals`
- **EnumFilter**: `equals`, `in_list`
- **DateTimeFilter**: `gt/gte/lt/lte`, `in_list`, `equals`
- **RelationFilter**: `is`, `is_not`

All operators **validated at compile-time** via trait implementations.

#### Logical & Relation Operators
- **AND** - Implicit: multiple where conditions
- **OR** - Explicit: `.or()` builder method
- **NOT** - `.not()` on field filters
- **Relation Filters**: `.some()`, `.every()`, `.none()`

**Example**:
```rust
user()
    .find_many()
    .where_clause(|w| {
        w.is_active().eq(true);
        w.posts().some(|p| {
            p.published().eq(true);
            p.created_at().gt(DateTime);
        });
    })
```

#### Advanced Include Patterns
1. **Basic Include** - All fields of relation:
   ```rust
   .include(|u| u.posts())
   ```

2. **Include with Filtering** (`_with` suffix) - Custom where clause:
   ```rust
   .include(|u| u.posts_with(|p| p.published().eq(true)))
   ```

3. **Include as Custom Type** (`_as` suffix) - Field projection:
   ```rust
   .include(|u| u.profile_as(saola::select_as!({
       bio: String,
       user: {
           email: String
       }
    })))
   ```

#### Field Projections with `select_as!` Macro
Zero-boilerplate field selection with optional field support:

```rust
post()
    .find_many()
    .select_as(saola::select_as!({
        id: String,
        title: String,
        views: i32,
        user: {
            email: String,
            name: String,
            profile?: {              // Optional nested relation
                bio?: String
            }
        }
    }))
    .where_clause(|w| w.published().eq(true))
    .exec(&client)
    .await?
```

#### Nested Operations
- **Nested Create**: Create entire object graphs
- **Nested Update**: Update related records
- **Nested Upsert**: Create or update relations
- **Nested Connect/Disconnect**: Link/unlink relations

### Transactions ✅

Production-ready transactions with isolation level control:

```rust
let transaction = client
    .transaction_begin(TransactionConfig::new(60000)
        .isolation_level(IsolationLevel::ReadCommitted)
    )
    .await?;

// Use transaction like client
let user = db::user()
    .create("alice@example.com".to_string())
    .exec(&transaction)
    .await?;

transaction.commit().await?;

// or
let result = client.transaction(|tx|  async move {
    let user = db::user()
        .create("alice@example.com".to_string())
        .exec(&tx)
        .await?;
    Ok(user)
})
.await?;

```

**Isolation Levels Supported**:
- `ReadUncommitted` - Lowest isolation
- `ReadCommitted` - PostgreSQL/SQL Server default
- `RepeatableRead` - MySQL default
- `Serializable` - CockroachDB/SQLite default
- `Snapshot` - SQL Server only

### Pagination & Sorting
- `.take(n)` - Limit results
- `.skip(n)` - Offset
- `.order_by(|o| o.field(SortOrder::Desc))` - Sort ascending/descending

### Enums & Serialization
- ✅ Full enum support with Display + PrismaValue conversion
- ✅ Serde serialization/deserialization
- ✅ Type-safe enum filters

### Supported Databases
- PostgreSQL ✅
- MySQL ✅
- SQLite ✅
- MSSQL ✅
- MongoDB ✅ 

## 📁 Project Structure

```
.
├── saola-core/                      # Runtime & traits
│   ├── src/
│   │   ├── builder.rs               # Executable, Filterable, Selectable traits
│   │   ├── read.rs                  # ReadBuilder for find operations
│   │   ├── write.rs                 # WriteBuilder for create/update/delete
│   │   ├── aggregate.rs             # Count, Aggregate, GroupBy
│   │   ├── transaction.rs           # Transaction with isolation levels
│   │   ├── filters.rs               # Type-safe filter system
│   │   ├── filter_builders.rs       # Where/Select/Include/Data builders
│   │   ├── prelude.rs               # Public API
│   │   └── client.rs                # PrismaClient init
│   └── Cargo.toml
│
├── saola-macros/                    # Code generation
│   ├── src/
│   │   ├── lib.rs                   # init!(), #[saola_model], select_as!
│   │   ├── model_gen.rs             # Schema → Rust types
│   │   ├── builder_gen.rs           # Where/Select/Include generators
│   │   ├── wrapper_gen.rs           # Thin wrapper builders
│   │   ├── query_gen.rs             # Query factories & operations
│   │   ├── model_analysis.rs        # Field metadata extraction
│   │   ├── codegen_orchestrator.rs  # Coordination logic
│   │   └── utils.rs                 # Helpers
│   └── Cargo.toml                   
│
├── psl-official/                    # Official PSL parser
├── query-engine/                    # Official query engine
└── schema-engine/                   # Official schema engine

# Root
Cargo.toml                            # Workspace config
README.md                             # This file
```

## 🔑 Key Architecture Decisions

1. **Two-level macros** - Schema parsing separate from builder generation
2. **Official PSL parser** - Accurate model/enum extraction
3. **Compile-time type safety** - Filter operators validated via traits
4. **Thin wrapper builders** - Minimal overhead delegation pattern
5. **No CLI code generation** - Pure compile-time macros
6. **Automatic client** - `saola::client()` from schema datasource
7. **QueryExecutorProvider trait** - Unified client/transaction interface

## 🎯 Design Philosophy

✅ **Single source of truth** - Schema file drives everything
✅ **Zero boilerplate** - Just `init!()` and write queries
✅ **Compile-time safety** - Wrong code = compile error
✅ **Production-ready** - Transactions, bulk ops, aggregations
✅ **Clean DX** - Fluent API, type inference, natural syntax

## 🤝 Contributing

Contributions welcome! Start with:

1. **New operations** - `saola-macros/src/query_gen.rs`
2. **Filter types** - `saola-core/src/filters.rs`
3. **Builders** - `saola-macros/src/builder_gen.rs`

## 📝 License

Apache License. See [LICENSE](LICENSE) for details.

## 🔗 Links

- **Official Prisma**: https://www.prisma.io
- **Prisma Docs**: https://www.prisma.io/docs

---

**Built with ❤️ to bring the Prisma DX to Rust** 🚀
