//! Thin wrapper builders for composability (ReadBuilder, WriteBuilder, CountBuilder, etc.)

use crate::model_analysis::ModelMetadata;
use heck::ToUpperCamelCase;
use quote::{format_ident, quote};

fn generate_include_methods(
    model_name: &syn::Ident,
    model_metadata: &ModelMetadata,
    wrapper_name: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    let mut methods = Vec::new();
    let scalar_field_names = &model_metadata.scalar_field_names;

    for field in &model_metadata.fields {
        if field.is_relation {
            let rust_name = format_ident!("{}", field.rust_name);
            let prisma_name = &field.prisma_name;

            let with_relation_type = format_ident!("{}With{}", model_name, field.rust_name.to_upper_camel_case());
            let type_str = crate::utils::get_inner_type(&field.field_type);
            let related_select_builder = format_ident!("{}SelectBuilder", type_str);

            methods.push(quote! {
                pub fn #rust_name(self) -> #wrapper_name<#with_relation_type> {
                    let mut inner = self.inner;
                    use ::prisma_core::Selectable;

                    for field in &[#(#scalar_field_names),*] {
                        inner.add_nested_selection(::prisma_core::query_core::Selection::with_name(field.to_string()));
                    }

                    let mut builder = #related_select_builder::default();
                    builder.all();
                    let mut sel = ::prisma_core::query_core::Selection::with_name(#prisma_name.to_string());
                    let fields: Vec<::prisma_core::query_core::Selection> = builder.into();
                    for f in fields {
                        sel.push_nested_selection(f);
                    }
                    inner.add_nested_selection(sel);

                    #wrapper_name {
                        inner: inner.with_type(),
                        _phantom: std::marker::PhantomData,
                    }
                }
            });
        }
    }
    methods
}

pub fn generate_read_wrappers(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let many_name = format_ident!("{}ManyReadBuilder", model_name);
    let unique_name = format_ident!("{}UniqueReadBuilder", model_name);
    let first_name = format_ident!("{}FirstReadBuilder", model_name);
    let unique_throw_name = format_ident!("{}UniqueOrThrowReadBuilder", model_name);
    let first_throw_name = format_ident!("{}FirstOrThrowReadBuilder", model_name);

    let where_name = format_ident!("{}WhereBuilder", model_name);
    let unique_where_name = format_ident!("{}UniqueWhereBuilder", model_name);
    let select_name = format_ident!("{}SelectBuilder", model_name);
    let include_name = format_ident!("{}IncludeBuilder", model_name);
    let scalar_field_names = &model_metadata.scalar_field_names;

    let many_includes = generate_include_methods(model_name, model_metadata, &many_name);
    let unique_includes = generate_include_methods(model_name, model_metadata, &unique_name);
    let first_includes = generate_include_methods(model_name, model_metadata, &first_name);
    let unique_throw_includes = generate_include_methods(model_name, model_metadata, &unique_throw_name);
    let first_throw_includes = generate_include_methods(model_name, model_metadata, &first_throw_name);

    quote! {
        /// findMany (Vec<T>, RegularWhere)
        pub struct #many_name<T = #model_name> {
            inner: prisma_core::ReadBuilder<Vec<T>>,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T: prisma_core::serde::de::DeserializeOwned + Send + Sync> #many_name<T> {
            pub fn where_clause<F>(mut self, f: F) -> Self where F: FnOnce(&mut #where_name) {
                let mut builder = #where_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args { map.insert(k, v); }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn select<F>(mut self, f: F) -> #many_name<::prisma_core::serde_json::Value> where F: FnOnce(&mut #select_name) {
                let mut builder = #select_name::default();
                f(&mut builder);
                let selections: Vec<prisma_core::query_core::Selection> = builder.into();
                use prisma_core::Selectable;
                self.inner.state.selection.clear_nested_selections();
                for sel in selections { self.inner.add_nested_selection(sel); }
                #many_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            pub fn include<F>(mut self, f: F) -> Self where F: FnOnce(&mut #include_name) {
                let mut builder = #include_name::default();
                f(&mut builder);
                use prisma_core::Selectable;
                for scalar_field in &[#(#scalar_field_names),*] {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(scalar_field.to_string()));
                }
                let includes: Vec<prisma_core::query_core::Selection> = builder.into();
                for sel in includes { self.inner.add_nested_selection(sel); }
                self
            }

            pub fn r#as<U: prisma_core::serde::de::DeserializeOwned + Send + Sync>(self) -> #many_name<U> {
                #many_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            #(#many_includes)*

            pub async fn exec(self, client: &prisma_core::client::PrismaClient) -> prisma_core::Result<Vec<T>> {
                self.inner.exec_inferred(client).await
            }
        }

        /// findUnique (Option<T>, UniqueWhere)
        pub struct #unique_name<T = #model_name> {
            inner: prisma_core::ReadBuilder<Option<T>>,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T: prisma_core::serde::de::DeserializeOwned + Send + Sync> #unique_name<T> {
            pub fn where_clause<F>(mut self, f: F) -> Self where F: FnOnce(&mut #unique_where_name) {
                let mut builder = #unique_where_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args { map.insert(k, v); }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn select<F>(mut self, f: F) -> #unique_name<::prisma_core::serde_json::Value> where F: FnOnce(&mut #select_name) {
                let mut builder = #select_name::default();
                f(&mut builder);
                let selections: Vec<prisma_core::query_core::Selection> = builder.into();
                use prisma_core::Selectable;
                self.inner.state.selection.clear_nested_selections();
                for sel in selections { self.inner.add_nested_selection(sel); }
                #unique_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            pub fn include<F>(mut self, f: F) -> Self where F: FnOnce(&mut #include_name) {
                let mut builder = #include_name::default();
                f(&mut builder);
                use prisma_core::Selectable;
                for scalar_field in &[#(#scalar_field_names),*] {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(scalar_field.to_string()));
                }
                let includes: Vec<prisma_core::query_core::Selection> = builder.into();
                for sel in includes { self.inner.add_nested_selection(sel); }
                self
            }

            pub fn r#as<U: prisma_core::serde::de::DeserializeOwned + Send + Sync>(self) -> #unique_name<U> {
                #unique_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            #(#unique_includes)*

            pub async fn exec(self, client: &prisma_core::client::PrismaClient) -> prisma_core::Result<Option<T>> {
                self.inner.exec_inferred(client).await
            }
        }

        /// findFirst (Option<T>, RegularWhere)
        pub struct #first_name<T = #model_name> {
            inner: prisma_core::ReadBuilder<Option<T>>,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T: prisma_core::serde::de::DeserializeOwned + Send + Sync> #first_name<T> {
            pub fn where_clause<F>(mut self, f: F) -> Self where F: FnOnce(&mut #where_name) {
                let mut builder = #where_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args { map.insert(k, v); }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn select<F>(mut self, f: F) -> #first_name<::prisma_core::serde_json::Value> where F: FnOnce(&mut #select_name) {
                let mut builder = #select_name::default();
                f(&mut builder);
                let selections: Vec<prisma_core::query_core::Selection> = builder.into();
                use prisma_core::Selectable;
                self.inner.state.selection.clear_nested_selections();
                for sel in selections { self.inner.add_nested_selection(sel); }
                #first_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            pub fn include<F>(mut self, f: F) -> Self where F: FnOnce(&mut #include_name) {
                let mut builder = #include_name::default();
                f(&mut builder);
                use prisma_core::Selectable;
                for scalar_field in &[#(#scalar_field_names),*] {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(scalar_field.to_string()));
                }
                let includes: Vec<prisma_core::query_core::Selection> = builder.into();
                for sel in includes { self.inner.add_nested_selection(sel); }
                self
            }

            pub fn r#as<U: prisma_core::serde::de::DeserializeOwned + Send + Sync>(self) -> #first_name<U> {
                #first_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            #(#first_includes)*

            pub async fn exec(self, client: &prisma_core::client::PrismaClient) -> prisma_core::Result<Option<T>> {
                self.inner.exec_inferred(client).await
            }
        }

        /// findUniqueOrThrow (T, UniqueWhere)
        pub struct #unique_throw_name<T = #model_name> {
            inner: prisma_core::ReadBuilder<T>,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T: prisma_core::serde::de::DeserializeOwned + Send + Sync> #unique_throw_name<T> {
            pub fn where_clause<F>(mut self, f: F) -> Self where F: FnOnce(&mut #unique_where_name) {
                let mut builder = #unique_where_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args { map.insert(k, v); }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn select<F>(mut self, f: F) -> #unique_throw_name<::prisma_core::serde_json::Value> where F: FnOnce(&mut #select_name) {
                let mut builder = #select_name::default();
                f(&mut builder);
                let selections: Vec<prisma_core::query_core::Selection> = builder.into();
                use prisma_core::Selectable;
                self.inner.state.selection.clear_nested_selections();
                for sel in selections { self.inner.add_nested_selection(sel); }
                #unique_throw_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            pub fn include<F>(mut self, f: F) -> Self where F: FnOnce(&mut #include_name) {
                let mut builder = #include_name::default();
                f(&mut builder);
                use prisma_core::Selectable;
                for scalar_field in &[#(#scalar_field_names),*] {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(scalar_field.to_string()));
                }
                let includes: Vec<prisma_core::query_core::Selection> = builder.into();
                for sel in includes { self.inner.add_nested_selection(sel); }
                self
            }

            pub fn r#as<U: prisma_core::serde::de::DeserializeOwned + Send + Sync>(self) -> #unique_throw_name<U> {
                #unique_throw_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            #(#unique_throw_includes)*

            pub async fn exec(self, client: &prisma_core::client::PrismaClient) -> prisma_core::Result<T> {
                self.inner.exec_inferred(client).await
            }
        }

        /// findFirstOrThrow (T, RegularWhere)
        pub struct #first_throw_name<T = #model_name> {
            inner: prisma_core::ReadBuilder<T>,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T: prisma_core::serde::de::DeserializeOwned + Send + Sync> #first_throw_name<T> {
            pub fn where_clause<F>(mut self, f: F) -> Self where F: FnOnce(&mut #where_name) {
                let mut builder = #where_name::default();
                f(&mut builder);
                if !builder.args.is_empty() {
                    let mut map = prisma_core::IndexMap::new();
                    for (k, v) in builder.args { map.insert(k, v); }
                    use prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn select<F>(mut self, f: F) -> #first_throw_name<::prisma_core::serde_json::Value> where F: FnOnce(&mut #select_name) {
                let mut builder = #select_name::default();
                f(&mut builder);
                let selections: Vec<prisma_core::query_core::Selection> = builder.into();
                use prisma_core::Selectable;
                self.inner.state.selection.clear_nested_selections();
                for sel in selections { self.inner.add_nested_selection(sel); }
                #first_throw_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            pub fn include<F>(mut self, f: F) -> Self where F: FnOnce(&mut #include_name) {
                let mut builder = #include_name::default();
                f(&mut builder);
                use prisma_core::Selectable;
                for scalar_field in &[#(#scalar_field_names),*] {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(scalar_field.to_string()));
                }
                let includes: Vec<prisma_core::query_core::Selection> = builder.into();
                for sel in includes { self.inner.add_nested_selection(sel); }
                self
            }

            pub fn r#as<U: prisma_core::serde::de::DeserializeOwned + Send + Sync>(self) -> #first_throw_name<U> {
                #first_throw_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            #(#first_throw_includes)*

            pub async fn exec(self, client: &prisma_core::client::PrismaClient) -> prisma_core::Result<T> {
                self.inner.exec_inferred(client).await
            }
        }
    }
}

pub fn generate_write_wrapper(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let write_wrapper_name = format_ident!("{}WriteBuilder", model_name);
    let unique_where_builder_name = format_ident!("{}UniqueWhereBuilder", model_name);
    let select_builder_name = format_ident!("{}SelectBuilder", model_name);
    let data_builder_name = format_ident!("{}DataBuilder", model_name);
    let include_builder_name = format_ident!("{}IncludeBuilder", model_name);
    let scalar_field_names = &model_metadata.scalar_field_names;

    quote! {
        /// Builder for write operations (returns T)
        pub struct #write_wrapper_name<T = #model_name> {
            inner: prisma_core::WriteBuilder<T>,
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T: prisma_core::serde::de::DeserializeOwned + Send + Sync> #write_wrapper_name<T> {
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

            pub fn select<F>(mut self, f: F) -> #write_wrapper_name<::prisma_core::serde_json::Value>
            where F: FnOnce(&mut #select_builder_name)
            {
                let mut builder = #select_builder_name::default();
                f(&mut builder);
                let selections: Vec<prisma_core::query_core::Selection> = builder.into();
                use prisma_core::Selectable;
                self.inner.state.selection.clear_nested_selections();
                for sel in selections {
                    self.inner.add_nested_selection(sel);
                }
                #write_wrapper_name {
                    inner: self.inner.with_type(),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn include<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #include_builder_name)
            {
                let mut builder = #include_builder_name::default();
                f(&mut builder);

                use prisma_core::Selectable;
                for scalar_field in &[#(#scalar_field_names),*] {
                    self.inner.add_nested_selection(prisma_core::query_core::Selection::with_name(scalar_field.to_string()));
                }

                let includes: Vec<prisma_core::query_core::Selection> = builder.into();
                for sel in includes {
                    self.inner.add_nested_selection(sel);
                }
                self
            }

            pub fn r#as<U: prisma_core::serde::de::DeserializeOwned + Send + Sync>(self) -> #write_wrapper_name<U> {
                #write_wrapper_name {
                    inner: self.inner.with_type(),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub async fn exec(self, client: &prisma_core::client::PrismaClient) -> prisma_core::Result<T> {
                self.inner.exec_inferred(client).await
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

            pub async fn exec(self, client: &prisma_core::client::PrismaClient) -> prisma_core::Result<i64> {
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

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::client::PrismaClient) -> prisma_core::Result<T> {
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

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::client::PrismaClient) -> prisma_core::Result<T> {
                use prisma_core::Executable;
                self.inner.exec(client).await
            }
        }
    }
}
