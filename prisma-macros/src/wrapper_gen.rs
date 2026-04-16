//! Thin wrapper builders for composability (ReadBuilder, WriteBuilder, CountBuilder, etc.)

use crate::model_analysis::ModelMetadata;
use heck::ToUpperCamelCase;
use quote::{format_ident, quote};

fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
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

    let include_marker_trait = format_ident!("{}IncludeMarker", model_name);
    let include_transition_trait = format_ident!("{}IncludeTransition", model_name);

    let relations: Vec<_> = model_metadata.fields.iter().filter(|f| f.is_relation).collect();

    let mut include_markers = Vec::new();
    let mut include_methods = Vec::new();
    let mut transitions = Vec::new();

    // Define the Empty marker for this model
    let empty_marker_name = format_ident!("{}IncludeEmpty", model_name);
    include_markers.push(quote! {
        pub struct #empty_marker_name;
        impl #include_marker_trait for #empty_marker_name {
            fn into_selection(self) -> Option<::prisma_core::query_core::Selection> { None }
        }
    });

    for relation in &relations {
        let pascal_name = format_ident!("{}Include{}", model_name, pascal_case(&relation.prisma_name));
        let pascal_name_with = format_ident!("{}Include{}With", model_name, pascal_case(&relation.prisma_name));
        let pascal_name_as = format_ident!("{}Include{}As", model_name, pascal_case(&relation.prisma_name));
        let rust_name = format_ident!("{}", relation.rust_name);
        let with_suffix_name = format_ident!("{}_with", relation.rust_name);
        let as_suffix_name = format_ident!("{}_as", relation.rust_name);
        let inner_type_str = crate::utils::get_inner_type(&relation.field_type);
        let related_select_builder = format_ident!("{}SelectBuilder", inner_type_str);
        let related_include_builder = format_ident!("{}IncludeBuilder", inner_type_str);
        let related_include_marker_trait = format_ident!("{}IncludeMarker", inner_type_str);
        let prisma_name = &relation.prisma_name;

        include_markers.push(quote! {
            pub struct #pascal_name { pub selection: ::prisma_core::query_core::Selection }
            impl #include_marker_trait for #pascal_name {
                fn into_selection(self) -> Option<::prisma_core::query_core::Selection> { Some(self.selection) }
            }
            
            pub struct #pascal_name_with<M> { 
                pub selection: ::prisma_core::query_core::Selection,
                pub _phantom: std::marker::PhantomData<M>
            }
            impl<M> #include_marker_trait for #pascal_name_with<M> {
                fn into_selection(self) -> Option<::prisma_core::query_core::Selection> { Some(self.selection) }
            }

            pub struct #pascal_name_as<U> { 
                pub selection: ::prisma_core::query_core::Selection,
                pub _phantom: std::marker::PhantomData<U>
            }
            impl<U> #include_marker_trait for #pascal_name_as<U> {
                fn into_selection(self) -> Option<::prisma_core::query_core::Selection> { Some(self.selection) }
            }
        });

        include_methods.push(quote! {
            pub fn #rust_name(&mut self) -> #pascal_name {
                let mut builder = #related_select_builder::default();
                builder.all();
                let mut sel = ::prisma_core::query_core::Selection::with_name(#prisma_name.to_string());
                let fields: Vec<::prisma_core::query_core::Selection> = builder.into();
                for f in fields { sel.push_nested_selection(f); }
                #pascal_name { selection: sel }
            }

            pub fn #with_suffix_name<M, F>(&mut self, f: F) -> #pascal_name_with<M>
            where 
                F: FnOnce(&mut #related_include_builder) -> M,
                M: #related_include_marker_trait
            {
                let mut builder = #related_include_builder::default();
                let marker = f(&mut builder);
                let mut sel = ::prisma_core::query_core::Selection::with_name(#prisma_name.to_string());
                for scalar in #related_include_builder::scalar_selections() {
                    sel.push_nested_selection(scalar);
                }
                if let Some(nested) = marker.into_selection() {
                    sel.push_nested_selection(nested);
                }
                for (k, v) in std::mem::take(&mut builder.args) {
                    sel.push_argument(k, v);
                }
                #pascal_name_with { selection: sel, _phantom: std::marker::PhantomData }
            }

            pub fn #as_suffix_name<U: ::prisma_core::serde::de::DeserializeOwned + Send + Sync, F>(&mut self, selection: (std::marker::PhantomData<U>, F)) -> #pascal_name_as<U>
            where F: FnOnce(&mut #related_select_builder)
            {
                let mut builder = #related_select_builder::default();
                (selection.1)(&mut builder);
                let mut sel = ::prisma_core::query_core::Selection::with_name(#prisma_name.to_string());
                let fields: Vec<::prisma_core::query_core::Selection> = builder.into();
                for f in fields { sel.push_nested_selection(f); }
                #pascal_name_as { selection: sel, _phantom: std::marker::PhantomData }
            }
        });
    }

    let mut powerset = vec![vec![]];
    for relation in &relations {
        let mut new_subsets = Vec::new();
        for subset in &powerset {
            let mut new_subset = subset.clone();
            new_subset.push(*relation);
            new_subsets.push(new_subset);
        }
        powerset.extend(new_subsets);
    }

    for subset in powerset {
        let mut sorted_subset = subset.clone();
        sorted_subset.sort_by_key(|r| &r.prisma_name);
        
        let combined_name = if sorted_subset.is_empty() {
            format_ident!("{}", model_name)
        } else {
            let names: Vec<_> = sorted_subset.iter().map(|r| pascal_case(&r.prisma_name)).collect();
            format_ident!("{}With{}", model_name, names.join("And"))
        };

        let mut current_generics_params = Vec::new();
        for r in &sorted_subset {
            let g = format_ident!("T{}", pascal_case(&r.prisma_name));
            current_generics_params.push(g);
        }
        let current_generics_decl = if current_generics_params.is_empty() { quote!{} } else { quote!{ <#(#current_generics_params),*> } };

        let mut impl_generics_base_list = Vec::new();
        for g in &current_generics_params { impl_generics_base_list.push(quote!{#g: ::prisma_core::serde::de::DeserializeOwned + Send + Sync}); }
        let impl_generics_base = if impl_generics_base_list.is_empty() { quote!{} } else { quote!{ <#(#impl_generics_base_list),*> } };

        // Empty transition for this type
        transitions.push(quote! {
            impl #impl_generics_base #include_transition_trait<#empty_marker_name> for #combined_name #current_generics_decl { 
                type Output = #combined_name #current_generics_decl; 
            }
        });

        for relation in &relations {
            let pascal_name = format_ident!("{}Include{}", model_name, pascal_case(&relation.prisma_name));
            let pascal_name_with = format_ident!("{}Include{}With", model_name, pascal_case(&relation.prisma_name));
            let pascal_name_as = format_ident!("{}Include{}As", model_name, pascal_case(&relation.prisma_name));
            let related_model = format_ident!("{}", crate::utils::get_inner_type(&relation.field_type));
            let related_include_transition_trait = format_ident!("{}IncludeTransition", related_model);

            let mut next_subset = subset.clone();
            let is_new_relation = !next_subset.iter().any(|r| r.prisma_name == relation.prisma_name);
            if is_new_relation {
                next_subset.push(*relation);
            }
            next_subset.sort_by_key(|r| &r.prisma_name);
            
            let next_combined_name = if next_subset.is_empty() {
                format_ident!("{}", model_name)
            } else {
                let names: Vec<_> = next_subset.iter().map(|r| pascal_case(&r.prisma_name)).collect();
                format_ident!("{}With{}", model_name, names.join("And"))
            };

            let mut next_generics_base = Vec::new();
            for r in &next_subset {
                if r.prisma_name == relation.prisma_name {
                    next_generics_base.push(quote! { #related_model });
                } else {
                    let g = format_ident!("T{}", pascal_case(&r.prisma_name));
                    next_generics_base.push(quote! { #g });
                }
            }
            let next_generics_base_tokens = if next_generics_base.is_empty() { quote!{} } else { quote!{ <#(#next_generics_base),*> } };

            let mut next_generics_with = Vec::new();
            for r in &next_subset {
                if r.prisma_name == relation.prisma_name {
                    next_generics_with.push(quote! { <#related_model as #related_include_transition_trait<M>>::Output });
                } else {
                    let g = format_ident!("T{}", pascal_case(&r.prisma_name));
                    next_generics_with.push(quote! { #g });
                }
            }
            let next_generics_with_tokens = if next_generics_with.is_empty() { quote!{} } else { quote!{ <#(#next_generics_with),*> } };

            let mut next_generics_as = Vec::new();
            for r in &next_subset {
                if r.prisma_name == relation.prisma_name {
                    next_generics_as.push(quote! { U });
                } else {
                    let g = format_ident!("T{}", pascal_case(&r.prisma_name));
                    next_generics_as.push(quote! { #g });
                }
            }
            let next_generics_as_tokens = if next_generics_as.is_empty() { quote!{} } else { quote!{ <#(#next_generics_as),*> } };

            let mut impl_generics_with_list = vec![quote!{M}];
            for g in &current_generics_params { impl_generics_with_list.push(quote!{#g: ::prisma_core::serde::de::DeserializeOwned + Send + Sync}); }
            let impl_generics_with = quote! { <#(#impl_generics_with_list),*> };

            let mut impl_generics_as_list = vec![quote!{U: ::prisma_core::serde::de::DeserializeOwned + Send + Sync}];
            for g in &current_generics_params { impl_generics_as_list.push(quote!{#g: ::prisma_core::serde::de::DeserializeOwned + Send + Sync}); }
            let impl_generics_as = quote! { <#(#impl_generics_as_list),*> };

            transitions.push(quote! {
                impl #impl_generics_base #include_transition_trait<#pascal_name> for #combined_name #current_generics_decl { 
                    type Output = #next_combined_name #next_generics_base_tokens; 
                }
                impl #impl_generics_with #include_transition_trait<#pascal_name_with<M>> for #combined_name #current_generics_decl 
                where #related_model: #related_include_transition_trait<M>
                { 
                    type Output = #next_combined_name #next_generics_with_tokens; 
                }
                impl #impl_generics_as #include_transition_trait<#pascal_name_as<U>> for #combined_name #current_generics_decl { 
                    type Output = #next_combined_name #next_generics_as_tokens; 
                }
            });
        }
    }
    
    // Empty transition for Value
    transitions.push(quote! {
        impl #include_transition_trait<#empty_marker_name> for ::prisma_core::serde_json::Value { type Output = ::prisma_core::serde_json::Value; }
    });

    for relation in &relations {
        let pascal_name = format_ident!("{}Include{}", model_name, pascal_case(&relation.prisma_name));
        let pascal_name_with = format_ident!("{}Include{}With", model_name, pascal_case(&relation.prisma_name));
        let pascal_name_as = format_ident!("{}Include{}As", model_name, pascal_case(&relation.prisma_name));
        transitions.push(quote! {
            impl #include_transition_trait<#pascal_name> for ::prisma_core::serde_json::Value { type Output = ::prisma_core::serde_json::Value; }
            impl<M> #include_transition_trait<#pascal_name_with<M>> for ::prisma_core::serde_json::Value { type Output = ::prisma_core::serde_json::Value; }
            impl<U> #include_transition_trait<#pascal_name_as<U>> for ::prisma_core::serde_json::Value { type Output = ::prisma_core::serde_json::Value; }
        });
    }

    let mut wrapper_impls = Vec::new();

    let wrappers = [
        (&many_name, &where_name, quote! { Vec<T> }),
        (&unique_name, &unique_where_name, quote! { Option<T> }),
        (&first_name, &where_name, quote! { Option<T> }),
        (&unique_throw_name, &unique_where_name, quote! { T }),
        (&first_throw_name, &where_name, quote! { T }),
    ];

    for (w_name, w_where, r_type) in wrappers {
        let select_name = format_ident!("{}SelectBuilder", model_name);
        wrapper_impls.push(quote! {
            pub struct #w_name<T = #model_name> {
                pub inner: ::prisma_core::ReadBuilder<#r_type>,
                pub _phantom: std::marker::PhantomData<T>,
            }

            impl<T: ::prisma_core::serde::de::DeserializeOwned + Send + Sync> #w_name<T> {
                pub fn where_clause<F>(mut self, f: F) -> Self where F: FnOnce(&mut #w_where) {
                    let mut builder = #w_where::default();
                    f(&mut builder);
                    let map = builder.build();
                    if !map.is_empty() {
                        use ::prisma_core::Filterable;
                        self.inner.add_filter_arg("where".to_string(), ::prisma_core::query_core::ArgumentValue::Object(map));
                    }
                    self
                }

                pub fn select<F>(mut self, f: F) -> #w_name<::prisma_core::serde_json::Value> where F: FnOnce(&mut #select_name) {
                    let mut builder = #select_name::default();
                    f(&mut builder);
                    let selections: Vec<::prisma_core::query_core::Selection> = builder.into();
                    use ::prisma_core::Selectable;
                    self.inner.state.selection.clear_nested_selections();
                    for sel in selections { self.inner.add_nested_selection(sel); }
                    #w_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
                }

                pub fn select_as<U: ::prisma_core::serde::de::DeserializeOwned + Send + Sync, F>(
                    mut self,
                    selection: (std::marker::PhantomData<U>, F)
                ) -> #w_name<U>
                where F: FnOnce(&mut #select_name) {
                    let mut builder = #select_name::default();
                    (selection.1)(&mut builder);
                    let selections: Vec<::prisma_core::query_core::Selection> = builder.into();
                    use ::prisma_core::Selectable;
                    self.inner.state.selection.clear_nested_selections();
                    for sel in selections { self.inner.add_nested_selection(sel); }
                    #w_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
                }

                pub fn include<M, F>(mut self, f: F) -> #w_name<<T as #include_transition_trait<M>>::Output>
                where
                    F: FnOnce(&mut #include_name) -> M,
                    M: #include_marker_trait,
                    T: #include_transition_trait<M>
                {
                    let mut builder = #include_name::default();
                    let marker = f(&mut builder);
                    use ::prisma_core::Selectable;

                    if self.inner.state.selection.nested_selections().is_empty() {
                        for scalar_field in &[#(#scalar_field_names),*] {
                            self.inner.add_nested_selection(::prisma_core::query_core::Selection::with_name(scalar_field.to_string()));
                        }
                    }

                    if let Some(nested) = marker.into_selection() {
                        self.inner.add_nested_selection(nested);
                    }
                    
                    for (k, v) in std::mem::take(&mut builder.args) {
                        self.inner.add_filter_arg(k, v);
                    }
                    #w_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
                }

                pub fn r#as<U: ::prisma_core::serde::de::DeserializeOwned + Send + Sync>(self) -> #w_name<U> {
                    #w_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
                }

                pub async fn exec(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<#r_type> {
                    self.inner.exec_inferred(client).await
                }
            }
        });
    }

    let scalar_selections = model_metadata.scalar_field_names.iter().map(|f| {
        quote! { ::prisma_core::query_core::Selection::with_name(#f.to_string()) }
    });

    quote! {
        pub trait #include_marker_trait {
            fn into_selection(self) -> Option<::prisma_core::query_core::Selection>;
        }

        pub trait #include_transition_trait<M> {
            type Output: ::prisma_core::serde::de::DeserializeOwned + Send + Sync;
        }

        #(#include_markers)*
        #(#transitions)*

        #[derive(Default)]
        pub struct #include_name {
            pub args: ::prisma_core::IndexMap<String, ::prisma_core::query_core::ArgumentValue>,
        }

        impl #include_name {
            pub fn scalar_selections() -> Vec<::prisma_core::query_core::Selection> {
                vec![
                    #(#scalar_selections),*
                ]
            }

            pub fn where_clause<F>(&mut self, f: F) -> &mut Self 
            where F: FnOnce(&mut #where_name) 
            {
                let mut builder = #where_name::default();
                f(&mut builder);
                let map = builder.build();
                if !map.is_empty() {
                    self.args.insert("where".to_string(), ::prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn take(&mut self, take: i64) -> &mut Self {
                self.args.insert("take".to_string(), ::prisma_core::query_core::ArgumentValue::Scalar(::prisma_core::query_structure::PrismaValue::Int(take)));
                self
            }

            pub fn skip(&mut self, skip: i64) -> &mut Self {
                self.args.insert("skip".to_string(), ::prisma_core::query_core::ArgumentValue::Scalar(::prisma_core::query_structure::PrismaValue::Int(skip)));
                self
            }

            pub fn empty(&mut self) -> #empty_marker_name {
                #empty_marker_name
            }

            #(#include_methods)*
        }

        impl #include_marker_trait for #include_name {
            fn into_selection(self) -> Option<::prisma_core::query_core::Selection> {
                None
            }
        }

        #(#wrapper_impls)*
    }
}

pub fn generate_write_wrapper(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let write_wrapper_name = format_ident!("{}WriteBuilder", model_name);
    let unique_where_builder_name = format_ident!("{}UniqueWhereBuilder", model_name);
    let select_builder_name = format_ident!("{}SelectBuilder", model_name);
    let data_builder_name = format_ident!("{}DataBuilder", model_name);
    let include_builder_name = format_ident!("{}IncludeBuilder", model_name);
    let scalar_field_names = &model_metadata.scalar_field_names;
    let include_marker_trait = format_ident!("{}IncludeMarker", model_name);
    let include_transition_trait = format_ident!("{}IncludeTransition", model_name);

    quote! {
        /// Builder for write operations (returns T)
        pub struct #write_wrapper_name<T = #model_name> {
            pub inner: ::prisma_core::WriteBuilder<T>,
            pub _phantom: std::marker::PhantomData<T>,
        }

        impl<T: ::prisma_core::serde::de::DeserializeOwned + Send + Sync> #write_wrapper_name<T> {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #unique_where_builder_name)
            {
                let mut builder = #unique_where_builder_name::default();
                f(&mut builder);
                let map = builder.build();
                if !map.is_empty() {
                    use ::prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), ::prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn data<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #data_builder_name)
            {
                let mut builder = #data_builder_name::default();
                f(&mut builder);
                
                // Find existing data arg and merge if it exists
                let mut merged_data = std::mem::take(&mut builder.data);
                if let Some(::prisma_core::query_core::ArgumentValue::Object(existing_map)) = self.inner.state.arguments.get("data") {
                    let mut new_map = existing_map.clone();
                    for (k, v) in merged_data {
                        new_map.insert(k, v);
                    }
                    merged_data = new_map;
                }

                use ::prisma_core::Filterable;
                self.inner.add_filter_arg("data".to_string(), ::prisma_core::query_core::ArgumentValue::Object(merged_data));
                self
            }

            pub fn select<F>(mut self, f: F) -> #write_wrapper_name<::prisma_core::serde_json::Value>
            where F: FnOnce(&mut #select_builder_name)
            {
                let mut builder = #select_builder_name::default();
                f(&mut builder);
                let selections: Vec<::prisma_core::query_core::Selection> = builder.into();
                use ::prisma_core::Selectable;
                self.inner.state.selection.clear_nested_selections();
                for sel in selections {
                    self.inner.add_nested_selection(sel);
                }
                #write_wrapper_name {
                    inner: self.inner.with_type(),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn select_as<U: ::prisma_core::serde::de::DeserializeOwned + Send + Sync, F>(
                mut self,
                selection: (std::marker::PhantomData<U>, F)
            ) -> #write_wrapper_name<U>
            where F: FnOnce(&mut #select_builder_name) {
                let mut builder = #select_builder_name::default();
                (selection.1)(&mut builder);
                let selections: Vec<::prisma_core::query_core::Selection> = builder.into();
                use ::prisma_core::Selectable;
                self.inner.state.selection.clear_nested_selections();
                for sel in selections { self.inner.add_nested_selection(sel); }
                #write_wrapper_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            pub fn include<M, F>(mut self, f: F) -> #write_wrapper_name<<T as #include_transition_trait<M>>::Output>
            where
                F: FnOnce(&mut #include_builder_name) -> M,
                M: #include_marker_trait,
                T: #include_transition_trait<M>
            {
                let mut builder = #include_builder_name::default();
                let marker = f(&mut builder);
                use ::prisma_core::Selectable;

                if self.inner.state.selection.nested_selections().is_empty() {
                    for scalar_field in &[#(#scalar_field_names),*] {
                        self.inner.add_nested_selection(::prisma_core::query_core::Selection::with_name(scalar_field.to_string()));
                    }
                }

                if let Some(nested) = marker.into_selection() {
                    self.inner.add_nested_selection(nested);
                }
                for (k, v) in std::mem::take(&mut builder.args) {
                    self.inner.add_filter_arg(k, v);
                }
                #write_wrapper_name { inner: self.inner.with_type(), _phantom: std::marker::PhantomData }
            }

            pub fn r#as<U: ::prisma_core::serde::de::DeserializeOwned + Send + Sync>(self) -> #write_wrapper_name<U> {
                #write_wrapper_name {
                    inner: self.inner.with_type(),
                    _phantom: std::marker::PhantomData,
                }
            }

            pub async fn exec(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<T> {
                self.inner.exec_inferred(client).await
            }
        }
    }
}

pub fn generate_upsert_wrapper(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let wrapper_name = format_ident!("{}UpsertBuilder", model_name);
    let where_unique_builder_name = format_ident!("{}UniqueWhereBuilder", model_name);
    let data_builder_name = format_ident!("{}DataBuilder", model_name);
    let create_params = model_metadata.create_params();
    let create_data_inserts = model_metadata.create_data_inserts("create_builder.data");

    quote! {
        pub struct #wrapper_name {
            pub inner: ::prisma_core::WriteBuilder<#model_name>,
        }

        impl #wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_unique_builder_name)
            {
                let mut builder = #where_unique_builder_name::default();
                f(&mut builder);
                let map = builder.build();
                if !map.is_empty() {
                    use ::prisma_core::FilterBuilder;
                    self.inner.add_filter_arg("where".to_string(), ::prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn update<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #data_builder_name)
            {
                let mut builder = #data_builder_name::default();
                f(&mut builder);
                self.inner.add_filter_arg("update".to_string(), ::prisma_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)));
                self
            }

            pub fn create<F>(mut self, #create_params, f: F) -> Self
            where F: FnOnce(&mut #data_builder_name)
            {
                let mut create_builder = #data_builder_name::default();
                #create_data_inserts
                f(&mut create_builder);
                self.inner.add_filter_arg("create".to_string(), ::prisma_core::query_core::ArgumentValue::Object(std::mem::take(&mut create_builder.data)));
                self
            }

            pub async fn exec(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<#model_name> {
                self.inner.exec_inferred(client).await
            }
        }
    }
}

pub fn generate_create_many_wrapper(model_name: &syn::Ident, _model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let wrapper_name = format_ident!("{}CreateManyBuilder", model_name);
    let scalar_data_builder_name = format_ident!("{}ScalarDataBuilder", model_name);

    quote! {
        pub struct #wrapper_name {
            pub inner: ::prisma_core::CreateManyBuilder,
        }

        impl #wrapper_name {
            pub fn data<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #scalar_data_builder_name)
            {
                let mut builder = #scalar_data_builder_name::default();
                f(&mut builder);
                
                let mut list = match self.inner.state.arguments.get("data") {
                    Some(::prisma_core::query_core::ArgumentValue::List(l)) => l.clone(),
                    _ => Vec::new(),
                };
                list.push(::prisma_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)));

                use ::prisma_core::Filterable;
                self.inner.add_filter_arg("data".to_string(), ::prisma_core::query_core::ArgumentValue::List(list));
                self
            }

            pub fn skip_duplicates(mut self, skip: bool) -> Self {
                self.inner = self.inner.skip_duplicates(skip);
                self
            }

            pub async fn exec(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<i64> {
                self.inner.exec(client).await
            }
        }
    }
}

pub fn generate_create_many_and_return_wrapper(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let wrapper_name = format_ident!("{}CreateManyAndReturnBuilder", model_name);
    let scalar_data_builder_name = format_ident!("{}ScalarDataBuilder", model_name);
    let select_builder_name = format_ident!("{}SelectBuilder", model_name);
    let scalar_field_names = &model_metadata.scalar_field_names;

    quote! {
        pub struct #wrapper_name {
            pub inner: ::prisma_core::CreateManyAndReturnBuilder<#model_name>,
        }

        impl #wrapper_name {
            pub fn data<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #scalar_data_builder_name)
            {
                let mut builder = #scalar_data_builder_name::default();
                f(&mut builder);
                
                let mut list = match self.inner.state.arguments.get("data") {
                    Some(::prisma_core::query_core::ArgumentValue::List(l)) => l.clone(),
                    _ => Vec::new(),
                };
                list.push(::prisma_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)));

                use ::prisma_core::Filterable;
                self.inner.add_filter_arg("data".to_string(), ::prisma_core::query_core::ArgumentValue::List(list));
                self
            }

            pub fn skip_duplicates(mut self, skip: bool) -> Self {
                self.inner = self.inner.skip_duplicates(skip);
                self
            }

            pub fn select<F>(mut self, f: F) -> #wrapper_name where F: FnOnce(&mut #select_builder_name) {
                let mut builder = #select_builder_name::default();
                f(&mut builder);
                let selections: Vec<::prisma_core::query_core::Selection> = builder.into();
                use ::prisma_core::Selectable;
                self.inner.state.selection.clear_nested_selections();
                for sel in selections { self.inner.add_nested_selection(sel); }
                self
            }

            pub async fn exec(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<Vec<#model_name>> {
                let mut builder = self;
                if builder.inner.state.selection.nested_selections().is_empty() {
                    for field in &[#(#scalar_field_names),*] {
                        use ::prisma_core::Selectable;
                        builder.inner.add_nested_selection(::prisma_core::query_core::Selection::with_name(field.to_string()));
                    }
                }
                builder.inner.exec(client).await
            }
        }
    }
}

pub fn generate_update_many_wrapper(model_name: &syn::Ident, _model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let wrapper_name = format_ident!("{}UpdateManyBuilder", model_name);
    let where_builder_name = format_ident!("{}WhereBuilder", model_name);
    let scalar_data_builder_name = format_ident!("{}ScalarDataBuilder", model_name);

    quote! {
        pub struct #wrapper_name {
            pub inner: ::prisma_core::UpdateManyBuilder,
        }

        impl #wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_builder_name)
            {
                let mut builder = #where_builder_name::default();
                f(&mut builder);
                let map = builder.build();
                if !map.is_empty() {
                    use ::prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), ::prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn data<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #scalar_data_builder_name)
            {
                let mut builder = #scalar_data_builder_name::default();
                f(&mut builder);
                use ::prisma_core::Filterable;
                self.inner.add_filter_arg("data".to_string(), ::prisma_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)));
                self
            }

            pub async fn exec(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<i64> {
                self.inner.exec(client).await
            }
        }
    }
}

pub fn generate_update_many_and_return_wrapper(model_name: &syn::Ident, model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let wrapper_name = format_ident!("{}UpdateManyAndReturnBuilder", model_name);
    let where_builder_name = format_ident!("{}WhereBuilder", model_name);
    let scalar_data_builder_name = format_ident!("{}ScalarDataBuilder", model_name);
    let select_builder_name = format_ident!("{}SelectBuilder", model_name);
    let scalar_field_names = &model_metadata.scalar_field_names;

    quote! {
        pub struct #wrapper_name {
            pub inner: ::prisma_core::UpdateManyAndReturnBuilder<#model_name>,
        }

        impl #wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_builder_name)
            {
                let mut builder = #where_builder_name::default();
                f(&mut builder);
                let map = builder.build();
                if !map.is_empty() {
                    use ::prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), ::prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub fn data<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #scalar_data_builder_name)
            {
                let mut builder = #scalar_data_builder_name::default();
                f(&mut builder);
                use ::prisma_core::Filterable;
                self.inner.add_filter_arg("data".to_string(), ::prisma_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)));
                self
            }

            pub fn select<F>(mut self, f: F) -> #wrapper_name where F: FnOnce(&mut #select_builder_name) {
                let mut builder = #select_builder_name::default();
                f(&mut builder);
                let selections: Vec<::prisma_core::query_core::Selection> = builder.into();
                use ::prisma_core::Selectable;
                self.inner.state.selection.clear_nested_selections();
                for sel in selections { self.inner.add_nested_selection(sel); }
                self
            }

            pub async fn exec(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<Vec<#model_name>> {
                let mut builder = self;
                if builder.inner.state.selection.nested_selections().is_empty() {
                    for field in &[#(#scalar_field_names),*] {
                        use ::prisma_core::Selectable;
                        builder.inner.add_nested_selection(::prisma_core::query_core::Selection::with_name(field.to_string()));
                    }
                }
                builder.inner.exec(client).await
            }
        }
    }
}

pub fn generate_delete_many_wrapper(model_name: &syn::Ident, _model_metadata: &ModelMetadata) -> proc_macro2::TokenStream {
    let wrapper_name = format_ident!("{}DeleteManyBuilder", model_name);
    let where_builder_name = format_ident!("{}WhereBuilder", model_name);

    quote! {
        pub struct #wrapper_name {
            pub inner: ::prisma_core::DeleteManyBuilder,
        }

        impl #wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_builder_name)
            {
                let mut builder = #where_builder_name::default();
                f(&mut builder);
                let map = builder.build();
                if !map.is_empty() {
                    use ::prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), ::prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub async fn exec(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<i64> {
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
            pub inner: ::prisma_core::CountBuilder,
        }

        impl #count_wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_builder_name)
            {
                let mut builder = #where_builder_name::default();
                f(&mut builder);
                let map = builder.build();
                if !map.is_empty() {
                    use ::prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), ::prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub async fn exec(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<i64> {
                use ::prisma_core::Executable;
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
            pub inner: ::prisma_core::AggregateBuilder,
        }

        impl #aggregate_wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_builder_name)
            {
                let mut builder = #where_builder_name::default();
                f(&mut builder);
                let map = builder.build();
                if !map.is_empty() {
                    use ::prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), ::prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub async fn exec<T: ::prisma_core::serde::de::DeserializeOwned>(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<T> {
                use ::prisma_core::Executable;
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
            pub inner: ::prisma_core::GroupByBuilder,
        }

        impl #group_by_wrapper_name {
            pub fn where_clause<F>(mut self, f: F) -> Self
            where F: FnOnce(&mut #where_builder_name)
            {
                let mut builder = #where_builder_name::default();
                f(&mut builder);
                let map = builder.build();
                if !map.is_empty() {
                    use ::prisma_core::Filterable;
                    self.inner.add_filter_arg("where".to_string(), ::prisma_core::query_core::ArgumentValue::Object(map));
                }
                self
            }

            pub async fn exec<T: ::prisma_core::serde::de::DeserializeOwned>(self, client: &::prisma_core::client::PrismaClient) -> ::prisma_core::Result<T> {
                use ::prisma_core::Executable;
                self.inner.exec(client).await
            }
        }
    }
}
