//! Thin wrapper builders for composability (ReadBuilder, WriteBuilder, CountBuilder, etc.)

use crate::model_analysis::ModelMetadata;
use quote::{format_ident, quote};

pub fn generate_read_wrapper(
    model_name: &syn::Ident,
    model_metadata: &ModelMetadata,
) -> proc_macro2::TokenStream {
    let read_wrapper_name = format_ident!("{}ReadBuilder", model_name);
    let where_builder_name = format_ident!("{}WhereBuilder", model_name);
    let select_builder_name = format_ident!("{}SelectBuilder", model_name);
    let include_builder_name = format_ident!("{}IncludeBuilder", model_name);
    let scalar_field_names = &model_metadata.scalar_field_names;

    quote! {
        pub struct #read_wrapper_name {
            inner: prisma_core::ReadBuilder,
        }

        impl #read_wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_builder_name)
            {
                let mut builder = #where_builder_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args {
                        map.insert(k, v);
                    }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn select<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #select_builder_name)
            {
                let mut builder = #select_builder_name::default();
                f(&mut builder);
                let fields: Vec<String> = builder.into();
                use prisma_core::Selectable;
                for field in fields {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(field));
                }
                self
            }

            pub fn include<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #include_builder_name)
            {
                let mut builder = #include_builder_name::default();
                f(&mut builder);

                // When including relations, automatically select all scalar fields
                use prisma_core::Selectable;
                for scalar_field in &[#(#scalar_field_names),*] {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(scalar_field.to_string()));
                }

                // Then add the relation selections
                let includes: Vec<prisma_core::query_core::Selection> = builder.into();
                for sel in includes {
                    self.inner.add_nested_selection(sel);
                }
                self
            }

            pub fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::client::PrismaClient) -> impl std::future::Future<Output = prisma_core::Result<T>> + Send {
                use prisma_core::Executable;
                self.inner.exec(client)
            }
        }
    }
}

pub fn generate_unique_read_wrapper(
    model_name: &syn::Ident,
    model_metadata: &ModelMetadata,
) -> proc_macro2::TokenStream {
    let unique_read_wrapper_name = format_ident!("{}UniqueReadBuilder", model_name);
    let unique_where_builder_name = format_ident!("{}UniqueWhereBuilder", model_name);
    let select_builder_name = format_ident!("{}SelectBuilder", model_name);
    let include_builder_name = format_ident!("{}IncludeBuilder", model_name);
    let scalar_field_names = &model_metadata.scalar_field_names;

    quote! {
        pub struct #unique_read_wrapper_name {
            inner: prisma_core::ReadBuilder,
        }

        impl #unique_read_wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #unique_where_builder_name)
            {
                let mut builder = #unique_where_builder_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args {
                        map.insert(k, v);
                    }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn select<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #select_builder_name)
            {
                let mut builder = #select_builder_name::default();
                f(&mut builder);
                let fields: Vec<String> = builder.into();
                use prisma_core::Selectable;
                for field in fields {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(field));
                }
                self
            }

            pub fn include<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #include_builder_name)
            {
                let mut builder = #include_builder_name::default();
                f(&mut builder);

                // When including relations, automatically select all scalar fields
                use prisma_core::Selectable;
                for scalar_field in &[#(#scalar_field_names),*] {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(scalar_field.to_string()));
                }

                // Then add the relation selections
                let includes: Vec<prisma_core::query_core::Selection> = builder.into();
                for sel in includes {
                    self.inner.add_nested_selection(sel);
                }
                self
            }

            pub fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::client::PrismaClient) -> impl std::future::Future<Output = prisma_core::Result<T>> + Send {
                use prisma_core::Executable;
                self.inner.exec(client)
            }
        }
    }
}

pub fn generate_write_wrapper(model_name: &syn::Ident) -> proc_macro2::TokenStream {
    let write_wrapper_name = format_ident!("{}WriteBuilder", model_name);
    let unique_where_builder_name = format_ident!("{}UniqueWhereBuilder", model_name);
    let select_builder_name = format_ident!("{}SelectBuilder", model_name);
    let data_builder_name = format_ident!("{}DataBuilder", model_name);

    quote! {
        pub struct #write_wrapper_name {
            inner: prisma_core::WriteBuilder,
        }

        impl #write_wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #unique_where_builder_name)
            {
                let mut builder = #unique_where_builder_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args {
                        map.insert(k, v);
                    }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn data<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #data_builder_name)
            {
                let mut builder = #data_builder_name::default();
                f(&mut builder);
                use prisma_core::Filterable;
                self.inner.add_filter_arg("data".to_string(), prisma_core::query_core::ArgumentValue::Object(builder.data));
                self
            }

            pub fn select<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #select_builder_name)
            {
                let mut builder = #select_builder_name::default();
                f(&mut builder);
                let fields: Vec<String> = builder.into();
                use prisma_core::Selectable;
                for field in fields {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(field));
                }
                self
            }

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::PrismaClient) -> prisma_core::Result<T> {
                use prisma_core::Executable;
                self.inner.exec(client).await
            }
        }
    }
}

pub fn generate_count_wrapper(model_name: &syn::Ident) -> proc_macro2::TokenStream {
    let count_wrapper_name = format_ident!("{}CountBuilder", model_name);
    let where_builder_name = format_ident!("{}WhereBuilder", model_name);

    quote! {
        pub struct #count_wrapper_name {
            inner: prisma_core::CountBuilder,
        }

        impl #count_wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_builder_name)
            {
                let mut builder = #where_builder_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args {
                        map.insert(k, v);
                    }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::PrismaClient) -> prisma_core::Result<T> {
                use prisma_core::Executable;
                self.inner.exec(client).await
            }
        }
    }
}

pub fn generate_aggregate_wrapper(model_name: &syn::Ident) -> proc_macro2::TokenStream {
    let aggregate_wrapper_name = format_ident!("{}AggregateBuilder", model_name);
    let where_builder_name = format_ident!("{}WhereBuilder", model_name);

    quote! {
        pub struct #aggregate_wrapper_name {
            inner: prisma_core::AggregateBuilder,
        }

        impl #aggregate_wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_builder_name)
            {
                let mut builder = #where_builder_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args {
                        map.insert(k, v);
                    }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::PrismaClient) -> prisma_core::Result<T> {
                use prisma_core::Executable;
                self.inner.exec(client).await
            }
        }
    }
}

pub fn generate_group_by_wrapper(model_name: &syn::Ident) -> proc_macro2::TokenStream {
    let group_by_wrapper_name = format_ident!("{}GroupByBuilder", model_name);
    let where_builder_name = format_ident!("{}WhereBuilder", model_name);

    quote! {
        pub struct #group_by_wrapper_name {
            inner: prisma_core::GroupByBuilder,
        }

        impl #group_by_wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_builder_name)
            {
                let mut builder = #where_builder_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args {
                        map.insert(k, v);
                    }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::PrismaClient) -> prisma_core::Result<T> {
                use prisma_core::Executable;
                self.inner.exec(client).await
            }
        }
    }
}
