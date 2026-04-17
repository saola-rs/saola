# Saola ORM - Clean Architecture Demo 🚀

## Overview

This example application demonstrates **Saola** (Prisma for Rust) with a real-world blog platform scenario. The project now has a clean separation between demo and comprehensive testing.

## Project Structure

```
example-app/
├── src/
│   ├── main.rs        🎯 Clean, simple demo (210 lines)
│   └── tests.rs       🧪 Comprehensive test suite (200 lines)
├── schema.prisma      📋 Enhanced schema (blog platform)
├── BENCHMARKS.md      📊 Performance benchmarking docs
└── README.md          (this file)
```

## Quick Start

### 1. Run the Clean Demo

```bash
cargo run -p example-app
```

**Output:** Real-world blog scenarios showing:
- ✓ Nested creates (user + profile + posts in one call)
- ✓ Relation filters (users → posts filtering)
- ✓ Complex OR queries
- ✓ Sorting & pagination
- ✓ Analytics (aggregations)
- ✓ Type-safe includes
- ✓ Field projections (select_as macro)

### 2. Run Comprehensive Tests

```bash
cargo test --lib tests --
```

Tests include:
- `test_create_with_nested_relations` - Hierarchical data creation
- `test_complex_filtering_with_relations` - Relation-based queries
- `test_bulk_operations` - Mass create/update/delete
- `test_aggregations` - Count & statistics
- `test_pagination_and_ordering` - List operations
- `test_CRUD_operations` - Full CRUD cycle

## Key Features Demonstrated

### 📝 Nested Creates (Real-world Blog Example)

Create a user with profile AND multiple posts in **one call**:

```rust
user()
    .create("alice@blog.com".to_string())
    .data(|d| {
        d.name("Alice Writer".to_string());
        d.profile(|prof| {        // Create profile
            prof.create(|p| {
                p.bio(Some("Tech writer".to_string()));
            });
        });
        d.posts(|p| {             // Create posts
            p.create("My First Post".to_string(), |post| {
                post.content("...".to_string());
            });
        });
    })
    .include(|u| u.posts())
    .include(|u| u.profile())
    .exec(&client)
    .await?;
```

### 🔍 Relation Filters (Complex Queries)

Find published posts by active users:

```rust
post()
    .find_many()
    .where_clause(|w| {
        w.published().eq(true);
        w.user().some(|u| {      // Filter by relation condition
            u.is_active().eq(true);
        });
    })
    .include(|i| i.user())
    .order_by(|o| o.created_at(SortOrder::Desc))
    .exec(&client)
    .await?
```

### 📊 Analytics (Aggregations)

```rust
// Count published posts
let total = post()
    .count()
    .where_clause(|w| w.published().eq(true))
    .exec(&client)
    .await?;

// Count active writers
let writers = user()
    .count()
    .where_clause(|w| {
        w.is_active().eq(true);
        w.posts().some(|p| p.published().eq(true));
    })
    .exec(&client)
    .await?;
```

### 🎯 Field Projections (Zero Boilerplate)

Select only the fields you need using `select_as!` macro:

```rust
post()
    .find_many()
    .select_as(saola::select_as!({
        id: String,
        title: String,
        views: i32
    }))
    .where_clause(|w| w.published().eq(true))
    .exec(&client)
    .await?
```

## Enhanced Schema

The schema has been enhanced to represent a **real blog platform**:

```prisma
model User {
  id        String
  email     String   @unique
  name      String
  role      Role     @default(USER)
  isActive  Boolean  @default(true)
  posts     Post[]
  profile   Profile?
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
}

model Post {
  id        String
  title     String
  content   String
  status    PostStatus @default(DRAFT)
  published Boolean
  views     Int
  userId    String
  user      User
  comments  Comment[]
  createdAt DateTime
  updatedAt DateTime
}

model Profile {
  id      String
  bio     String?
  avatar  String?
  website String?
  userId  String  @unique
  user    User
}

model Comment {
  id      String
  text    String
  postId  String
  post    Post
  createdAt DateTime
}

model Tag {
  id    String
  name  String @unique
  posts Post[]  // Many-to-many example
}
```

## Why This Architecture?

### ✨ Clean Separation

- **main.rs** - Simple, readable demo showing real-world usage (~210 lines)
- **tests.rs** - Comprehensive tests for all features (~200 lines)
- **Schema-driven** - All code is generated from schema, zero manual setup

### 🎯 Shows Real-World Usage

Main demo scenarios:
1. Creating complex nested structures
2. Filtering with relations
3. Analytics queries
4. Pagination & sorting
5. Field projections

### 🧪 Comprehensive Testing

Test coverage for:
- Nested relations
- Complex filtering
- Bulk operations
- Aggregations
- Pagination
- CRUD cycle

## How It Shows ORM Power

### Before (Manual ORM)
```rust
// 10+ lines to create user + profile + posts
let user = create_user(...).await?;
let profile = create_profile(..., user.id).await?;
let post1 = create_post(..., user.id).await?;
let post2 = create_post(..., user.id).await?;
// Manual joins to fetch together...
```

### After (Saola)
```rust
// 1 fluent chain - create + include everything
user()
    .create(...)
    .data(|d| {
        d.profile(|p| p.create(...));
        d.posts(|posts| {
            posts.create(...);
            posts.create(...);
        });
    })
    .include(|u| u.posts())
    .include(|u| u.profile())
    .exec(&client)
    .await?
```

## Development Commands

```bash
# Run demo
cargo run -p example-app

# Run all tests
cargo test --lib tests

# Run specific test
cargo test --lib tests::test_create_with_nested_relations

# Run with output
cargo test --lib tests -- --nocapture

# Build
cargo build -p example-app

# Check for compile errors
cargo check -p example-app
```

## Key Principles

✅ **Zero Boilerplate** - Schema is single source of truth
✅ **Type Safe** - All queries validated at compile time
✅ **Composable** - Builders can be chained intuitively
✅ **Real-world** - Demos actual use cases, not toy examples
✅ **Production Ready** - Handles transactions, aggregations, bulk ops

## What Makes This ORM Special

1. **Nested Creates** - Create entire object graphs in one operation
2. **Relation Filters** - Filter by conditions on related models
3. **Zero Code Generation** - Pure macros, no CLI tools required
4. **Type Inference** - Builders automatically track return types
5. **Compile-time Validation** - Wrong queries = compile error
6. **Natural Syntax** - Reads like TypeScript/JavaScript

## Next Steps

### To Extend This Example

1. **Add Authentication** - `User` model with password, roles
2. **Add Comments** - Already in schema, add nested comment creation
3. **Add Tags** - Use many-to-many relations
4. **Add Search** - Complex filtering on post content
5. **Add Notifications** - Track user follows & post interactions

### To Learn More

- Check `BENCHMARKS.md` for performance insights
- Read `CLAUDE.md` for architecture details
- Run tests to see all features in action

## Files Overview

| File | Purpose | Lines |
|------|---------|-------|
| `main.rs` | Real-world demo, clean & simple | 210 |
| `tests.rs` | Comprehensive feature tests | 200 |
| `schema.prisma` | Blog platform data model | 70 |
| `BENCHMARKS.md` | Performance testing guide | - |

---

**Built with Saola - The Rust ORM that feels like JavaScript** 🚀
