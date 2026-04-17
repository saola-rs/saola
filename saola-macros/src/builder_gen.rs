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
            pub args: Vec<(String, saola_core::query_core::ArgumentValue)>,
        }

        impl saola_core::FilterBuilder for #where_builder_name {
            fn add_arg(&mut self, name: String, value: saola_core::query_core::ArgumentValue) {
                self.args.push((name, value));
            }

            fn build(mut self) -> saola_core::IndexMap<String, saola_core::query_core::ArgumentValue> {
                let mut map = saola_core::IndexMap::new();
                for (k, v) in std::mem::take(&mut self.args) {
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
                    let mut map = saola_core::IndexMap::new();
                    for (k, v) in std::mem::take(&mut builder.args) {
                        map.insert(k, v);
                    }
                    self.args.push(("AND".to_string(), saola_core::query_core::ArgumentValue::Object(map)));
                }
                self
            }

            pub fn or<F>(&mut self, f: F) -> &mut Self
            where F: FnOnce(&mut Self)
            {
                let mut builder = Self::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = saola_core::IndexMap::new();
                    for (k, v) in std::mem::take(&mut builder.args) {
                        map.insert(k, v);
                    }
                    self.args.push(("OR".to_string(), saola_core::query_core::ArgumentValue::List(vec![saola_core::query_core::ArgumentValue::Object(map)])));
                }
                self
            }

            pub fn not<F>(&mut self, f: F) -> &mut Self
            where F: FnOnce(&mut Self)
            {
                let mut builder = Self::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = saola_core::IndexMap::new();
                    for (k, v) in std::mem::take(&mut builder.args) {
                        map.insert(k, v);
                    }
                    self.args.push(("NOT".to_string(), saola_core::query_core::ArgumentValue::Object(map)));
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
            pub args: Vec<(String, saola_core::query_core::ArgumentValue)>,
        }

        impl saola_core::FilterBuilder for #unique_where_builder_name {
            fn add_arg(&mut self, name: String, value: saola_core::query_core::ArgumentValue) {
                self.args.push((name, value));
            }

            fn build(mut self) -> saola_core::IndexMap<String, saola_core::query_core::ArgumentValue> {
                let mut map = saola_core::IndexMap::new();
                for (k, v) in std::mem::take(&mut self.args) {
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
            pub selections: Vec<saola_core::query_core::Selection>,
        }

        impl #select_builder_name {
            pub fn all(&mut self) -> &mut Self {
                for field in &[#(#scalar_field_names),*] {
                    self.selections.push(saola_core::query_core::Selection::with_name(field.to_string()));
                }
                self
            }

            #(#select_methods)*
        }

        impl From<#select_builder_name> for Vec<saola_core::query_core::Selection> {
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
    let scalar_data_builder_name = format_ident!("{}ScalarDataBuilder", model_name);

    let full_data_methods = crate::model_analysis::generate_data_methods(model_name, &model_metadata.fields, false);
    let scalar_data_methods = crate::model_analysis::generate_data_methods(model_name, &model_metadata.fields, true);

    let rel_write_builders = generate_relation_write_builders(model_name, model_metadata, all_metadata);

    quote! {
        #[derive(Default)]
        pub struct #data_builder_name {
            pub data: saola_core::IndexMap<String, saola_core::query_core::ArgumentValue>,
        }

        impl #data_builder_name {
            #(#full_data_methods)*
        }

        impl From<#data_builder_name> for saola_core::query_structure::PrismaValue {
            fn from(_b: #data_builder_name) -> Self {
                saola_core::query_structure::PrismaValue::Null
            }
        }

        #[derive(Default)]
        pub struct #scalar_data_builder_name {
            pub data: saola_core::IndexMap<String, saola_core::query_core::ArgumentValue>,
        }

        impl #scalar_data_builder_name {
            #(#scalar_data_methods)*
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
            let related_where_builder = format_ident!("{}WhereBuilder", inner_type_str);

            let (create_args_vec, create_inserts) = if let Some(map) = all_metadata {
                if let Some(rel_meta) = map.get(&inner_type_str) {
                    (rel_meta.create_params_with_ignore(field.opposite_relation_field.as_deref()), 
                     rel_meta.create_data_inserts_with_ignore("builder.data", field.opposite_relation_field.as_deref()))
                } else {
                    (Vec::new(), quote! {})
                }
            } else {
                (Vec::new(), quote! {})
            };

            let create_args = if create_args_vec.is_empty() {
                quote! {}
            } else {
                quote! { #(#create_args_vec),*, }
            };

            let (upsert_create_args_vec, upsert_create_inserts) = if let Some(map) = all_metadata {
                if let Some(rel_meta) = map.get(&inner_type_str) {
                    (rel_meta.create_params_with_ignore(field.opposite_relation_field.as_deref()), 
                     rel_meta.create_data_inserts_with_ignore("create_builder.data", field.opposite_relation_field.as_deref()))
                } else {
                    (Vec::new(), quote! {})
                }
            } else {
                (Vec::new(), quote! {})
            };

            let upsert_create_args = if upsert_create_args_vec.is_empty() {
                quote! {}
            } else {
                quote! { #(#upsert_create_args_vec),*, }
            };

            if field.is_list {
                quote! {
                    #[derive(Default)]
                    pub struct #builder_name {
                        pub data: saola_core::IndexMap<String, saola_core::query_core::ArgumentValue>,
                    }

                    impl #builder_name {
                        pub fn create<F>(&mut self, #create_args f: F) -> &mut Self
                        where F: FnOnce(&mut #related_data_builder) {
                            let mut builder = #related_data_builder::default();
                            #create_inserts
                            f(&mut builder);
                            let val = saola_core::query_core::ArgumentValue::Object(builder.data);

                            let list = self.data.entry("create".to_string())
                                .or_insert_with(|| saola_core::query_core::ArgumentValue::List(Vec::new()));

                            if let saola_core::query_core::ArgumentValue::List(l) = list {
                                l.push(val.clone());
                            }
                            self
                        }

                        pub fn create_many<F>(&mut self, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_data_builder) {
                            let mut builder = #related_data_builder::default();
                            f(&mut builder);
                            let val = saola_core::query_core::ArgumentValue::Object(builder.data);

                            let list = self.data.entry("createMany".to_string())
                                .or_insert_with(|| {
                                    let mut map = saola_core::IndexMap::new();
                                    map.insert("data".to_string(), saola_core::query_core::ArgumentValue::List(Vec::new()));
                                    saola_core::query_core::ArgumentValue::Object(map)
                                });

                            if let saola_core::query_core::ArgumentValue::Object(map) = list {
                                if let Some(saola_core::query_core::ArgumentValue::List(l)) = map.get_mut("data") {
                                    l.push(val);
                                }
                            }
                            self
                        }

                        pub fn connect<F>(&mut self, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_unique_where_builder) {
                            let mut builder = #related_unique_where_builder::default();
                            f(&mut builder);
                            use saola_core::FilterBuilder;
                            let val = saola_core::query_core::ArgumentValue::Object(builder.build());

                            let list = self.data.entry("connect".to_string())
                                .or_insert_with(|| saola_core::query_core::ArgumentValue::List(Vec::new()));

                            if let saola_core::query_core::ArgumentValue::List(l) = list {
                                l.push(val.clone());
                            }
                            self
                        }

                        pub fn set<F>(&mut self, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_unique_where_builder) {
                            let mut builder = #related_unique_where_builder::default();
                            f(&mut builder);
                            use saola_core::FilterBuilder;
                            let val = saola_core::query_core::ArgumentValue::Object(builder.build());

                            let list = self.data.entry("set".to_string())
                                .or_insert_with(|| saola_core::query_core::ArgumentValue::List(Vec::new()));

                            if let saola_core::query_core::ArgumentValue::List(l) = list {
                                l.push(val.clone());
                            }
                            self
                        }

                        pub fn disconnect<F>(&mut self, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_unique_where_builder) {
                            let mut builder = #related_unique_where_builder::default();
                            f(&mut builder);
                            use saola_core::FilterBuilder;
                            let val = saola_core::query_core::ArgumentValue::Object(builder.build());

                            let list = self.data.entry("disconnect".to_string())
                                .or_insert_with(|| saola_core::query_core::ArgumentValue::List(Vec::new()));

                            if let saola_core::query_core::ArgumentValue::List(l) = list {
                                l.push(val.clone());
                            }
                            self
                        }

                        pub fn delete<F>(&mut self, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_unique_where_builder) {
                            let mut builder = #related_unique_where_builder::default();
                            f(&mut builder);
                            use saola_core::FilterBuilder;
                            let val = saola_core::query_core::ArgumentValue::Object(builder.build());

                            let list = self.data.entry("delete".to_string())
                                .or_insert_with(|| saola_core::query_core::ArgumentValue::List(Vec::new()));

                            if let saola_core::query_core::ArgumentValue::List(l) = list {
                                l.push(val.clone());
                            }
                            self
                        }

                        pub fn update<F>(&mut self, where_f: impl FnOnce(&mut #related_unique_where_builder), data_f: F) -> &mut Self
                        where F: FnOnce(&mut #related_data_builder) {
                            let mut w_builder = #related_unique_where_builder::default();
                            where_f(&mut w_builder);
                            let mut d_builder = #related_data_builder::default();
                            data_f(&mut d_builder);
                            
                            let mut map = saola_core::IndexMap::new();
                            use saola_core::FilterBuilder;
                            map.insert("where".to_string(), saola_core::query_core::ArgumentValue::Object(w_builder.build()));
                            map.insert("data".to_string(), saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut d_builder.data)));
                            
                            let list = self.data.entry("update".to_string())
                                .or_insert_with(|| saola_core::query_core::ArgumentValue::List(Vec::new()));

                            if let saola_core::query_core::ArgumentValue::List(l) = list {
                                l.push(saola_core::query_core::ArgumentValue::Object(map));
                            }
                            self
                        }

                        pub fn update_many<F>(&mut self, where_f: impl FnOnce(&mut #related_where_builder), data_f: F) -> &mut Self
                        where F: FnOnce(&mut #related_data_builder) {
                            let mut w_builder = #related_where_builder::default();
                            where_f(&mut w_builder);
                            let mut d_builder = #related_data_builder::default();
                            data_f(&mut d_builder);
                            
                            let mut map = saola_core::IndexMap::new();
                            use saola_core::FilterBuilder;
                            map.insert("where".to_string(), saola_core::query_core::ArgumentValue::Object(w_builder.build()));
                            map.insert("data".to_string(), saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut d_builder.data)));
                            
                            self.data.insert("updateMany".to_string(), saola_core::query_core::ArgumentValue::Object(map));
                            self
                        }

                        pub fn delete_many(&mut self, f: impl FnOnce(&mut #related_where_builder)) -> &mut Self {
                            let mut w_builder = #related_where_builder::default();
                            f(&mut w_builder);
                            use saola_core::FilterBuilder;
                            self.data.insert("deleteMany".to_string(), saola_core::query_core::ArgumentValue::Object(w_builder.build()));
                            self
                        }

                        pub fn upsert<F>(&mut self, where_f: impl FnOnce(&mut #related_unique_where_builder), #upsert_create_args create_f: impl FnOnce(&mut #related_data_builder), update_f: F) -> &mut Self
                        where F: FnOnce(&mut #related_data_builder) {
                            let mut w_builder = #related_unique_where_builder::default();
                            where_f(&mut w_builder);
                            
                            let mut create_builder = #related_data_builder::default();
                            #upsert_create_inserts
                            create_f(&mut create_builder);
                            
                            let mut update_builder = #related_data_builder::default();
                            update_f(&mut update_builder);
                            
                            let mut map = saola_core::IndexMap::new();
                            use saola_core::FilterBuilder;
                            map.insert("where".to_string(), saola_core::query_core::ArgumentValue::Object(w_builder.build()));
                            map.insert("create".to_string(), saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut create_builder.data)));
                            map.insert("update".to_string(), saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut update_builder.data)));
                            
                            let list = self.data.entry("upsert".to_string())
                                .or_insert_with(|| saola_core::query_core::ArgumentValue::List(Vec::new()));

                            if let saola_core::query_core::ArgumentValue::List(l) = list {
                                l.push(saola_core::query_core::ArgumentValue::Object(map));
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
                        pub data: saola_core::IndexMap<String, saola_core::query_core::ArgumentValue>,
                    }

                    impl #builder_name {
                        pub fn create<F>(&mut self, #create_args f: F) -> &mut Self
                        where F: FnOnce(&mut #related_data_builder) {
                            let mut builder = #related_data_builder::default();
                            #create_inserts
                            f(&mut builder);
                            let val = saola_core::query_core::ArgumentValue::Object(builder.data);
                            
                            let mut wrap = saola_core::IndexMap::new();
                            wrap.insert("create".to_string(), val);
                            self.data = wrap;
                            self
                        }

                        pub fn connect<F>(&mut self, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_unique_where_builder) {
                            let mut builder = #related_unique_where_builder::default();
                            f(&mut builder);
                            use saola_core::FilterBuilder;
                            let val = saola_core::query_core::ArgumentValue::Object(builder.build());
                            
                            let mut wrap = saola_core::IndexMap::new();
                            wrap.insert("connect".to_string(), val);
                            self.data = wrap;
                            self
                        }

                        pub fn disconnect(&mut self) -> &mut Self {
                            let mut wrap = saola_core::IndexMap::new();
                            wrap.insert("disconnect".to_string(), saola_core::query_core::ArgumentValue::Scalar(saola_core::query_structure::PrismaValue::Boolean(true)));
                            self.data = wrap;
                            self
                        }

                        pub fn delete(&mut self) -> &mut Self {
                            let mut wrap = saola_core::IndexMap::new();
                            wrap.insert("delete".to_string(), saola_core::query_core::ArgumentValue::Scalar(saola_core::query_structure::PrismaValue::Boolean(true)));
                            self.data = wrap;
                            self
                        }

                        pub fn update<F>(&mut self, f: F) -> &mut Self
                        where F: FnOnce(&mut #related_data_builder) {
                            let mut builder = #related_data_builder::default();
                            f(&mut builder);
                            
                            let mut wrap = saola_core::IndexMap::new();
                            wrap.insert("update".to_string(), saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)));
                            self.data = wrap;
                            self
                        }

                        pub fn upsert<F>(&mut self, #upsert_create_args create_f: impl FnOnce(&mut #related_data_builder), update_f: F) -> &mut Self
                        where F: FnOnce(&mut #related_data_builder) {
                            let mut create_builder = #related_data_builder::default();
                            #upsert_create_inserts
                            create_f(&mut create_builder);
                            
                            let mut update_builder = #related_data_builder::default();
                            update_f(&mut update_builder);
                            
                            let mut map = saola_core::IndexMap::new();
                            map.insert("create".to_string(), saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut create_builder.data)));
                            map.insert("update".to_string(), saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut update_builder.data)));
                            
                            let mut wrap = saola_core::IndexMap::new();
                            wrap.insert("upsert".to_string(), saola_core::query_core::ArgumentValue::Object(map));
                            self.data = wrap;
                            self
                        }
                    }
                }
            }
        })
        .collect()
}

pub fn generate_aggregate_select_builders(
    model_name: &syn::Ident,
    model_metadata: &ModelMetadata,
) -> proc_macro2::TokenStream {
    let count_builder = format_ident!("{}CountAggregateSelectBuilder", model_name);
    let sum_builder = format_ident!("{}SumAggregateSelectBuilder", model_name);
    let avg_builder = format_ident!("{}AvgAggregateSelectBuilder", model_name);
    let min_builder = format_ident!("{}MinAggregateSelectBuilder", model_name);
    let max_builder = format_ident!("{}MaxAggregateSelectBuilder", model_name);

    let mut count_methods = Vec::new();
    let mut sum_methods = Vec::new();
    let mut avg_methods = Vec::new();
    let mut min_methods = Vec::new();
    let mut max_methods = Vec::new();

    count_methods.push(quote! {
        pub fn _all(&mut self) -> &mut Self {
            self.selections.push(::saola_core::query_core::Selection::with_name("_all"));
            self
        }
    });

    for field in &model_metadata.fields {
        if field.is_relation {
            continue;
        }
        let rust_name = format_ident!("{}", field.rust_name);
        let prisma_name = &field.prisma_name;

        count_methods.push(quote! {
            pub fn #rust_name(&mut self) -> &mut Self {
                self.selections.push(::saola_core::query_core::Selection::with_name(#prisma_name));
                self
            }
        });

        let type_name = crate::utils::get_inner_type(&field.field_type);
        let is_numeric = match type_name.as_str() {
            "i32" | "i64" | "f32" | "f64" | "BigDecimal" => true,
            _ => false,
        };

        if is_numeric {
            sum_methods.push(quote! {
                pub fn #rust_name(&mut self) -> &mut Self {
                    self.selections.push(::saola_core::query_core::Selection::with_name(#prisma_name));
                    self
                }
            });
            avg_methods.push(quote! {
                pub fn #rust_name(&mut self) -> &mut Self {
                    self.selections.push(::saola_core::query_core::Selection::with_name(#prisma_name));
                    self
                }
            });
        }

        min_methods.push(quote! {
            pub fn #rust_name(&mut self) -> &mut Self {
                self.selections.push(::saola_core::query_core::Selection::with_name(#prisma_name));
                self
            }
        });
        max_methods.push(quote! {
            pub fn #rust_name(&mut self) -> &mut Self {
                self.selections.push(::saola_core::query_core::Selection::with_name(#prisma_name));
                self
            }
        });
    }

    quote! {
        #[derive(Default)]
        pub struct #count_builder { pub selections: Vec<::saola_core::query_core::Selection> }
        impl #count_builder { #(#count_methods)* }

        #[derive(Default)]
        pub struct #sum_builder { pub selections: Vec<::saola_core::query_core::Selection> }
        impl #sum_builder { #(#sum_methods)* }

        #[derive(Default)]
        pub struct #avg_builder { pub selections: Vec<::saola_core::query_core::Selection> }
        impl #avg_builder { #(#avg_methods)* }

        #[derive(Default)]
        pub struct #min_builder { pub selections: Vec<::saola_core::query_core::Selection> }
        impl #min_builder { #(#min_methods)* }

        #[derive(Default)]
        pub struct #max_builder { pub selections: Vec<::saola_core::query_core::Selection> }
        impl #max_builder { #(#max_methods)* }
    }
}

pub fn generate_group_by_builder(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let builder_name = format_ident!("{}GroupBySelectBuilder", model_name);
    let mut methods = Vec::new();

    for field in &model_metadata.fields {
        if field.is_relation {
            continue;
        }
        let rust_name = format_ident!("{}", field.rust_name);
        let prisma_name = &field.prisma_name;

        methods.push(quote! {
            pub fn #rust_name(&mut self) -> &mut Self {
                self.fields.push(#prisma_name.to_string());
                self
            }
        });
    }

    quote! {
        #[derive(Default)]
        pub struct #builder_name {
            pub fields: Vec<String>,
        }

        impl #builder_name {
            #(#methods)*
        }
    }
}

pub fn generate_order_by_builder(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let order_by_builder_name = format_ident!("{}OrderByBuilder", model_name);
    let order_by_methods = crate::model_analysis::generate_order_by_methods(&model_metadata.fields);

    quote! {
        #[derive(Default)]
        pub struct #order_by_builder_name {
            pub args: Vec<saola_core::ArgumentValue>,
        }

        impl #order_by_builder_name {
            #(#order_by_methods)*
        }
    }
}
