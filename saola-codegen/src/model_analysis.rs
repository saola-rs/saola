//! Model field analysis and metadata extraction

use crate::utils::{get_filter_type, get_inner_type};
use heck::ToUpperCamelCase;
use quote::{format_ident, quote};
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
    pub is_updated_at: bool,
    pub opposite_relation_field: Option<String>,
    pub field_type: Type,
    pub enum_name: Option<String>,
}

/// Metadata about a model
#[derive(Clone)]
pub struct ModelMetadata {
    pub name: String,
    pub fields: Vec<FieldMetadata>,
    pub scalar_field_names: Vec<String>,
}

impl ModelMetadata {
    pub fn new(name: String, fields: Vec<FieldMetadata>) -> Self {
        let scalar_field_names = fields
            .iter()
            .filter(|f| !f.is_relation)
            .map(|f| f.prisma_name.clone())
            .collect();
        Self {
            name,
            fields,
            scalar_field_names,
        }
    }

    /// Generate arguments for create methods
    pub fn create_params(&self) -> Vec<proc_macro2::TokenStream> {
        self.create_params_with_ignore(None)
    }

    pub fn create_params_with_ignore(&self, ignore_field: Option<&str>) -> Vec<proc_macro2::TokenStream> {
        self.fields
            .iter()
            .filter(|f| {
                if let Some(ignore) = ignore_field {
                    if f.prisma_name == ignore {
                        return false;
                    }
                }
                // Filter for required fields that don't have a default value
                !f.is_optional && !f.has_default && !f.is_updated_at && !f.is_relation_link
            })
            .map(|f| {
                let name_raw = &f.rust_name;
                let name = if matches!(
                    name_raw.as_str(),
                    "type" | "use" | "mod" | "crate" | "self" | "super" | "trait" | "impl" | "for" | "match" | "where"
                ) {
                    format_ident!("r#{}", name_raw)
                } else {
                    format_ident!("{}", name_raw)
                };

                if f.is_relation {
                    let rel_builder =
                        format_ident!("{}{}RelationWriteBuilder", self.name, f.rust_name.to_upper_camel_case());
                    quote! { #name: impl FnOnce(&mut #rel_builder) }
                } else {
                    let ty = &f.field_type;
                    quote! { #name: #ty }
                }
            })
            .collect()
    }

    /// Generate data inserts for create methods
    pub fn create_data_inserts(&self, map_expr: &str) -> proc_macro2::TokenStream {
        self.create_data_inserts_with_ignore(map_expr, None)
    }

    pub fn create_data_inserts_with_ignore(
        &self,
        map_expr_str: &str,
        ignore_field: Option<&str>,
    ) -> proc_macro2::TokenStream {
        let map_expr = syn::parse_str::<syn::Expr>(map_expr_str).unwrap();
        let inserts = self.fields
            .iter()
            .filter(|f| {
                if let Some(ignore) = ignore_field {
                    if f.prisma_name == ignore { return false; }
                }
                !f.is_optional && !f.has_default && !f.is_updated_at && !f.is_relation_link
            })
            .map(|field| {
                let rust_name_raw = &field.rust_name;
                let rust_name = if matches!(rust_name_raw.as_str(), "type" | "use" | "mod" | "crate" | "self" | "super" | "trait" | "impl" | "for" | "match" | "where") {
                    format_ident!("r#{}", rust_name_raw)
                } else {
                    format_ident!("{}", rust_name_raw)
                };
                let prisma_name = &field.prisma_name;
                
                if field.is_relation {
                    let rel_builder = format_ident!("{}{}RelationWriteBuilder", self.name, field.rust_name.to_upper_camel_case());
                    quote! {
                        {
                            let mut rel_builder = #rel_builder::default();
                            #rust_name(&mut rel_builder);
                            #map_expr.insert(#prisma_name.to_string(), ::saola_core::query_core::ArgumentValue::Object(rel_builder.data));
                        }
                    }
                } else {
                    quote! {
                        #map_expr.insert(#prisma_name.to_string(), ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(#rust_name)));
                    }
                }
            });

        quote! { #(#inserts)* }
    }
}

/// Generate filter methods for WhereBuilder
pub fn generate_filter_methods(fields: &[FieldMetadata]) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            let rust_name_raw = &field.rust_name;
            let rust_field_name = if matches!(rust_name_raw.as_str(), "type" | "use" | "mod" | "crate" | "self" | "super" | "trait" | "impl" | "for" | "match" | "where") {
                format_ident!("r#{}", rust_name_raw)
            } else {
                format_ident!("{}", rust_name_raw)
            };
            let prisma_name = &field.prisma_name;

            if field.is_relation {
                let inner_type_str = get_inner_type(&field.field_type);
                let related_where_builder = format_ident!("{}WhereBuilder", inner_type_str);
                
                Some(quote! {
                    pub fn #rust_field_name(&mut self) -> ::saola_core::RelationFilter<'_, Self, #related_where_builder> {
                        ::saola_core::RelationFilter {
                            builder: self,
                            field_name: #prisma_name,
                            _phantom: std::marker::PhantomData,
                        }
                    }
                })
            } else if let Some((filter_builder_name, _trait_name)) = get_filter_type(&field.field_type) {
                let filter_builder_ident = format_ident!("{}", filter_builder_name);
                let enum_extra_ident = field.enum_name.as_ref().map(|name| {
                    let ident = format_ident!("{}", name);

                    quote! {
                        , enums::#ident
                    }
                });

                Some(quote! {
                    pub fn #rust_field_name(&mut self) -> ::saola_core::#filter_builder_ident<'_, Self #enum_extra_ident> {
                        ::saola_core::#filter_builder_ident {
                            builder: self,
                            field_name: #prisma_name,
                            _phantom: std::marker::PhantomData,
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
            let rust_name_raw = &field.rust_name;
            let rust_name = if matches!(rust_name_raw.as_str(), "type" | "use" | "mod" | "crate" | "self" | "super" | "trait" | "impl" | "for" | "match" | "where") {
                format_ident!("r#{}", rust_name_raw)
            } else {
                format_ident!("{}", rust_name_raw)
            };
            let prisma_name = &field.prisma_name;

            quote! {
                pub fn #rust_name<T>(&mut self, value: T) -> &mut Self
                where T: Into<::saola_core::query_structure::PrismaValue>
                {
                    use ::saola_core::FilterBuilder;
                    self.add_arg(#prisma_name.to_string(), ::saola_core::query_core::ArgumentValue::Scalar(value.into()));
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
            let rust_name_raw = &field.rust_name;
            let rust_name = if matches!(
                rust_name_raw.as_str(),
                "type" | "use" | "mod" | "crate" | "self" | "super" | "trait" | "impl" | "for" | "match" | "where"
            ) {
                format_ident!("r#{}", rust_name_raw)
            } else {
                format_ident!("{}", rust_name_raw)
            };
            let prisma_name = &field.prisma_name;
            let field_type = &field.field_type;

            if field.is_relation {
                let inner_type_str = get_inner_type(&field.field_type);
                let related_select_builder = format_ident!("{}SelectBuilder", inner_type_str);

                let marker_type = if field.is_list {
                    quote! { Vec<()> }
                } else {
                    quote! { Option<()> }
                };

                let validate_ident = quote::format_ident!("_validate_field_{}", prisma_name);

                quote! {
                    #[allow(non_snake_case)]
                    pub fn #validate_ident(&self) {}

                    pub fn #rust_name<F>(&mut self, f: F) -> ::saola_core::SelectionRelField<'_, #marker_type, Self>
                    where F: FnOnce(&mut #related_select_builder)
                    {
                        let mut builder = #related_select_builder::default();
                        f(&mut builder);
                        let selections: Vec<::saola_core::query_core::Selection> = builder.into_selections();
                        let mut sel = ::saola_core::query_core::Selection::with_name(#prisma_name.to_string());
                        for s in selections {
                            sel.push_nested_selection(s);
                        }
                        self.selections.push(sel);
                        ::saola_core::SelectionRelField::new(self)
                    }
                }
            } else {
                let validate_ident = quote::format_ident!("_validate_field_{}", prisma_name);
                quote! {
                    #[allow(non_snake_case)]
                    pub fn #validate_ident(&self) {}

                    pub fn #rust_name(&mut self) -> ::saola_core::SelectionField<'_, #field_type, Self> {
                        self.selections.push(::saola_core::query_core::Selection::with_name(#prisma_name.to_string()));
                        ::saola_core::SelectionField::new(self)
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
            let rust_name_raw = &field.rust_name;
            let rust_name = if matches!(rust_name_raw.as_str(), "type" | "use" | "mod" | "crate" | "self" | "super" | "trait" | "impl" | "for" | "match" | "where") {
                format_ident!("r#{}", rust_name_raw)
            } else {
                format_ident!("{}", rust_name_raw)
            };
            let prisma_name = &field.prisma_name;

            if field.is_relation {
                if scalar_only {
                    return None;
                }
                let rel_write_builder = format_ident!("{}{}RelationWriteBuilder", model_name, field.rust_name.to_upper_camel_case());
                
                Some(quote! {
                    pub fn #rust_name<F>(&mut self, f: F) -> &mut Self
                    where F: FnOnce(&mut #rel_write_builder)
                    {
                        let mut builder = #rel_write_builder::default();
                        f(&mut builder);
                        if !builder.data.is_empty() {
                            self.data.insert(#prisma_name.to_string(), ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)));
                        }
                        self
                    }
                })
            } else {
                // Scalar fields
                let mut methods = vec![
                    quote! {
                        pub fn #rust_name<T>(&mut self, value: T) -> &mut Self
                        where T: Into<::saola_core::query_structure::PrismaValue>
                        {
                            self.data.insert(#prisma_name.to_string(), ::saola_core::query_core::ArgumentValue::Scalar(value.into()));
                            self
                        }
                    }
                ];

                let type_name = crate::utils::get_inner_type(&field.field_type);
                let is_numeric = match type_name.as_str() {
                    "i32" | "i64" | "f32" | "f64" | "BigDecimal" => true,
                    _ => false,
                };

                if is_numeric {
                    let inc_name = quote::format_ident!("{}_increment", field.rust_name);
                    let dec_name = quote::format_ident!("{}_decrement", field.rust_name);
                    let mul_name = quote::format_ident!("{}_multiply", field.rust_name);
                    let div_name = quote::format_ident!("{}_divide", field.rust_name);

                    methods.push(quote! {
                        pub fn #inc_name<T>(&mut self, value: T) -> &mut Self
                        where T: Into<::saola_core::query_structure::PrismaValue>
                        {
                            let mut map = ::saola_core::IndexMap::new();
                            map.insert("increment".to_string(), ::saola_core::query_core::ArgumentValue::Scalar(value.into()));
                            self.data.insert(#prisma_name.to_string(), ::saola_core::query_core::ArgumentValue::Object(map));
                            self
                        }
                        pub fn #dec_name<T>(&mut self, value: T) -> &mut Self
                        where T: Into<::saola_core::query_structure::PrismaValue>
                        {
                            let mut map = ::saola_core::IndexMap::new();
                            map.insert("decrement".to_string(), ::saola_core::query_core::ArgumentValue::Scalar(value.into()));
                            self.data.insert(#prisma_name.to_string(), ::saola_core::query_core::ArgumentValue::Object(map));
                            self
                        }
                        pub fn #mul_name<T>(&mut self, value: T) -> &mut Self
                        where T: Into<::saola_core::query_structure::PrismaValue>
                        {
                            let mut map = ::saola_core::IndexMap::new();
                            map.insert("multiply".to_string(), ::saola_core::query_core::ArgumentValue::Scalar(value.into()));
                            self.data.insert(#prisma_name.to_string(), ::saola_core::query_core::ArgumentValue::Object(map));
                            self
                        }
                        pub fn #div_name<T>(&mut self, value: T) -> &mut Self
                        where T: Into<::saola_core::query_structure::PrismaValue>
                        {
                            let mut map = ::saola_core::IndexMap::new();
                            map.insert("divide".to_string(), ::saola_core::query_core::ArgumentValue::Scalar(value.into()));
                            self.data.insert(#prisma_name.to_string(), ::saola_core::query_core::ArgumentValue::Object(map));
                            self
                        }
                    });
                }

                Some(quote! { #(#methods)* })
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
            let rust_name_raw = &field.rust_name;
            let rust_name = if matches!(
                rust_name_raw.as_str(),
                "type" | "use" | "mod" | "crate" | "self" | "super" | "trait" | "impl" | "for" | "match" | "where"
            ) {
                format_ident!("r#{}", rust_name_raw)
            } else {
                format_ident!("{}", rust_name_raw)
            };
            let prisma_name = &field.prisma_name;

            quote! {
                pub fn #rust_name(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
                    let mut map = ::saola_core::IndexMap::new();
                    map.insert(#prisma_name.to_string(), ::saola_core::ArgumentValue::from(order));
                    self.args.push(::saola_core::ArgumentValue::Object(map));
                    self
                }
            }
        })
        .collect()
}
