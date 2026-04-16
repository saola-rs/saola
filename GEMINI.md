# Prisma Rust ORM Project Context

## Project Overview
This project is building a production-grade, native Rust ORM powered by Prisma's internal Rust engines (`query-engine`, `quaint`). Unlike the standard Prisma Client Rust which generates large `.rs` files via a CLI step, this project uses a **single-macro compile-time generation approach** (`prisma_macros::init!("schema.prisma")`) to provide highly ergonomic, type-safe builder APIs with minimal boilerplate.

## Core Architectural Principles
1. **Compile-Time Generation:** All models, enums, and query builders are generated at compile time using the official `psl` (Prisma Schema Language) parser within proc-macros.
2. **Ergonomics & Type Inference:** The API is designed to feel like the TypeScript Prisma Client. Return types are automatically inferred based on the query:
   - `.find_many()` -> `Vec<T>` (via `ManyReadBuilder`)
   - `.find_unique()` / `.find_first()` -> `Option<T>` (via `OptionalReadBuilder`)
   - `..._or_throw()` -> `T` (via `RequiredReadBuilder`)
   - Write operations (`create`, `update`, `delete`) -> `T` (via `WriteBuilder`)
<!-- 3. **Relationship Type Transitions:** Calling a relation include method (e.g., `.posts()`) on a builder automatically transitions the generic return type of the builder to a specialized, auto-generated struct (e.g., `UserWithPosts`). This enables type-safe nested reads without requiring the user to define intermediate structs manually. -->
4. **Ad-Hoc Structural Typing:** The `select_as!` macro allows for TypeScript-like partial selections. It generates anonymous structs on the fly to perfectly match the requested data shape, including nested arrays and objects.

## Codebase Map
- **`prisma_core/`**: The runtime crate. It initializes the internal `query-engine`, manages the `PrismaClient`, and provides the base generic operation builders (`ReadBuilder<T>`, `WriteBuilder<T>`, etc.) and foundational traits (`Filterable`, `Selectable`, `Executable`, `FilterOp`).
- **`prisma-macros/`**: The proc-macro engine.
  - `src/lib.rs`: Exports `init!`, `prisma_model` attribute macro, and `select_as!`.
  - `src/codegen_orchestrator.rs`: Orchestrates the generation of the `db` module.
  - `src/model_gen.rs`: Generates standard base Rust structs (`User`, `Post`), Enums, and relation combo types (`UserWithPosts`).
  - `src/wrapper_gen.rs`: Generates the ergonomic, type-state wrapper structs (`UserManyReadBuilder`, etc.) that provide the fluent API, manage the generic `ReturnTy`, and expose relationship include methods.
  - `src/builder_gen.rs`: Generates the specific argument builders (`WhereBuilder`, `UniqueWhereBuilder`, `SelectBuilder`, `DataBuilder`).
  - `src/model_analysis.rs`: Analyzes `syn` ASTs to extract metadata and generate specific methods (like filter operators and data insertions).
  - `src/select_macro.rs`: Contains the parsing and struct-generation logic for the `select_as!` macro.
- **`example-app/`**: The primary testbed and demonstration application. Always run `cargo run -p example-app` to verify end-to-end functionality after making architectural changes.

## Development Mandates
- **Type Safety Above All:** Never compromise on compile-time type safety. Ensure that all generated builders correctly constrain user inputs (e.g., only allowing fields marked with `@unique` or `@id` in `UniqueWhereBuilder`).
- **Handle `EmptySelection` Carefully:** When generating nested selections (e.g., within `.include()` or inside the `select_as!` macro), ALWAYS ensure that necessary scalar fields are selected. The Prisma query engine will throw an `EmptySelection` runtime error if a model is queried without explicitly selecting at least one field.
- **Macro Hygiene:** Ensure generated code does not produce compiler warnings. Prefix unused generated variables with an underscore (e.g., `_scalar_field_names`) and provide explicit type signatures in closures where inference might fail inside macros.
- **Testing:** Any new ORM feature (e.g., nested writes, transactions, complex aggregations) must be demonstrated and verified within `example-app/src/main.rs`.
