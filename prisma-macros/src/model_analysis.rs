//! Model field analysis and metadata extraction

use crate::utils::{get_filter_type, get_inner_type};
use quote::quote;
use syn::Type;

/// Metadata about a field in the model
#[derive(Clone)]
pub struct FieldMetadata {
    pub rust_name: String,
    pub prisma_name: String,
    pub is_relation: bool,
    pub is_unique: bool,
    pub is_id: bool,
    pub is_optional: bool,
    pub field_type: Type,
}

/// Metadata about the model being processed
pub struct ModelMetadata {
    pub fields: Vec<FieldMetadata>,
    pub scalar_field_names: Vec<String>,
    pub create_required_fields: Vec<FieldMetadata>,
}

impl ModelMetadata {
    /// Create from parsed field metadata
    pub fn new(fields: Vec<FieldMetadata>) -> Self {
        let scalar_field_names: Vec<String> = fields
            .iter()
            .filter(|f| !f.is_relation)
            .map(|f| f.prisma_name.clone())
            .collect();

        let create_required_fields: Vec<FieldMetadata> = fields
            .iter()
            .filter(|f| !f.is_relation && !f.is_optional && !f.is_id)
            .cloned()
            .collect();

        Self {
            fields,
            scalar_field_names,
            create_required_fields,
        }
    }

    pub fn create_params(&self) -> proc_macro2::TokenStream {
        let params = self
            .create_required_fields
            .iter()
            .map(|field| {
                let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
                let ty = &field.field_type;
                quote! { #rust_name: #ty }
            })
            .collect::<Vec<_>>();

        if params.is_empty() {
            quote! {}
        } else {
            quote! { #(#params),* }
        }
    }

    /// Generate data map insertions for create function
    pub fn create_data_inserts(&self) -> proc_macro2::TokenStream {
        self.create_required_fields
            .iter()
            .map(|field| {
                let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
                let prisma_name = &field.prisma_name;

                quote! {
                    data_map.insert(#prisma_name.to_string(), prisma_core::query_core::ArgumentValue::Scalar(prisma_core::query_structure::PrismaValue::from(#rust_name)));
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .collect()
    }
}

/// Generate type-safe filter methods for a model
pub fn generate_scalar_filter_methods(fields: &[FieldMetadata]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            if field.is_relation {
                return None;
            }

            if let Some((filter_builder_name, _trait_name)) = get_filter_type(&field.field_type) {
                let rust_field_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
                let filter_builder_ident = syn::Ident::new(filter_builder_name, proc_macro2::Span::call_site());
                let prisma_name = &field.prisma_name;

                Some(quote! {
                    pub fn #rust_field_name(&mut self) -> prisma_core::#filter_builder_ident<'_, Self> {
                        prisma_core::#filter_builder_ident {
                            builder: self,
                            field_name: #prisma_name,
                        }
                    }
                })
            } else {
                None
            }
        })
        .collect()
}

/// Generate equality filter methods for unique fields
pub fn generate_unique_filter_methods(fields: &[FieldMetadata]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            if !field.is_unique {
                return None;
            }

            let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
            let prisma_name = &field.prisma_name;

            Some(quote! {
                pub fn #rust_name<T>(&mut self, value: T) -> &mut Self
                where T: Into<prisma_core::query_structure::PrismaValue>
                {
                    use prisma_core::FilterBuilder;
                    self.add_arg(#prisma_name.to_string(), prisma_core::query_core::ArgumentValue::Scalar(value.into()));
                    self
                }
            })
        })
        .collect()
}

/// Generate select methods for scalar and relation fields
pub fn generate_select_methods(fields: &[FieldMetadata]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|field| {
            let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
            let prisma_name = &field.prisma_name;

            if field.is_relation {
                let inner_type_str = get_inner_type(&field.field_type);
                let related_select_builder = syn::Ident::new(
                    &format!("{}SelectBuilder", inner_type_str),
                    proc_macro2::Span::call_site(),
                );

                quote! {
                    pub fn #rust_name<F>(&mut self, f: F) -> &mut Self
                    where F: FnOnce(&mut #related_select_builder)
                    {
                        let mut builder = #related_select_builder::default();
                        f(&mut builder);
                        let mut sel = prisma_core::query_core::Selection::with_name(#prisma_name.to_string());
                        let nested: Vec<prisma_core::query_core::Selection> = builder.into();
                        for n in nested {
                            sel.push_nested_selection(n);
                        }
                        self.selections.push(sel);
                        self
                    }
                }
            } else {
                quote! {
                    pub fn #rust_name(&mut self) -> &mut Self {
                        self.selections.push(prisma_core::query_core::Selection::with_name(#prisma_name.to_string()));
                        self
                    }
                }
            }
        })
        .collect()
}

/// Generate data builder methods for scalar fields
pub fn generate_data_methods(fields: &[FieldMetadata]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
            let prisma_name = &field.prisma_name;

            if field.is_relation {
                // Relation fields get special handling
                let inner_type = get_inner_type(&field.field_type);
                let related_data_builder =
                    syn::Ident::new(&format!("{}DataBuilder", inner_type), proc_macro2::Span::call_site());

                Some(quote! {
                    pub fn #rust_name<F>(&mut self, f: F) -> &mut Self
                    where F: FnOnce(&mut #related_data_builder)
                    {
                        let mut builder = #related_data_builder::default();
                        f(&mut builder);

                        let mut create_map = prisma_core::IndexMap::new();
                        create_map.insert("create".to_string(), prisma_core::query_core::ArgumentValue::Object(builder.data));

                        self.data.insert(#prisma_name.to_string(), prisma_core::query_core::ArgumentValue::Object(create_map));
                        self
                    }
                })
            } else {
                // Scalar fields
                Some(quote! {
                    pub fn #rust_name<T>(&mut self, value: T) -> &mut Self
                    where T: Into<prisma_core::query_structure::PrismaValue>
                    {
                        self.data.insert(#prisma_name.to_string(), prisma_core::query_core::ArgumentValue::Scalar(value.into()));
                        self
                    }
                })
            }
        })
        .collect()
}

/// Generate include methods for relation fields
pub fn generate_include_methods(fields: &[FieldMetadata]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            if !field.is_relation {
                return None;
            }

            let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
            let with_suffix_name =
                syn::Ident::new(&format!("{}_with", field.rust_name), proc_macro2::Span::call_site());
            let prisma_name = &field.prisma_name;
            let inner_type_str = get_inner_type(&field.field_type);
            let related_select_builder = syn::Ident::new(
                &format!("{}SelectBuilder", inner_type_str),
                proc_macro2::Span::call_site(),
            );

            Some(quote! {
                // Include all fields of this relation
                pub fn #rust_name(&mut self) -> &mut Self {
                    let mut builder = #related_select_builder::default();
                    builder.all();
                    let mut sel = prisma_core::query_core::Selection::with_name(#prisma_name.to_string());
                    let fields: Vec<prisma_core::query_core::Selection> = builder.into();
                    for f in fields {
                        sel.push_nested_selection(f);
                    }
                    self.includes.push(sel);
                    self
                }

                // Include specific fields of this relation via closure
                pub fn #with_suffix_name<F>(&mut self, f: F) -> &mut Self
                where F: FnOnce(&mut #related_select_builder)
                {
                    let mut builder = #related_select_builder::default();
                    f(&mut builder);
                    let mut sel = prisma_core::query_core::Selection::with_name(#prisma_name.to_string());
                    let fields: Vec<prisma_core::query_core::Selection> = builder.into();
                    for f in fields {
                        sel.push_nested_selection(f);
                    }
                    self.includes.push(sel);
                    self
                }
            })
        })
        .collect()
}
