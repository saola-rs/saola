//! Model field analysis and metadata extraction

use crate::utils::{get_filter_type, get_inner_type};
use heck::ToUpperCamelCase;
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
    pub is_list: bool,
    pub is_relation_link: bool,
    pub has_default: bool,
    pub is_updated_at: bool, // New field
    pub opposite_relation_field: Option<String>,
    pub field_type: Type,
}

/// Metadata about the model being processed
pub struct ModelMetadata {
    pub name: String,
    pub fields: Vec<FieldMetadata>,
    pub scalar_field_names: Vec<String>,
    pub create_required_fields: Vec<FieldMetadata>,
}

impl ModelMetadata {
    /// Create from parsed field metadata
    pub fn new(name: String, fields: Vec<FieldMetadata>) -> Self {
        let scalar_field_names: Vec<String> = fields
            .iter()
            .filter(|f| !f.is_relation)
            .map(|f| f.prisma_name.clone())
            .collect();

        let create_required_fields: Vec<FieldMetadata> = fields
            .iter()
            .filter(|f| {
                !f.is_optional
                    && !f.is_relation_link
                    && !f.is_list
                    && !f.has_default
                    && !f.is_updated_at // Exclude updatedAt fields from required
            })
            .cloned()
            .collect();

        Self {
            name,
            fields,
            scalar_field_names,
            create_required_fields,
        }
    }

    /// Generate parameter list for create function, optionally ignoring one relation (for nested create)
    pub fn create_params_with_ignore(&self, ignore_relation: Option<&str>) -> Vec<proc_macro2::TokenStream> {
        self.create_required_fields
            .iter()
            .filter(|f| {
                if let Some(ignore) = ignore_relation {
                    f.prisma_name != ignore
                } else {
                    true
                }
            })
            .map(|field| {
                let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
                let field_type = &field.field_type;

                if field.is_relation {
                    // For relations, we take a closure
                    let rel_builder = quote::format_ident!(
                        "{}{}RelationWriteBuilder",
                        self.name,
                        field.rust_name.to_upper_camel_case()
                    );
                    quote! { #rust_name: impl FnOnce(&mut #rel_builder) }
                } else {
                    quote! { #rust_name: #field_type }
                }
            })
            .collect()
    }

    pub fn create_params(&self) -> Vec<proc_macro2::TokenStream> {
        self.create_params_with_ignore(None)
    }

    /// Generate data map insertions for create function
    pub fn create_data_inserts_with_ignore(
        &self,
        map_name: &str,
        ignore_relation: Option<&str>,
    ) -> proc_macro2::TokenStream {
        let map_expr: syn::Expr = syn::parse_str(map_name).expect("Failed to parse map name as expression");
        self.create_required_fields
            .iter()
            .filter(|f| {
                if let Some(ignore) = ignore_relation {
                    f.prisma_name != ignore
                } else {
                    true
                }
            })
            .map(|field| {
                let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
                let prisma_name = &field.prisma_name;

                if field.is_relation {
                    let rel_builder = quote::format_ident!("{}{}RelationWriteBuilder", self.name, field.rust_name.to_upper_camel_case());
                    quote! {
                        {
                            let mut rel_builder = #rel_builder::default();
                            #rust_name(&mut rel_builder);
                            #map_expr.insert(#prisma_name.to_string(), saola_core::query_core::ArgumentValue::Object(rel_builder.data));
                        }
                    }
                } else {
                    quote! {
                        #map_expr.insert(#prisma_name.to_string(), saola_core::query_core::ArgumentValue::Scalar(saola_core::query_structure::PrismaValue::from(#rust_name)));
                    }
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .collect()
    }

    pub fn create_data_inserts(&self, map_name: &str) -> proc_macro2::TokenStream {
        self.create_data_inserts_with_ignore(map_name, None)
    }
}

/// Generate type-safe filter methods for a model (scalars and relations)
pub fn generate_filter_methods(fields: &[FieldMetadata]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            let rust_field_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
            let prisma_name = &field.prisma_name;

            if field.is_relation {
                let inner_type_str = get_inner_type(&field.field_type);
                let related_where_builder = syn::Ident::new(
                    &format!("{}WhereBuilder", inner_type_str),
                    proc_macro2::Span::call_site(),
                );

                Some(quote! {
                    pub fn #rust_field_name(&mut self) -> saola_core::RelationFilter<'_, Self, #related_where_builder> {
                        saola_core::RelationFilter {
                            builder: self,
                            field_name: #prisma_name,
                            _phantom: std::marker::PhantomData,
                        }
                    }
                })
            } else if let Some((filter_builder_name, _trait_name)) = get_filter_type(&field.field_type) {
                let filter_builder_ident = syn::Ident::new(filter_builder_name, proc_macro2::Span::call_site());

                Some(quote! {
                    pub fn #rust_field_name(&mut self) -> saola_core::#filter_builder_ident<'_, Self> {
                        saola_core::#filter_builder_ident {
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

/// Generate unique filter methods for UniqueWhereBuilder
pub fn generate_unique_filter_methods(fields: &[FieldMetadata]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .filter(|f| f.is_unique || f.is_id)
        .map(|field| {
            let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
            let prisma_name = &field.prisma_name;
            let _ty = &field.field_type;

            quote! {
                pub fn #rust_name<T>(&mut self, value: T) -> &mut Self
                where T: Into<saola_core::query_structure::PrismaValue>
                {
                    use saola_core::FilterBuilder;
                    self.add_arg(#prisma_name.to_string(), saola_core::query_core::ArgumentValue::Scalar(value.into()));
                    self
                }
            }
        })
        .collect()
}

/// Generate selection builder methods
pub fn generate_select_methods(fields: &[FieldMetadata]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|field| {
            let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
            let prisma_name = &field.prisma_name;
            let field_type = &field.field_type;

            if field.is_relation {
                let inner_type_str = get_inner_type(&field.field_type);
                let related_select_builder = syn::Ident::new(
                    &format!("{}SelectBuilder", inner_type_str),
                    proc_macro2::Span::call_site(),
                );

                let marker_type = if field.is_list {
                    quote! { Vec<()> }
                } else {
                    quote! { Option<()> }
                };

                quote! {
                    pub fn #rust_name<F>(&mut self, f: F) -> saola_core::SelectionRelField<'_, #marker_type, Self>
                    where F: FnOnce(&mut #related_select_builder)
                    {
                        let mut builder = #related_select_builder::default();
                        f(&mut builder);
                        let selections: Vec<saola_core::query_core::Selection> = builder.into();
                        let mut sel = saola_core::query_core::Selection::with_name(#prisma_name.to_string());
                        for s in selections {
                            sel.push_nested_selection(s);
                        }
                        self.selections.push(sel);
                        saola_core::SelectionRelField::new(self)
                    }
                }
            } else {
                quote! {
                    pub fn #rust_name(&mut self) -> saola_core::SelectionField<'_, #field_type, Self> {
                        self.selections.push(saola_core::query_core::Selection::with_name(#prisma_name.to_string()));
                        saola_core::SelectionField::new(self)
                    }
                }
            }
        })
        .collect()
}

/// Generate data builder methods for scalar and relation fields
pub fn generate_data_methods(
    model_name: &syn::Ident,
    fields: &[FieldMetadata],
    scalar_only: bool,
) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
            let prisma_name = &field.prisma_name;

            if field.is_relation {
                if scalar_only {
                    return None;
                }
                let rel_write_builder = syn::Ident::new(
                    &format!("{}{}RelationWriteBuilder", model_name, field.rust_name.to_upper_camel_case()),
                    proc_macro2::Span::call_site(),
                );

                Some(quote! {
                    pub fn #rust_name<F>(&mut self, f: F) -> &mut Self
                    where F: FnOnce(&mut #rel_write_builder)
                    {
                        let mut builder = #rel_write_builder::default();
                        f(&mut builder);
                        if !builder.data.is_empty() {
                            // The RelationWriteBuilder already has the "create" or "connect" key!
                            // So we just need to insert its internal map as the value for this field.
                            self.data.insert(#prisma_name.to_string(), saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)));
                        }
                        self
                    }
                })
            } else {
                // Scalar fields
                Some(quote! {
                    pub fn #rust_name<T>(&mut self, value: T) -> &mut Self
                    where T: Into<saola_core::query_structure::PrismaValue>
                    {
                        self.data.insert(#prisma_name.to_string(), saola_core::query_core::ArgumentValue::Scalar(value.into()));
                        self
                    }
                })
            }
        })
        .collect()
}

/// Generate order by methods for OrderByBuilder
pub fn generate_order_by_methods(fields: &[FieldMetadata]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .filter(|f| !f.is_relation)
        .map(|field| {
            let rust_name = syn::Ident::new(&field.rust_name, proc_macro2::Span::call_site());
            let prisma_name = &field.prisma_name;

            quote! {
                pub fn #rust_name(&mut self, order: saola_core::SortOrder) -> &mut Self {
                    let mut map = saola_core::IndexMap::new();
                    map.insert(#prisma_name.to_string(), saola_core::ArgumentValue::from(order));
                    self.args.push(saola_core::ArgumentValue::Object(map));
                    self
                }
            }
        })
        .collect()
}
