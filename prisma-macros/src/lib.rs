extern crate proc_macro;

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{GenericArgument, ItemStruct, PathArguments, Type, parse_macro_input};

fn get_inner_type(ty: &Type) -> String {
    match ty {
        Type::Path(tp) => {
            let segment = tp.path.segments.last().unwrap();
            match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        get_inner_type(inner_ty)
                    } else {
                        segment.ident.to_string()
                    }
                }
                _ => segment.ident.to_string(),
            }
        }
        _ => "String".to_string(),
    }
}

#[proc_macro_attribute]
pub fn prisma_model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);
    let model_name = &input.ident;
    let model_name_str = model_name.to_string();
    let model_name_snake = format_ident!("{}", model_name_str.to_snake_case());

    let mut filter_methods = Vec::new();
    let mut unique_filter_methods = Vec::new();
    let mut select_methods = Vec::new();
    let mut include_methods = Vec::new();
    let mut filter_struct_defs = Vec::new();
    let mut data_methods = Vec::new();

    let mut create_args = Vec::new();
    let mut create_args_push = Vec::new();

    let mut scalar_field_names = Vec::new();

    let where_builder_name = format_ident!("{}WhereBuilder", model_name);
    let unique_where_builder_name = format_ident!("{}UniqueWhereBuilder", model_name);
    let select_builder_name = format_ident!("{}SelectBuilder", model_name);
    let include_builder_name = format_ident!("{}IncludeBuilder", model_name);
    let data_builder_name = format_ident!("{}DataBuilder", model_name);

    for field in &mut input.fields {
        let rust_field_name = field.ident.as_ref().unwrap();
        let rust_field_name_str = rust_field_name.to_string();

        let mut prisma_field_name = rust_field_name_str.clone();
        let mut is_relation = false;
        let mut is_unique = false;

        let is_optional = {
            if let Type::Path(tp) = &field.ty {
                tp.path.segments.last().unwrap().ident == "Option"
            } else {
                false
            }
        };
        let mut is_id = false;

        field.attrs.retain(|attr| {
            if attr.path().is_ident("prisma") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        let s: syn::LitStr = value.parse()?;
                        prisma_field_name = s.value();
                    } else if meta.path.is_ident("relation") {
                        is_relation = true;
                    } else if meta.path.is_ident("id") {
                        is_id = true;
                        is_unique = true;
                    } else if meta.path.is_ident("unique") {
                        is_unique = true;
                    }
                    Ok(())
                });
                false
            } else {
                true
            }
        });

        if !is_relation {
            scalar_field_names.push(prisma_field_name.clone());

            if !is_optional && !is_id {
                let ty = &field.ty;
                create_args.push(quote! { #rust_field_name: #ty });
                create_args_push.push(quote! {
                    data_map.insert(#prisma_field_name.to_string(), prisma_core::query_core::ArgumentValue::Scalar(prisma_core::query_structure::PrismaValue::from(#rust_field_name)));
                });
            }

            if is_unique {
                unique_filter_methods.push(quote! {
                    pub fn #rust_field_name<T>(&mut self, value: T) -> &mut Self 
                    where T: Into<prisma_core::query_structure::PrismaValue>
                    {
                        use prisma_core::FilterBuilder;
                        self.add_arg(#prisma_field_name.to_string(), prisma_core::query_core::ArgumentValue::Scalar(value.into()));
                        self
                    }
                });
            }
        }

        let filter_name_str = format!("{}{}Filter", model_name, rust_field_name_str.to_upper_camel_case());
        let filter_name = format_ident!("{}", filter_name_str);

        if is_relation {
            let inner_type_str = get_inner_type(&field.ty);
            let related_select_builder = format_ident!("{}SelectBuilder", inner_type_str);
            let related_data_builder = format_ident!("{}DataBuilder", inner_type_str);

            include_methods.push(quote! {
                pub fn #rust_field_name<F>(&mut self, f: F) -> &mut Self
                where F: FnOnce(&mut #related_select_builder)
                {
                    let mut builder = #related_select_builder::default();
                    f(&mut builder);
                    let mut sel = prisma_core::query_core::Selection::with_name(#prisma_field_name.to_string());
                    let fields: Vec<String> = builder.into();
                    for field in fields {
                        sel.push_nested_selection(prisma_core::query_core::Selection::with_name(field));
                    }
                    self.includes.push(sel);
                    self
                }
            });

            data_methods.push(quote! {
                pub fn #rust_field_name<F>(&mut self, f: F) -> &mut Self
                where F: FnOnce(&mut #related_data_builder)
                {
                    let mut builder = #related_data_builder::default();
                    f(&mut builder);
                    
                    let mut create_map = prisma_core::IndexMap::new();
                    create_map.insert("create".to_string(), prisma_core::query_core::ArgumentValue::Object(builder.data));
                    
                    self.data.insert(#prisma_field_name.to_string(), prisma_core::query_core::ArgumentValue::Object(create_map));
                    self
                }
            });
        } else {
            filter_struct_defs.push(quote! {
                pub struct #filter_name<'a, B> {
                    builder: &'a mut B,
                    prisma_name: &'static str,
                }
                impl<'a, B> #filter_name<'a, B> {
                    fn add_op<T>(self, op: &str, value: T) -> &'a mut B
                    where 
                        T: Into<prisma_core::query_structure::PrismaValue>,
                        B: prisma_core::FilterBuilder
                    {
                        let mut map = prisma_core::IndexMap::new();
                        map.insert(op.to_string(), prisma_core::query_core::ArgumentValue::Scalar(value.into()));
                        self.builder.add_arg(self.prisma_name.to_string(), prisma_core::query_core::ArgumentValue::Object(map));
                        self.builder
                    }

                    pub fn eq<T>(self, value: T) -> &'a mut B where T: Into<prisma_core::query_structure::PrismaValue>, B: prisma_core::FilterBuilder { self.add_op("equals", value) }
                    pub fn not<T>(self, value: T) -> &'a mut B where T: Into<prisma_core::query_structure::PrismaValue>, B: prisma_core::FilterBuilder { self.add_op("not", value) }
                    pub fn gt<T>(self, value: T) -> &'a mut B where T: Into<prisma_core::query_structure::PrismaValue>, B: prisma_core::FilterBuilder { self.add_op("gt", value) }
                    pub fn gte<T>(self, value: T) -> &'a mut B where T: Into<prisma_core::query_structure::PrismaValue>, B: prisma_core::FilterBuilder { self.add_op("gte", value) }
                    pub fn lt<T>(self, value: T) -> &'a mut B where T: Into<prisma_core::query_structure::PrismaValue>, B: prisma_core::FilterBuilder { self.add_op("lt", value) }
                    pub fn lte<T>(self, value: T) -> &'a mut B where T: Into<prisma_core::query_structure::PrismaValue>, B: prisma_core::FilterBuilder { self.add_op("lte", value) }
                    pub fn contains<T>(self, value: T) -> &'a mut B where T: Into<prisma_core::query_structure::PrismaValue>, B: prisma_core::FilterBuilder { self.add_op("contains", value) }
                    pub fn starts_with<T>(self, value: T) -> &'a mut B where T: Into<prisma_core::query_structure::PrismaValue>, B: prisma_core::FilterBuilder { self.add_op("startsWith", value) }
                    pub fn ends_with<T>(self, value: T) -> &'a mut B where T: Into<prisma_core::query_structure::PrismaValue>, B: prisma_core::FilterBuilder { self.add_op("endsWith", value) }
                }
            });

            filter_methods.push(quote! {
                pub fn #rust_field_name(&mut self) -> #filter_name<'_, Self> {
                    #filter_name {
                        builder: self,
                        prisma_name: #prisma_field_name,
                    }
                }
            });

            select_methods.push(quote! {
                pub fn #rust_field_name(&mut self) -> &mut Self {
                    self.fields.push(#prisma_field_name.to_string());
                    self
                }
            });

            data_methods.push(quote! {
                pub fn #rust_field_name<T>(&mut self, value: T) -> &mut Self 
                where T: Into<prisma_core::query_structure::PrismaValue>
                {
                    self.data.insert(#prisma_field_name.to_string(), prisma_core::query_core::ArgumentValue::Scalar(value.into()));
                    self
                }
            });
        }
    }

    let query_name = format_ident!("{}Query", model_name);
    let read_wrapper_name = format_ident!("{}ReadBuilder", model_name);
    let write_wrapper_name = format_ident!("{}WriteBuilder", model_name);
    let count_wrapper_name = format_ident!("{}CountBuilder", model_name);
    let aggregate_wrapper_name = format_ident!("{}AggregateBuilder", model_name);
    let group_by_wrapper_name = format_ident!("{}GroupByBuilder", model_name);

    let expanded = quote! {
        #input

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
            #(#unique_filter_methods)*
        }

        #(#filter_struct_defs)*

        #[derive(Default)]
        pub struct #select_builder_name {
            pub fields: Vec<String>,
        }
        impl #select_builder_name {
            pub fn all(&mut self) -> &mut Self {
                self.fields.extend(vec![#(#scalar_field_names.to_string()),*]);
                self
            }
            #(#select_methods)*
        }
        impl From<#select_builder_name> for Vec<String> {
            fn from(b: #select_builder_name) -> Self { b.fields }
        }

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

        #[derive(Default)]
        pub struct #data_builder_name {
            pub data: prisma_core::IndexMap<String, prisma_core::query_core::ArgumentValue>,
        }
        impl #data_builder_name {
            #(#data_methods)*
        }
        impl From<#data_builder_name> for prisma_core::query_core::ArgumentValue {
            fn from(b: #data_builder_name) -> Self {
                prisma_core::query_core::ArgumentValue::Object(b.data)
            }
        }

        // ============ THIN WRAPPER BUILDERS FOR COMPOSABILITY ============
        
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
                let includes: Vec<prisma_core::query_core::Selection> = builder.into();
                use prisma_core::Selectable;
                for sel in includes {
                    self.inner.add_nested_selection(sel);
                }
                self
            }

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::PrismaClient) -> Result<T, prisma_core::anyhow::Error> {
                use prisma_core::Executable;
                self.inner.exec(client).await
            }
        }

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

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::PrismaClient) -> Result<T, prisma_core::anyhow::Error> {
                use prisma_core::Executable;
                self.inner.exec(client).await
            }
        }

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

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::PrismaClient) -> Result<T, prisma_core::anyhow::Error> {
                use prisma_core::Executable;
                self.inner.exec(client).await
            }
        }

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

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::PrismaClient) -> Result<T, prisma_core::anyhow::Error> {
                use prisma_core::Executable;
                self.inner.exec(client).await
            }
        }

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

            pub async fn exec<T: prisma_core::serde::de::DeserializeOwned>(self, client: &prisma_core::PrismaClient) -> Result<T, prisma_core::anyhow::Error> {
                use prisma_core::Executable;
                self.inner.exec(client).await
            }
        }

        // ============ QUERY FACTORY ============

        pub struct #query_name;

        pub fn #model_name_snake() -> #query_name {
            #query_name
        }

        impl #query_name {
            // ============ READ OPERATIONS ============
            pub fn find_many(&self) -> #read_wrapper_name {
                #read_wrapper_name {
                    inner: prisma_core::ReadBuilder::find_many(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            pub fn find_unique(&self) -> #read_wrapper_name {
                #read_wrapper_name {
                    inner: prisma_core::ReadBuilder::find_unique(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            pub fn find_first(&self) -> #read_wrapper_name {
                #read_wrapper_name {
                    inner: prisma_core::ReadBuilder::find_first(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            pub fn find_first_or_throw(&self) -> #read_wrapper_name {
                #read_wrapper_name {
                    inner: prisma_core::ReadBuilder::find_first_or_throw(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            pub fn find_unique_or_throw(&self) -> #read_wrapper_name {
                #read_wrapper_name {
                    inner: prisma_core::ReadBuilder::find_unique_or_throw(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            // ============ WRITE OPERATIONS ============
            pub fn create(&self, #(#create_args),*) -> #write_wrapper_name {
                let mut builder = prisma_core::WriteBuilder::create(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*]);
                let mut data_map = prisma_core::IndexMap::new();
                #(#create_args_push)*
                use prisma_core::Filterable;
                builder.add_filter_arg("data".to_string(), prisma_core::query_core::ArgumentValue::Object(data_map));
                #write_wrapper_name { inner: builder }
            }

            pub fn update(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::update(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            pub fn delete(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::delete(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            pub fn create_many(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::create_many(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            pub fn update_many(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::update_many(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            pub fn delete_many(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::delete_many(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            pub fn upsert(&self) -> #write_wrapper_name {
                #write_wrapper_name {
                    inner: prisma_core::WriteBuilder::upsert(#model_name_str.to_string(), vec![#(#scalar_field_names.to_string()),*])
                }
            }

            // ============ AGGREGATION OPERATIONS ============
            pub fn count(&self) -> #count_wrapper_name {
                #count_wrapper_name {
                    inner: prisma_core::CountBuilder::new(#model_name_str.to_string(), vec![])
                }
            }

            pub fn aggregate(&self) -> #aggregate_wrapper_name {
                #aggregate_wrapper_name {
                    inner: prisma_core::AggregateBuilder::new(#model_name_str.to_string(), vec![])
                }
            }

            pub fn group_by(&self) -> #group_by_wrapper_name {
                #group_by_wrapper_name {
                    inner: prisma_core::GroupByBuilder::new(#model_name_str.to_string(), vec![])
                }
            }
        }
    };

    TokenStream::from(expanded)
}
