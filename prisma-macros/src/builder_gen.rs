//! Builder struct generation (where, select, include, data builders)

use crate::model_analysis::ModelMetadata;
use quote::{format_ident, quote};

pub fn generate_where_builder(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let where_builder_name = format_ident!("{}WhereBuilder", model_name);
    let filter_methods = crate::model_analysis::generate_scalar_filter_methods(&model_metadata.fields);

    quote! {
        #[derive(Default)]
        pub struct #where_builder_name {
            pub args: Vec<(String, prisma_core::query_core::ArgumentValue)>,
        }

        impl prisma_core::FilterBuilder for #where_builder_name {
            fn add_arg(&mut self, name: String, value: prisma_core::query_core::ArgumentValue) {
                self.args.push((name, value));
            }
        }

        impl #where_builder_name {
            pub fn and<F>(&mut self, f: F) -> &mut Self
            where F: FnOnce(&mut Self)
            {
                let mut builder = Self::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args {
                        map.insert(k, v);
                    }
                    self.args.push(("AND".to_string(), prisma_core::query_core::ArgumentValue::Object(map)));
                }
                self
            }

            pub fn or<F>(&mut self, f: F) -> &mut Self
            where F: FnOnce(&mut Self)
            {
                let mut builder = Self::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args {
                        map.insert(k, v);
                    }
                    self.args.push(("OR".to_string(), prisma_core::query_core::ArgumentValue::List(vec![prisma_core::query_core::ArgumentValue::Object(map)])));
                }
                self
            }

            pub fn not<F>(&mut self, f: F) -> &mut Self
            where F: FnOnce(&mut Self)
            {
                let mut builder = Self::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args {
                        map.insert(k, v);
                    }
                    self.args.push(("NOT".to_string(), prisma_core::query_core::ArgumentValue::Object(map)));
                }
                self
            }

            #(#filter_methods)*
        }
    }
}

pub fn generate_unique_where_builder(
    model_name: &syn::Ident,
    model_metadata: &ModelMetadata,
) -> proc_macro2::TokenStream {
    let unique_where_builder_name = format_ident!("{}UniqueWhereBuilder", model_name);
    let unique_filters = crate::model_analysis::generate_unique_filter_methods(&model_metadata.fields);

    quote! {
        #[derive(Default)]
        pub struct #unique_where_builder_name {
            pub args: Vec<(String, prisma_core::query_core::ArgumentValue)>,
        }

        impl prisma_core::FilterBuilder for #unique_where_builder_name {
            fn add_arg(&mut self, name: String, value: prisma_core::query_core::ArgumentValue) {
                self.args.push((name, value));
            }
        }

        impl #unique_where_builder_name {
            #(#unique_filters)*
        }
    }
}

pub fn generate_select_builder(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let select_builder_name = format_ident!("{}SelectBuilder", model_name);
    let select_methods = crate::model_analysis::generate_select_methods(&model_metadata.fields);
    let scalar_field_names = &model_metadata.scalar_field_names;

    quote! {
        #[derive(Default)]
        pub struct #select_builder_name {
            pub selections: Vec<prisma_core::query_core::Selection>,
        }

        impl #select_builder_name {
            pub fn all(&mut self) -> &mut Self {
                for field in &[#(#scalar_field_names),*] {
                    self.selections.push(prisma_core::query_core::Selection::with_name(field.to_string()));
                }
                self
            }

            #(#select_methods)*
        }

        impl From<#select_builder_name> for Vec<prisma_core::query_core::Selection> {
            fn from(b: #select_builder_name) -> Self { b.selections }
        }
    }
}

pub fn generate_include_builder(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let include_builder_name = format_ident!("{}IncludeBuilder", model_name);
    let include_methods = crate::model_analysis::generate_include_methods(&model_metadata.fields);

    quote! {
        #[derive(Default)]
        pub struct #include_builder_name {
            pub includes: Vec<prisma_core::query_core::Selection>,
        }

        impl #include_builder_name {
            #(#include_methods)*
        }

        impl From<#include_builder_name> for Vec<prisma_core::query_core::Selection> {
            fn from(b: #include_builder_name) -> Self { b.includes }
        }
    }
}

pub fn generate_data_builder(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let data_builder_name = format_ident!("{}DataBuilder", model_name);
    let data_methods = crate::model_analysis::generate_data_methods(&model_metadata.fields);

    quote! {
        #[derive(Default)]
        pub struct #data_builder_name {
            pub data: prisma_core::IndexMap<String, prisma_core::query_core::ArgumentValue>,
        }

        impl #data_builder_name {
            #(#data_methods)*
        }

        impl From<#data_builder_name> for prisma_core::query_structure::PrismaValue {
            fn from(_b: #data_builder_name) -> Self {
                // This is a bit complex as we need to convert IndexMap<String, ArgumentValue> to PrismaValue
                // For now, return a placeholder
                prisma_core::query_structure::PrismaValue::Null
            }
        }
    }
}
