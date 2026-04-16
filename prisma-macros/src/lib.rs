extern crate proc_macro;

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::format_ident;
use syn::{ItemStruct, Type, parse_macro_input};

// Internal modules
mod builder_gen;
mod codegen_orchestrator;
mod model_analysis;
mod model_gen;
mod query_gen;
mod select_macro;
mod utils;
mod wrapper_gen;

use model_analysis::{FieldMetadata, ModelMetadata};

/// Parse model fields and extract metadata
fn parse_model_fields(input: &mut ItemStruct) -> Vec<FieldMetadata> {
    let mut fields = Vec::new();

    for field in &mut input.fields {
        let rust_name = field.ident.as_ref().unwrap().to_string();
        let mut prisma_name = rust_name.clone();
        let mut is_relation = false;
        let mut is_unique = false;
        let mut is_id = false;

        let is_optional = {
            if let Type::Path(tp) = &field.ty {
                tp.path.segments.last().unwrap().ident == "Option"
            } else {
                false
            }
        };

        let is_list = {
            if let Type::Path(tp) = &field.ty {
                tp.path.segments.last().unwrap().ident == "Vec"
            } else {
                false
            }
        };

        // Parse prisma attributes
        field.attrs.retain(|attr| {
            if attr.path().is_ident("prisma") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        let s: syn::LitStr = value.parse()?;
                        prisma_name = s.value();
                    } else if meta.path.is_ident("relation") {
                        is_relation = true;
                    } else if meta.path.is_ident("id") {
                        is_id = true;
                        is_unique = true;
                    } else if meta.path.is_ident("unique") {
                        is_unique = true;
                    }
                    Ok(())
                });
                false
            } else {
                true
            }
        });

        fields.push(FieldMetadata {
            rust_name,
            prisma_name,
            is_relation,
            is_unique,
            is_id,
            is_optional,
            is_list,
            is_relation_link: false,
            has_default: false,
            opposite_relation_field: None,
            field_type: field.ty.clone(),
        });
    }

    fields
}

/// Dummy attribute macro to keep prisma-specific field annotations in generated structs
#[proc_macro_attribute]
pub fn db_prisma(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Generate type-safe model builders from Rust struct
#[proc_macro_attribute]
pub fn prisma_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    // Extract model name before parsing fields (to avoid borrow conflicts)
    let model_name = input.ident.clone();
    let model_name_str = model_name.to_string();
    let model_name_snake = format_ident!("{}", model_name_str.to_snake_case());

    // Parse field metadata from model
    let fields = parse_model_fields(&mut input);
    let model_metadata = ModelMetadata::new(model_name_str.clone(), fields);

    // Generate all builder and query code
    let where_builder = builder_gen::generate_where_builder(&model_name, &model_metadata);
    let unique_where_builder = builder_gen::generate_unique_where_builder(&model_name, &model_metadata);
    let select_builder = builder_gen::generate_select_builder(&model_name, &model_metadata);
    let include_builder = builder_gen::generate_include_builder(&model_name, &model_metadata);
    let empty_map: std::collections::HashMap<String, ModelMetadata> = std::collections::HashMap::new();
    let data_builder = builder_gen::generate_data_builder(&model_name, &model_metadata, Some(&empty_map));

    let read_wrappers = wrapper_gen::generate_read_wrappers(&model_name, &model_metadata);
    let write_wrapper = wrapper_gen::generate_write_wrapper(&model_name, &model_metadata);
    let count_wrapper = wrapper_gen::generate_count_wrapper(&model_name);
    let aggregate_wrapper = wrapper_gen::generate_aggregate_wrapper(&model_name);
    let group_by_wrapper = wrapper_gen::generate_group_by_wrapper(&model_name);

    let query_factory =
        query_gen::generate_query_factory(&model_name, &model_name_snake, &model_name_str, &model_metadata);

    let expanded = quote::quote! {
        // Original model struct with annotations preserved
        #input

        // Phase 2.1: Type-safe filter builders with compile-time operator validation

        // ============ WHERE BUILDERS ============
        #where_builder
        #unique_where_builder

        // ============ SELECTION BUILDERS ============
        #select_builder
        #include_builder

        // ============ DATA BUILDERS ============
        #data_builder

        // ============ THIN WRAPPER BUILDERS ============
        #read_wrappers
        #write_wrapper
        #count_wrapper
        #aggregate_wrapper
        #group_by_wrapper

        // ============ QUERY FACTORY ============
        #query_factory
    };

    TokenStream::from(expanded)
}

/// The `init!` macro generates the entire Prisma client module from a schema file.
#[proc_macro]
pub fn init(input: TokenStream) -> TokenStream {
    let schema_path = if input.is_empty() {
        "schema.prisma".to_string()
    } else {
        let lit: syn::LitStr = syn::parse_macro_input!(input);
        lit.value()
    };

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let full_path = std::path::PathBuf::from(manifest_dir).join(&schema_path);

    let schema_content = std::fs::read_to_string(&full_path)
        .unwrap_or_else(|e| panic!("Could not read schema at {:?}: {}", full_path, e));

    let source_file = psl::SourceFile::from(schema_content.as_str());
    let parsed_schema = psl::validate(source_file, &psl::parser_database::NoExtensionTypes);

    if !parsed_schema.diagnostics.errors().is_empty() {
        panic!("Schema validation failed: {:?}", parsed_schema.diagnostics.errors());
    }

    let module = codegen_orchestrator::generate_module(&parsed_schema, &schema_path);

    TokenStream::from(module)
}

/// The `as_type!` macro generates a unique type identifier for ad-hoc structs.
#[proc_macro]
pub fn as_type(input: TokenStream) -> TokenStream {
    select_macro::generate_as_type(input.into()).into()
}

/// The `select_as!` macro generates an ad-hoc struct and applies it to a query.
///
/// Usage: `select_as!(user().find_many(), { id: String, email: String })`
#[proc_macro]
pub fn select_as(input: TokenStream) -> TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);
    let shape = syn::parse2::<select_macro::SelectShape>(input2)
        .expect("Failed to parse select_as! input. Ensure syntax is select_as!(query, { ... })");
    select_macro::select_macro_impl(shape).into()
}
