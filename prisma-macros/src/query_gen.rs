//! Query factory generation - creates the {Model}Query struct and factory functions

use crate::model_analysis::ModelMetadata;
use quote::{format_ident, quote};

pub fn generate_query_factory(
    model_name: &syn::Ident,
    model_name_snake: &syn::Ident,
    model_name_str: &str,
    model_metadata: &ModelMetadata,
) -> proc_macro2::TokenStream {
    let query_name = format_ident!("{}Query", model_name);
    let many_name = format_ident!("{}ManyReadBuilder", model_name);
    let unique_name = format_ident!("{}UniqueReadBuilder", model_name);
    let first_name = format_ident!("{}FirstReadBuilder", model_name);
    let unique_throw_name = format_ident!("{}UniqueOrThrowReadBuilder", model_name);
    let first_throw_name = format_ident!("{}FirstOrThrowReadBuilder", model_name);

    let write_wrapper_name = format_ident!("{}WriteBuilder", model_name);
    let count_wrapper_name = format_ident!("{}CountBuilder", model_name);
    let aggregate_wrapper_name = format_ident!("{}AggregateBuilder", model_name);
    let group_by_wrapper_name = format_ident!("{}GroupByBuilder", model_name);

    let scalar_field_names = &model_metadata.scalar_field_names;
    let create_params = model_metadata.create_params();
    let create_data_inserts = model_metadata.create_data_inserts("data_map");

    quote! {
        /// Query factory for #model_name operations
        pub struct #query_name;

        /// Create a query builder for #model_name operations
        pub fn #model_name_snake() -> #query_name {
            #query_name
        }

        impl #query_name {
            // ============ READ OPERATIONS ============

            pub fn find_many(&self) -> #many_name {
                #many_name {
                    inner: ::prisma_core::ReadBuilder::<Vec<#model_name>>::find_many(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*]),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn find_unique(&self) -> #unique_name {
                #unique_name {
                    inner: ::prisma_core::ReadBuilder::<Option<#model_name>>::find_unique(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*]),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn find_first(&self) -> #first_name {
                #first_name {
                    inner: ::prisma_core::ReadBuilder::<Option<#model_name>>::find_first(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*]),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn find_first_or_throw(&self) -> #first_throw_name {
                #first_throw_name {
                    inner: ::prisma_core::ReadBuilder::<#model_name>::find_first_or_throw(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*]),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn find_unique_or_throw(&self) -> #unique_throw_name {
                #unique_throw_name {
                    inner: ::prisma_core::ReadBuilder::<#model_name>::find_unique_or_throw(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*]),
                    _phantom: std::marker::PhantomData,
                }
            }

            // ============ WRITE OPERATIONS ============

            pub fn create(&self, #create_params) -> #write_wrapper_name {
                let mut builder = ::prisma_core::WriteBuilder::<#model_name>::create(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*]);
                let mut data_map = ::prisma_core::IndexMap::new();
                #create_data_inserts
                use ::prisma_core::Filterable;
                builder.add_filter_arg("data".to_string(), ::prisma_core::query_core::ArgumentValue::Object(data_map));
                #write_wrapper_name {
                    inner: builder,
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn update(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: ::prisma_core::WriteBuilder::<#model_name>::update(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*]),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn delete(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: ::prisma_core::WriteBuilder::<#model_name>::delete(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*]),
                    _phantom: std::marker::PhantomData,
                }
            }

            // ============ AGGREGATION OPERATIONS ============

            pub fn count(&self) -> #count_wrapper_name {
                #count_wrapper_name {
                    inner: ::prisma_core::CountBuilder::new(#model_name_str.to_string(), vec![])
                }
            }

            pub fn aggregate(&self) -> #aggregate_wrapper_name {
                #aggregate_wrapper_name {
                    inner: ::prisma_core::AggregateBuilder::new(#model_name_str.to_string(), vec![])
                }
            }

            pub fn group_by(&self) -> #group_by_wrapper_name {
                #group_by_wrapper_name {
                    inner: ::prisma_core::GroupByBuilder::new(#model_name_str.to_string(), vec![])
                }
            }
        }
    }
}
