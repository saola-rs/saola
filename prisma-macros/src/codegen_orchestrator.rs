//! Orchestrate all code generation for unified init! macro

use quote::quote;
use crate::model_gen;

/// Generate the complete module structure
pub fn generate_module(schema: &psl::ValidatedSchema, schema_path: &str) -> proc_macro2::TokenStream {
    let db = &schema.db;

    // Generate all enums first
    let mut enum_code = Vec::new();
    for walker in db.walk_enums() {
        enum_code.push(model_gen::generate_enum(db, walker));
    }

    // Generate all model structs
    let mut model_code = Vec::new();
    for walker in db.walk_models() {
        model_code.push(model_gen::generate_model_struct(db, walker));
    }

    // Extract datasource info
    let datasource = schema.configuration.datasources.first().expect("No datasource found in schema");
    let datasource_url = &datasource.url.as_literal().unwrap_or_else(|| {
        datasource.url.as_env_var().unwrap_or("")
    });
    // For runtime evaluation, we either use literal or env! macro if it's an env var
    let url_tokens = if let Some(env_var) = datasource.url.as_env_var() {
        quote! { std::env::var(#env_var).unwrap_or_else(|_| String::new()).as_str() }
    } else {
        quote! { #datasource_url }
    };

    quote! {
        pub mod db {
            // Re-export prelude for generated code
            use ::prisma_core::prelude::*;

            // ============ ENUMS ============
            #(#enum_code)*

            // ============ MODELS ============
            #(#model_code)*

            // ============ CLIENT INITIALIZATION ============

            /// Get or create the Prisma client with auto-configured datasource
            pub async fn client() -> ::prisma_core::Result<::prisma_core::PrismaClient> {
                ::prisma_core::PrismaClient::new(
                    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #schema_path)),
                    #url_tokens
                ).await
            }

            // ============ QUERY FACTORIES ============
            // Auto-generated query factory functions are exposed in the generated #[prisma_model] structs
        }
    }
}
