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
    let many_read_name = format_ident!("{}ManyReadBuilder", model_name);
    let unique_read_name = format_ident!("{}UniqueReadBuilder", model_name);
    let first_read_name = format_ident!("{}FirstReadBuilder", model_name);
    let unique_throw_read_name = format_ident!("{}UniqueOrThrowReadBuilder", model_name);
    let first_throw_read_name = format_ident!("{}FirstOrThrowReadBuilder", model_name);
    let write_name = format_ident!("{}WriteBuilder", model_name);
    let upsert_name = format_ident!("{}UpsertBuilder", model_name);
    let create_many_name = format_ident!("{}CreateManyBuilder", model_name);
    let create_many_and_return_name = format_ident!("{}CreateManyAndReturnBuilder", model_name);
    let update_many_name = format_ident!("{}UpdateManyBuilder", model_name);
    let update_many_and_return_name = format_ident!("{}UpdateManyAndReturnBuilder", model_name);
    let delete_many_name = format_ident!("{}DeleteManyBuilder", model_name);
    let count_name = format_ident!("{}CountBuilder", model_name);
    let aggregate_name = format_ident!("{}AggregateBuilder", model_name);
    let group_by_name = format_ident!("{}GroupByBuilder", model_name);

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

            pub fn find_many(&self) -> #many_read_name<#model_name> {
                #many_read_name {
                    inner: ::saola_core::ReadBuilder::find_many(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn find_unique(&self) -> #unique_read_name<#model_name> {
                #unique_read_name {
                    inner: ::saola_core::ReadBuilder::find_unique(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn find_first(&self) -> #first_read_name<#model_name> {
                #first_read_name {
                    inner: ::saola_core::ReadBuilder::find_first(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn find_unique_or_throw(&self) -> #unique_throw_read_name<#model_name> {
                #unique_throw_read_name {
                    inner: ::saola_core::ReadBuilder::find_unique_or_throw(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn find_first_or_throw(&self) -> #first_throw_read_name<#model_name> {
                #first_throw_read_name {
                    inner: ::saola_core::ReadBuilder::find_first_or_throw(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                    _phantom: std::marker::PhantomData,
                }
            }

            // ============ WRITE OPERATIONS ============

            pub fn create(&self, #create_params) -> #write_name<#model_name> {
                let mut inner = ::saola_core::WriteBuilder::create(
                    #marker_name::NAME.to_string(),
                    #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                ).with_provider(self.provider.clone());
                let mut data_map = ::saola_core::IndexMap::new();
                #create_data_inserts
                inner.state.arguments.insert("data".to_string(), ::saola_core::query_core::ArgumentValue::Object(data_map));
                #write_name {
                    inner,
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn update(&self) -> #write_name<#model_name> {
                #write_name {
                    inner: ::saola_core::WriteBuilder::update(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn delete(&self) -> #write_name<#model_name> {
                #write_name {
                    inner: ::saola_core::WriteBuilder::delete(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn upsert(&self, #create_params) -> #upsert_name {
                let mut inner = ::saola_core::WriteBuilder::upsert(
                    #marker_name::NAME.to_string(),
                    #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                ).with_provider(self.provider.clone());
                let mut data_map = ::saola_core::IndexMap::new();
                #create_data_inserts
                inner.state.arguments.insert("create".to_string(), ::saola_core::query_core::ArgumentValue::Object(data_map));
                #upsert_name {
                    inner,
                }
            }

            pub fn create_many(&self) -> #create_many_name {
                #create_many_name {
                    inner: ::saola_core::CreateManyBuilder::new(
                        #marker_name::NAME.to_string(),
                    ).with_provider(self.provider.clone()),
                }
            }

            pub fn create_many_and_return(&self) -> #create_many_and_return_name {
                #create_many_and_return_name {
                    inner: ::saola_core::CreateManyAndReturnBuilder::new(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn update_many(&self) -> #update_many_name {
                #update_many_name {
                    inner: ::saola_core::UpdateManyBuilder::new(
                        #marker_name::NAME.to_string(),
                    ).with_provider(self.provider.clone()),
                }
            }

            pub fn update_many_and_return(&self) -> #update_many_and_return_name {
                #update_many_and_return_name {
                    inner: ::saola_core::UpdateManyAndReturnBuilder::new(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn delete_many(&self) -> #delete_many_name {
                #delete_many_name {
                    inner: ::saola_core::DeleteManyBuilder::new(
                        #marker_name::NAME.to_string(),
                    ).with_provider(self.provider.clone()),
                }
            }

            pub fn count(&self) -> #count_name {
                #count_name {
                    inner: ::saola_core::CountBuilder::new(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                }
            }

            pub fn aggregate(&self) -> #aggregate_name {
                #aggregate_name {
                    inner: ::saola_core::AggregateBuilder::new(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                }
            }

            pub fn group_by(&self) -> #group_by_name {
                #group_by_name {
                    inner: ::saola_core::GroupByBuilder::new(
                        #marker_name::NAME.to_string(),
                        #marker_name::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
                    ).with_provider(self.provider.clone()),
                }
            }
        }
    }
}
