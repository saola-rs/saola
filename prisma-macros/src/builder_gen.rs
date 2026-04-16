//! Builder struct generation (where, select, include, data builders)

use crate::model_analysis::ModelMetadata;
use heck::ToUpperCamelCase;
use quote::{format_ident, quote};
use std::collections::HashMap;

pub fn generate_where_builder(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let where_builder_name = format_ident!("{}WhereBuilder", model_name);
    let filter_methods = crate::model_analysis::generate_filter_methods(&model_metadata.fields);

    quote! {
        #[derive(Default)]
        pub struct #where_builder_name {
            pub args: Vec<(String, prisma_core::query_core::ArgumentValue)>,
        }

        impl prisma_core::FilterBuilder for #where_builder_name {
            fn add_arg(&mut self, name: String, value: prisma_core::query_core::ArgumentValue) {
                self.args.push((name, value));
            }

            fn build(self) -> prisma_core::IndexMap<String, prisma_core::query_core::ArgumentValue> {
                let mut map = prisma_core::IndexMap::new();
                for (k, v) in self.args {
                    map.insert(k, v);
                }
                map
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

            fn build(self) -> prisma_core::IndexMap<String, prisma_core::query_core::ArgumentValue> {
                let mut map = prisma_core::IndexMap::new();
                for (k, v) in self.args {
                    map.insert(k, v);
                }
                map
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

pub fn generate_include_builder(_model_name: &syn::Ident, _model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    quote::quote! {}
}

pub fn generate_data_builder(
    model_name: &syn::Ident,
    model_metadata: &ModelMetadata,
    all_metadata: Option<&HashMap<String, ModelMetadata>>,
) -> proc_macro2::TokenStream {
    let data_builder_name = format_ident!("{}DataBuilder", model_name);
    let data_methods = crate::model_analysis::generate_data_methods(model_name, &model_metadata.fields);
    let rel_write_builders = generate_relation_write_builders(model_name, model_metadata, all_metadata);

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
                prisma_core::query_structure::PrismaValue::Null
            }
        }

        #(#rel_write_builders)*
    }
}

pub fn generate_relation_write_builders(
    model_name: &syn::Ident,
    model_metadata: &ModelMetadata,
    all_metadata: Option<&HashMap<String, ModelMetadata>>,
) -> Vec<proc_macro2::TokenStream> {
    model_metadata
        .fields
        .iter()
        .filter(|f| f.is_relation)
        .map(|field| {
            let builder_name = format_ident!(
                "{}{}RelationWriteBuilder",
                model_name,
                field.rust_name.to_upper_camel_case()
            );
            let inner_type_str = crate::utils::get_inner_type(&field.field_type);
            let related_data_builder = format_ident!("{}DataBuilder", inner_type_str);
            let related_unique_where_builder = format_ident!("{}UniqueWhereBuilder", inner_type_str);

            let (create_args, create_inserts) = if let Some(map) = all_metadata {
                if let Some(rel_meta) = map.get(&inner_type_str) {
                    (rel_meta.create_params_with_ignore(field.opposite_relation_field.as_deref()), 
                     rel_meta.create_data_inserts_with_ignore("builder.data", field.opposite_relation_field.as_deref()))
                } else {
                    (quote! {}, quote! {})
                }
            } else {
                (quote! {}, quote! {})
            };

            if field.is_list {
                quote! {
                    #[derive(Default)]
                    pub struct #builder_name {
                        pub data: prisma_core::IndexMap<String, prisma_core::query_core::ArgumentValue>,
                    }

                    impl #builder_name {
                        pub fn create<F>(&mut self, #create_args, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_data_builder) {
                            let mut builder = #related_data_builder::default();
                            #create_inserts
                            f(&mut builder);
                            let val = prisma_core::query_core::ArgumentValue::Object(builder.data);

                            let list = self.data.entry("create".to_string())
                                .or_insert_with(|| prisma_core::query_core::ArgumentValue::List(Vec::new()));

                            if let prisma_core::query_core::ArgumentValue::List(l) = list {
                                l.push(val.clone());
                            }
                            self
                        }

                        pub fn connect<F>(&mut self, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_unique_where_builder) {
                            let mut builder = #related_unique_where_builder::default();
                            f(&mut builder);
                            use prisma_core::FilterBuilder;
                            let val = prisma_core::query_core::ArgumentValue::Object(builder.build());

                            let list = self.data.entry("connect".to_string())
                                .or_insert_with(|| prisma_core::query_core::ArgumentValue::List(Vec::new()));

                            if let prisma_core::query_core::ArgumentValue::List(l) = list {
                                l.push(val.clone());
                            }
                            self
                        }

                        pub fn disconnect<F>(&mut self, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_unique_where_builder) {
                            let mut builder = #related_unique_where_builder::default();
                            f(&mut builder);
                            use prisma_core::FilterBuilder;
                            let val = prisma_core::query_core::ArgumentValue::Object(builder.build());

                            let list = self.data.entry("disconnect".to_string())
                                .or_insert_with(|| prisma_core::query_core::ArgumentValue::List(Vec::new()));

                            if let prisma_core::query_core::ArgumentValue::List(l) = list {
                                l.push(val.clone());
                            }
                            self
                        }
                    }
                }
            } else {
                // To-One relation
                quote! {
                    #[derive(Default)]
                    pub struct #builder_name {
                        pub data: prisma_core::IndexMap<String, prisma_core::query_core::ArgumentValue>,
                    }

                    impl #builder_name {
                        pub fn create<F>(&mut self, #create_args, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_data_builder) {
                            let mut builder = #related_data_builder::default();
                            #create_inserts
                            f(&mut builder);
                            let val = prisma_core::query_core::ArgumentValue::Object(builder.data);
                            
                            let mut wrap = prisma_core::IndexMap::new();
                            wrap.insert("create".to_string(), val);
                            self.data = wrap;
                            self
                        }

                        pub fn connect<F>(&mut self, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_unique_where_builder) {
                            let mut builder = #related_unique_where_builder::default();
                            f(&mut builder);
                            use prisma_core::FilterBuilder;
                            let val = prisma_core::query_core::ArgumentValue::Object(builder.build());
                            
                            let mut wrap = prisma_core::IndexMap::new();
                            wrap.insert("connect".to_string(), val);
                            self.data = wrap;
                            self
                        }

                        pub fn disconnect(&mut self) -> &mut Self {
                            let mut wrap = prisma_core::IndexMap::new();
                            wrap.insert("disconnect".to_string(), prisma_core::query_core::ArgumentValue::Scalar(prisma_core::query_structure::PrismaValue::Boolean(true)));
                            self.data = wrap;
                            self
                        }
                    }
                }
            }
        })
        .collect()
}
