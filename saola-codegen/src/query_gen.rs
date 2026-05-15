//! Query factory generation - creates the {Model}Query struct and factory functions

use crate::model_analysis::ModelMetadata;
use quote::{format_ident, quote};

pub fn generate_query_factory(
    model_name: &syn::Ident,
    _model_name_snake: &syn::Ident,
    model_metadata: &ModelMetadata,
) -> proc_macro2::TokenStream {
    let query_struct_name = format_ident!("{}Query", model_name);
    let marker_name = format_ident!("{}Marker", model_name);

    let create_params_vec = model_metadata.create_params();
    let create_params = if create_params_vec.is_empty() {
        quote! {}
    } else {
        quote! { #(#create_params_vec),* }
    };
    let create_data_inserts = model_metadata.create_data_inserts("data_map");

    quote! {
        pub struct #query_struct_name {
            pub provider: std::sync::Arc<dyn ::saola_core::transaction::QueryExecutorProvider>,
        }

        impl #query_struct_name {
            // ============ READ OPERATIONS ============

            pub fn find_many(&self) -> ::saola_core::Query<#marker_name, ::saola_core::FindMany, Vec<#model_name>> {
                ::saola_core::Query::new("findMany").with_provider(self.provider.clone())
            }

            pub fn find_unique(&self) -> ::saola_core::Query<#marker_name, ::saola_core::FindUnique, Option<#model_name>> {
                ::saola_core::Query::new("findUnique").with_provider(self.provider.clone())
            }

            pub fn find_first(&self) -> ::saola_core::Query<#marker_name, ::saola_core::FindFirst, Option<#model_name>> {
                ::saola_core::Query::new("findFirst").with_provider(self.provider.clone())
            }

            pub fn find_unique_or_throw(&self) -> ::saola_core::Query<#marker_name, ::saola_core::FindUniqueOrThrow, #model_name> {
                ::saola_core::Query::new("findUniqueOrThrow").with_provider(self.provider.clone())
            }

            pub fn find_first_or_throw(&self) -> ::saola_core::Query<#marker_name, ::saola_core::FindFirstOrThrow, #model_name> {
                ::saola_core::Query::new("findFirstOrThrow").with_provider(self.provider.clone())
            }

            // ============ WRITE OPERATIONS ============

            pub fn create(&self, #create_params) -> ::saola_core::Query<#marker_name, ::saola_core::Create, #model_name> {
                let mut query = ::saola_core::Query::new("createOne").with_provider(self.provider.clone());
                let mut data_map = ::saola_core::IndexMap::new();
                #create_data_inserts
                query.state.arguments.insert("data".to_string(), ::saola_core::query_core::ArgumentValue::Object(data_map));
                query
            }

            pub fn update(&self) -> ::saola_core::Query<#marker_name, ::saola_core::Update, #model_name> {
                ::saola_core::Query::new("updateOne").with_provider(self.provider.clone())
            }

            pub fn delete(&self) -> ::saola_core::Query<#marker_name, ::saola_core::Delete, #model_name> {
                ::saola_core::Query::new("deleteOne").with_provider(self.provider.clone())
            }

            pub fn upsert(&self, #create_params) -> ::saola_core::Query<#marker_name, ::saola_core::Upsert, #model_name> {
                let mut query = ::saola_core::Query::new("upsertOne").with_provider(self.provider.clone());
                let mut data_map = ::saola_core::IndexMap::new();
                #create_data_inserts
                query.state.arguments.insert("create".to_string(), ::saola_core::query_core::ArgumentValue::Object(data_map));
                query
            }
        }
    }
}
