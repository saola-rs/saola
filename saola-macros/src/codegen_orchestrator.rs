//! Orchestrate all code generation for unified init! macro

use crate::builder_gen;
use crate::model_analysis::{FieldMetadata, ModelMetadata};
use crate::model_gen;
use crate::query_gen;
use crate::wrapper_gen;
use heck::ToSnakeCase;
use quote::{format_ident, quote};
use std::collections::HashMap;

/// Generate the complete module structure
pub fn generate_module(schema: &psl::ValidatedSchema, schema_path: &str) -> proc_macro2::TokenStream {
    let db = &schema.db;

    // 1. Collect metadata for all models first
    let mut model_metadata_map = HashMap::new();
    for walker in db.walk_models() {
        let mut fields = Vec::new();

        let relation_link_fields: std::collections::HashSet<_> = walker
            .relation_fields()
            .filter_map(|rf| rf.referencing_fields())
            .flatten()
            .map(|f| f.field_id())
            .collect();

        for field in walker.scalar_fields() {
            let field_type_str = match field.scalar_field_type() {
                psl::parser_database::ScalarFieldType::BuiltInScalar(psl::parser_database::ScalarType::String) => {
                    "String"
                }
                psl::parser_database::ScalarFieldType::BuiltInScalar(psl::parser_database::ScalarType::Int) => "i32",
                psl::parser_database::ScalarFieldType::BuiltInScalar(psl::parser_database::ScalarType::Boolean) => {
                    "bool"
                }
                psl::parser_database::ScalarFieldType::Enum(id) => db.walk(id).name(),
                _ => "String",
            }
            .to_string();

            let mut field_type: syn::Type = syn::parse_str(&field_type_str).unwrap();
            let is_optional = !field.ast_field().arity.is_required();

            if is_optional {
                field_type = syn::parse_quote! { Option<#field_type> };
            }

            fields.push(FieldMetadata {
                rust_name: field.name().to_snake_case(),
                prisma_name: field.name().to_string(),
                is_relation: false,
                is_unique: field.is_unique(),
                is_id: walker
                    .primary_key()
                    .map(|pk| pk.fields().any(|f| f.field_id() == field.field_id()))
                    .unwrap_or(false),
                is_optional,
                is_list: field.ast_field().arity.is_list(),
                is_relation_link: relation_link_fields.contains(&field.field_id()),
                has_default: field.default_value().is_some(),
                is_updated_at: field.is_updated_at(),
                opposite_relation_field: None,
                field_type,
            });
        }
        for field in walker.relation_fields() {
            let related_model_name = field.related_model().name();
            fields.push(FieldMetadata {
                rust_name: field.name().to_snake_case(),
                prisma_name: field.name().to_string(),
                is_relation: true,
                is_unique: false,
                is_id: false,
                is_optional: !field.ast_field().arity.is_required(),
                is_list: field.ast_field().arity.is_list(),
                is_relation_link: false,
                has_default: false,
                is_updated_at: false,
                opposite_relation_field: Some(field.opposite_relation_field().unwrap().name().to_string()),
                field_type: syn::parse_str(related_model_name).unwrap(),
            });
        }
        model_metadata_map.insert(
            walker.name().to_string(),
            ModelMetadata::new(walker.name().to_string(), fields),
        );
    }

    // Generate all enums first
    let mut enum_code = Vec::new();
    for walker in db.walk_enums() {
        enum_code.push(model_gen::generate_enum(db, walker));
    }

    // Generate all model structs AND their builders
    let mut model_code = Vec::new();
    for walker in db.walk_models() {
        let model_name_str = walker.name().to_string();
        let model_name = format_ident!("{}", model_name_str);
        let model_name_snake = format_ident!("{}", model_name_str.to_snake_case());
        let model_metadata = model_metadata_map.get(&model_name_str).unwrap();

        // 1. Struct and Relation Types
        model_code.push(model_gen::generate_model_struct(db, walker, &model_metadata_map));
        model_code.push(model_gen::generate_relation_types(db, walker));
        model_code.push(model_gen::generate_aggregation_structs(db, walker));
        model_code.push(model_gen::generate_group_by_struct(db, walker));

        // 2. Builders
        model_code.push(builder_gen::generate_where_builder(&model_name, model_metadata));
        model_code.push(builder_gen::generate_unique_where_builder(&model_name, model_metadata));
        model_code.push(builder_gen::generate_order_by_builder(&model_name, model_metadata));
        model_code.push(builder_gen::generate_select_builder(&model_name, model_metadata));
        model_code.push(builder_gen::generate_data_builder(
            &model_name,
            model_metadata,
            Some(&model_metadata_map),
        ));
        model_code.push(builder_gen::generate_aggregate_select_builders(
            &model_name,
            model_metadata,
        ));
        model_code.push(builder_gen::generate_group_by_builder(&model_name, model_metadata));

        // 3. Wrappers
        model_code.push(wrapper_gen::generate_read_wrappers(&model_name, model_metadata));
        model_code.push(wrapper_gen::generate_write_wrapper(&model_name, model_metadata));
        model_code.push(wrapper_gen::generate_upsert_wrapper(&model_name, model_metadata));
        model_code.push(wrapper_gen::generate_create_many_wrapper(&model_name, model_metadata));
        model_code.push(wrapper_gen::generate_create_many_and_return_wrapper(
            &model_name,
            model_metadata,
        ));
        model_code.push(wrapper_gen::generate_update_many_wrapper(&model_name, model_metadata));
        model_code.push(wrapper_gen::generate_update_many_and_return_wrapper(
            &model_name,
            model_metadata,
        ));
        model_code.push(wrapper_gen::generate_delete_many_wrapper(&model_name, model_metadata));
        model_code.push(wrapper_gen::generate_count_wrapper(&model_name));
        model_code.push(wrapper_gen::generate_aggregate_wrapper(&model_name));
        model_code.push(wrapper_gen::generate_group_by_wrapper(&model_name));

        // 4. Query Factory
        model_code.push(query_gen::generate_query_factory(
            &model_name,
            &model_name_snake,
            &model_name_str,
            model_metadata,
        ));
    }

    // Extract datasource info
    let datasource = schema
        .configuration
        .datasources
        .first()
        .expect("No datasource found in schema");
    let datasource_url = &datasource
        .url
        .as_literal()
        .unwrap_or_else(|| datasource.url.as_env_var().unwrap_or(""));
    let url_tokens = if let Some(env_var) = datasource.url.as_env_var() {
        quote! { std::env::var(#env_var).unwrap_or_else(|_| String::new()).as_str() }
    } else {
        quote! { #datasource_url }
    };

    quote! {
        pub mod saola {
            use ::saola_core::prelude::*;
            pub use ::saola_macros::select_as;
            pub use ::saola_macros::saola;

            #(#enum_code)*
            #(#model_code)*

            pub async fn client() -> ::saola_core::Result<::saola_core::SaolaClient> {
                ::saola_core::SaolaClient::new(
                    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #schema_path)),
                    #url_tokens
                ).await
            }
        }
    }
}
