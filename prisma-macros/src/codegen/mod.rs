use parser_database::ParserDatabase;
use proc_macro2::TokenStream;

// Sub-modules for modular code generation
mod enums;
mod select;
mod include;
mod filters;
mod builders;
mod returns;

/// Main orchestrator: Generate all client code from the parsed schema
pub fn generate_client(db: &ParserDatabase) -> TokenStream {
    let mut output = TokenStream::new();

    // 1. Generate enums first (needed for type-aware filter methods)
    output.extend(enums::generate_enums(db));

    // 2. Generate SelectBuilder per model (scalar fields only)
    output.extend(select::generate_select_builders(db));

    // 3. Generate IncludeBuilder per model (relations only)
    output.extend(include::generate_include_builders(db));

    // 4. Generate WhereBuilder per model (type-aware filters)
    output.extend(filters::generate_where_builders(db));

    // 5. Generate query builders (FindMany, FindUnique, Create, Update, Delete)
    output.extend(builders::generate_query_builders(db));

    // 6. Generate model namespaces (UserQuery, PostQuery, etc)
    output.extend(builders::generate_model_namespaces(db));

    // 7. Generate return types (UserSelected, UserWithPosts, etc)
    output.extend(returns::generate_return_types(db));

    // 8. Generate client root entry point
    output.extend(builders::generate_client_root(db));

    output
}
