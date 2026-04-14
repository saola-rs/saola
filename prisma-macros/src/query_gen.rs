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
    let read_wrapper_name = format_ident!("{}ReadBuilder", model_name);
    let unique_read_wrapper_name = format_ident!("{}UniqueReadBuilder", model_name);
    let write_wrapper_name = format_ident!("{}WriteBuilder", model_name);
    let count_wrapper_name = format_ident!("{}CountBuilder", model_name);
    let aggregate_wrapper_name = format_ident!("{}AggregateBuilder", model_name);
    let group_by_wrapper_name = format_ident!("{}GroupByBuilder", model_name);

    let scalar_field_names = &model_metadata.scalar_field_names;
    let create_params = model_metadata.create_params();
    let create_data_inserts = model_metadata.create_data_inserts();

    quote! {
        /// Query factory for #model_name operations
        pub struct #query_name;

        /// Create a query builder for #model_name operations
        ///
        /// # Example
        /// ```ignore
        /// let user = #model_name_snake()
        ///     .find_unique()
        ///     .where_clause(|w| w.id().eq("123"))
        ///     .exec(&client)
        ///     .await?;
        /// ```
        pub fn #model_name_snake() -> #query_name {
            #query_name
        }

        impl #query_name {
            // ============ READ OPERATIONS ============

            /// Find many records matching criteria
            pub fn find_many(&self) -> #read_wrapper_name {
                #read_wrapper_name {
                    inner: prisma_core::ReadBuilder::find_many(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            /// Find a unique record by unique field
            pub fn find_unique(&self) -> #unique_read_wrapper_name {
                #unique_read_wrapper_name {
                    inner: prisma_core::ReadBuilder::find_unique(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            /// Find the first record matching criteria (with ordering)
            pub fn find_first(&self) -> #read_wrapper_name {
                #read_wrapper_name {
                    inner: prisma_core::ReadBuilder::find_first(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            /// Find the first record or throw an error if not found
            pub fn find_first_or_throw(&self) -> #read_wrapper_name {
                #read_wrapper_name {
                    inner: prisma_core::ReadBuilder::find_first_or_throw(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            /// Find a unique record or throw an error if not found
            pub fn find_unique_or_throw(&self) -> #unique_read_wrapper_name {
                #unique_read_wrapper_name {
                    inner: prisma_core::ReadBuilder::find_unique_or_throw(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            // ============ WRITE OPERATIONS ============

            /// Create a new record
            pub fn create(&self, #create_params) -> #write_wrapper_name {
                let mut builder = prisma_core::WriteBuilder::create(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*]);
                let mut data_map = prisma_core::IndexMap::new();
                #create_data_inserts
                use prisma_core::Filterable;
                builder.add_filter_arg("data".to_string(), prisma_core::query_core::ArgumentValue::Object(data_map));
                #write_wrapper_name { inner: builder }
            }

            /// Update a record by unique identifier
            pub fn update(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::update(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            /// Delete a record by unique identifier
            pub fn delete(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::delete(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            /// Create multiple records
            pub fn create_many(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::create_many(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            /// Update multiple records matching criteria
            pub fn update_many(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::update_many(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            /// Delete multiple records matching criteria
            pub fn delete_many(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::delete_many(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            /// Create or update a record
            pub fn upsert(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::upsert(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            // ============ AGGREGATION OPERATIONS ============

            /// Count records matching criteria
            pub fn count(&self) -> #count_wrapper_name {
                #count_wrapper_name {
                    inner: prisma_core::CountBuilder::new(#model_name_str.to_string(), vec![])
                }
            }

            /// Aggregate data (count, sum, avg, min, max)
            pub fn aggregate(&self) -> #aggregate_wrapper_name {
                #aggregate_wrapper_name {
                    inner: prisma_core::AggregateBuilder::new(#model_name_str.to_string(), vec![])
                }
            }

            /// Group records by field(s) with aggregation
            pub fn group_by(&self) -> #group_by_wrapper_name {
                #group_by_wrapper_name {
                    inner: prisma_core::GroupByBuilder::new(#model_name_str.to_string(), vec![])
                }
            }
        }
    }
}
