use parser_database::walkers::{ModelWalker, ScalarFieldWalker};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate return types based on model structures
/// Generates typed structs for common selection patterns:
/// - {Model}Selected - all scalar fields
/// - {Model}WithRelations - for included relations
pub fn generate_return_types(db: &parser_database::ParserDatabase) -> TokenStream {
    let mut output = TokenStream::new();

    for model in db.walk_models() {
        let model_name = model.name();

        // Generate the "all scalar fields" return type
        output.extend(generate_model_selected_type(model_name, model));

        // Generate return types for each relation (for use with .include())
        for relation in model.relation_fields() {
            output.extend(generate_model_with_relation_type(model_name, model, relation));
        }
    }

    output
}

/// Generate a return struct with all scalar fields for a model
/// Example: `struct UserSelected { id: String, email: String, name: Option<String>, role: Role }`
fn generate_model_selected_type(model_name: &str, model: ModelWalker) -> TokenStream {
    let struct_name = format_ident!("{}Selected", model_name);

    let fields: Vec<_> = model
        .scalar_fields()
        .map(|field| {
            let field_name = format_ident!("{}", field.name());
            let field_type = get_field_rust_type(field);

            quote! {
                pub #field_name: #field_type,
            }
        })
        .collect();

    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct #struct_name {
            #(#fields)*
        }
    }
}

/// Generate a return struct for a model with a specific relation included
/// Example: `struct UserWithPosts { id: String, email: String, ..., posts: Vec<PostSelected> }`
fn generate_model_with_relation_type(
    model_name: &str,
    model: ModelWalker,
    relation: parser_database::walkers::RelationFieldWalker,
) -> TokenStream {
    let struct_name = format_ident!(
        "{}With{}",
        model_name,
        capitalize_first(relation.name())
    );

    // All scalar fields from the model
    let scalar_fields: Vec<_> = model
        .scalar_fields()
        .map(|field| {
            let field_name = format_ident!("{}", field.name());
            let field_type = get_field_rust_type(field);

            quote! {
                pub #field_name: #field_type,
            }
        })
        .collect();

    // The relation field
    let relation_name = format_ident!("{}", relation.name());
    let related_model_name = relation.related_model().name();
    let related_struct = format_ident!("{}Selected", related_model_name);
    let is_list = relation.ast_field().arity.is_list();

    let relation_type = if is_list {
        quote! { Vec<#related_struct> }
    } else {
        quote! { Option<#related_struct> }
    };

    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct #struct_name {
            #(#scalar_fields)*
            pub #relation_name: #relation_type,
        }
    }
}

/// Get the Rust type for a scalar field
fn get_field_rust_type(field: ScalarFieldWalker) -> TokenStream {
    let base_type = match field.scalar_type() {
        Some(parser_database::ScalarType::String) => quote! { String },
        Some(parser_database::ScalarType::Int) => quote! { i32 },
        Some(parser_database::ScalarType::BigInt) => quote! { i64 },
        Some(parser_database::ScalarType::Float) => quote! { f64 },
        Some(parser_database::ScalarType::Boolean) => quote! { bool },
        Some(parser_database::ScalarType::DateTime) => quote! { String }, // TODO: use chrono
        Some(parser_database::ScalarType::Decimal) => quote! { String },  // TODO: use decimal
        Some(parser_database::ScalarType::Json) => quote! { serde_json::Value },
        Some(parser_database::ScalarType::Bytes) => quote! { Vec<u8> },
        _ => {
            // Check if it's an enum
            if let Some(enum_def) = field.field_type_as_enum() {
                let enum_name = enum_def.name();
                let enum_ident = format_ident!("{}", enum_name);
                return if field.is_optional() {
                    quote! { Option<#enum_ident> }
                } else {
                    quote! { #enum_ident }
                };
            }
            // Fallback to String for unknown types
            quote! { String }
        }
    };

    if field.is_optional() {
        quote! { Option<#base_type> }
    } else {
        base_type
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

