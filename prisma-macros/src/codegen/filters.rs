use parser_database::walkers::ModelWalker;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Helper function to capitalize first letter
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Generate WhereBuilder for all models
pub fn generate_where_builders(db: &parser_database::ParserDatabase) -> TokenStream {
    let mut output = TokenStream::new();

    for model in db.walk_models() {
        output.extend(generate_where_builder(model.name(), model));
    }

    output
}

/// Generate WhereBuilder for a single model
fn generate_where_builder(model_name: &str, model: ModelWalker) -> TokenStream {
    let builder_name = format_ident!("{}WhereBuilder", model_name);

    // Generate a field filter struct for each scalar field
    let field_filters: Vec<_> = model
        .scalar_fields()
        .map(|field| {
            let field_name = field.name();
            let filter_name = format_ident!("{}{}Filter", model_name, capitalize_first(field_name));
            let scalar_type = field.scalar_type();

            // Check if this is an enum field
            let field_enum = field.field_type_as_enum();

            let filter_methods = if let Some(enum_def) = field_enum {
                // This is an enum field - generate enum-specific filter methods
                let enum_name = enum_def.name();
                let enum_ident = format_ident!("{}", enum_name);

                quote! {
                    #[inline]
                    pub fn eq(self, value: #enum_ident) -> &'a mut #builder_name {
                        self.builder.conditions.push(format!("{} = {}", self.field, value.as_str()));
                        self.builder
                    }

                    #[inline]
                    pub fn eq_str(self, value: &str) -> &'a mut #builder_name {
                        self.builder.conditions.push(format!("{} = {}", self.field, value));
                        self.builder
                    }
                }
            } else {
                // Regular scalar field - use scalar-type-aware filters
                get_scalar_filter_methods(&filter_name, &builder_name, scalar_type)
            };

            quote! {
                pub struct #filter_name<'a> {
                    builder: &'a mut #builder_name,
                    field: &'static str,
                }

                impl<'a> #filter_name<'a> {
                    #filter_methods
                }
            }
        })
        .collect();

    // Generate method on builder for each scalar field
    let field_methods: Vec<_> = model
        .scalar_fields()
        .map(|field| {
            let field_name = field.name();
            let field_ident = format_ident!("{}", field_name);
            let filter_name = format_ident!("{}{}Filter", model_name, capitalize_first(field_name));

            quote! {
                #[inline]
                pub fn #field_ident(&mut self) -> #filter_name<'_> {
                    #filter_name {
                        builder: self,
                        field: stringify!(#field_ident),
                    }
                }
            }
        })
        .collect();

    // Generate method for each relation field (nested filter)
    let relation_methods: Vec<_> = model
        .relation_fields()
        .map(|field| {
            let field_name = field.name();
            let field_ident = format_ident!("{}", field_name);

            // Get the related model name
            let related_model_name = field.related_model().name();
            let related_builder_name = format!("{}WhereBuilder", related_model_name);
            let related_builder = format_ident!("{}", related_builder_name);

            quote! {
                #[inline]
                pub fn #field_ident<F>(&mut self, f: F) -> &mut Self
                where
                    F: FnOnce(&mut #related_builder),
                {
                    let mut nested = #related_builder::new();
                    f(&mut nested);
                    self.relation_filters.push((stringify!(#field_ident), Box::new(nested)));
                    self
                }
            }
        })
        .collect();

    quote! {
        #(#field_filters)*

        pub struct #builder_name {
            pub conditions: Vec<String>,
            pub relation_filters: Vec<(&'static str, Box<dyn std::any::Any>)>,
        }

        impl #builder_name {
            #[inline]
            pub fn new() -> Self {
                #builder_name {
                    conditions: Vec::new(),
                    relation_filters: Vec::new(),
                }
            }

            #(#field_methods)*
            #(#relation_methods)*

            pub fn conditions_count(&self) -> usize {
                self.conditions.len()
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate type-aware filter methods for scalar fields
fn get_scalar_filter_methods(
    _filter_name: &proc_macro2::Ident,
    builder_name: &proc_macro2::Ident,
    scalar_type: Option<parser_database::ScalarType>,
) -> TokenStream {
    match scalar_type {
        Some(parser_database::ScalarType::String) => {
            quote! {
                #[inline]
                pub fn eq(self, value: &str) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} = {}", self.field, value));
                    self.builder
                }

                #[inline]
                pub fn contains(self, value: &str) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} CONTAINS {}", self.field, value));
                    self.builder
                }

                #[inline]
                pub fn in_list(self, values: &[&str]) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} IN {:?}", self.field, values));
                    self.builder
                }
            }
        }
        Some(parser_database::ScalarType::Boolean) => {
            quote! {
                #[inline]
                pub fn eq(self, value: bool) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} = {}", self.field, value));
                    self.builder
                }

                #[inline]
                pub fn is_true(self) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} = true", self.field));
                    self.builder
                }

                #[inline]
                pub fn is_false(self) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} = false", self.field));
                    self.builder
                }
            }
        }
        Some(parser_database::ScalarType::Int) | Some(parser_database::ScalarType::BigInt) => {
            quote! {
                #[inline]
                pub fn eq(self, value: i64) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} = {}", self.field, value));
                    self.builder
                }

                #[inline]
                pub fn gt(self, value: i64) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} > {}", self.field, value));
                    self.builder
                }

                #[inline]
                pub fn gte(self, value: i64) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} >= {}", self.field, value));
                    self.builder
                }

                #[inline]
                pub fn lt(self, value: i64) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} < {}", self.field, value));
                    self.builder
                }

                #[inline]
                pub fn lte(self, value: i64) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} <= {}", self.field, value));
                    self.builder
                }
            }
        }
        _ => {
            // Default to string-like for unknown types
            quote! {
                #[inline]
                pub fn eq(self, value: &'static str) -> &'a mut #builder_name {
                    self.builder.conditions.push(format!("{} = {}", self.field, value));
                    self.builder
                }
            }
        }
    }
}
