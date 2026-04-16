# Rust ORM Implementation Status

This document tracks the features implemented in the native Rust Prisma ORM compared to the official TypeScript client.

## Model Methods

| Feature | Status | Rust Method | Notes |
| :--- | :--- | :--- | :--- |
| `findMany` | ✅ Supported | `.find_many()` | Full support for filtering and pagination. |
| `findUnique` | ✅ Supported | `.find_unique()` | Returns `Option<T>`. |
| `findUniqueOrThrow` | ✅ Supported | `.find_unique_or_throw()` | Returns `T`, panics/errors if not found. |
| `findFirst` | ✅ Supported | `.find_first()` | Returns `Option<T>`. |
| `findFirstOrThrow` | ✅ Supported | `.find_first_or_throw()` | Returns `T`. |
| `create` | ✅ Supported | `.create(...)` | Required fields as args. Supports **Nested Writes** via `.data()`. |
| `update` | ✅ Supported | `.update()` | Requires `.where_clause()` with unique filter. |
| `delete` | ✅ Supported | `.delete()` | Requires `.where_clause()` with unique filter. |
| `upsert` | ❌ Not Supported | - | Planned. |
| `createMany` | ❌ Not Supported | - | Planned. |
| `updateMany` | ❌ Not Supported | - | Planned. |
| `deleteMany` | ❌ Not Supported | - | Planned. |
| `count` | ✅ Supported | `.count()` | Basic count implemented. |
| `aggregate` | ✅ Supported | `.aggregate()` | Base structure exists. |
| `groupBy` | ✅ Supported | `.group_by()` | Base structure exists. |

## Query Arguments

| Argument | Status | Rust API | Notes |
| :--- | :--- | :--- | :--- |
| `where` | ✅ Supported | `.where_clause(|w| ...)` | Supports AND, OR, NOT and **Nested Relation Filters** (some, every, none, is, is_not). |
| `orderBy` | ❌ Not Supported | - | Planned. |
| `take` | ✅ Supported | `.take(n)` | Available on ManyReadBuilder and nested includes. |
| `skip` | ✅ Supported | `.skip(n)` | Available on ManyReadBuilder and nested includes. |
| `select` | ✅ Supported | `.select(|s| ...)` | Returns flattened `serde_json::Value`. |
| `include` | ✅ Supported | `.include(|i| ...)` | **Advanced Implementation**: Supports nested, filtered, and paginated inclusions with full type safety. |
| `data` | ✅ Supported | `.data(|d| ...)` | Supports **Nested Relation Writes** (create, connect, disconnect). |
| `distinct` | ❌ Not Supported | - | |
| `cursor` | ❌ Not Supported | - | |

## Special Rust Features

### 1. Ad-hoc Structural Typing (`select_as!`)
The `select_as!` macro allows creating anonymous Rust structs on the fly, similar to TypeScript's structural typing.
```rust
let data = user().find_unique().select_as(db::select_as!({
    id: String,
    posts: { title: String }[]
})).exec(&client).await?;
```

### 2. Type-Safe Relation Transitions
Calling `.include()` or relation methods (like `.posts()`) automatically transitions the return type of the builder (e.g., from `User` to `UserWithPosts`). This is handled via a trait-based state machine generated at compile time.

### 3. Nested Include API
Supports deep recursion and filtering inside relations:
```rust
.include(|i| i.posts_with(|p| {
    p.where_clause(|w| w.published().eq(true));
    p.comments(); // Nesting comments inside posts
}))
```

## Global Features

| Feature | Status | Notes |
| :--- | :--- | :--- |
| Client Init | ✅ Supported | `db::client().await?` |
| Transactions | ❌ Not Supported | Planned. |
| Raw Queries | ❌ Not Supported | Planned. |
| Middlewares | ❌ Not Supported | |
| Enums | ✅ Supported | Full generation from schema. |
