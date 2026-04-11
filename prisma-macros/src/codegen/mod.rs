use parser_database::ParserDatabase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod enums;

/// Generate all client code from the parsed schema
pub fn generate_client(db: &ParserDatabase) -> TokenStream {
    let mut output = TokenStream::new();

    // 0. Generate enums first (needed for filter methods)
    output.extend(enums::generate_enums(db));

    for model in db.walk_models() {
        let model_name = model.name();

        // 1. Generate SelectBuilder for this model
        output.extend(generate_select_builder(model_name, model));

        // 2. Generate IncludeBuilder for this model
        output.extend(generate_include_builder(model_name, model));

        // 3. Generate WhereBuilder for this model
        output.extend(generate_where_builder(model_name, model));

        // 3. Generate all query builders
        output.extend(generate_find_many_builder(model_name));
        output.extend(generate_find_unique_builder(model_name));
        output.extend(generate_create_builder(model_name));
        output.extend(generate_update_builder(model_name));
        output.extend(generate_delete_builder(model_name));

        // 4. Generate model namespace with query entry point
        output.extend(generate_model_namespace(model_name));
    }

    // 5. Generate client root entry point
    output.extend(generate_client_root(db));

    output
}

/// Generate SelectBuilder for a model
fn generate_select_builder(
    model_name: &str,
    model: parser_database::walkers::ModelWalker,
) -> TokenStream {
    let builder_name = format_ident!("{}SelectBuilder", model_name);

    // Generate method for each scalar field only
    let field_methods: Vec<_> = model
        .scalar_fields()
        .map(|field| {
            let field_name = field.name();
            let field_ident = format_ident!("{}", field_name);

            quote! {
                #[inline]
                pub fn #field_ident(&mut self) -> &mut Self {
                    self.fields.push(stringify!(#field_ident));
                    self
                }
            }
        })
        .collect();

    quote! {
        pub struct #builder_name {
            pub fields: Vec<&'static str>,
        }

        impl #builder_name {
            #[inline]
            pub fn new() -> Self {
                #builder_name {
                    fields: Vec::new(),
                }
            }

            #(#field_methods)*
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate IncludeBuilder for a model (relation fields only)
fn generate_include_builder(
    model_name: &str,
    model: parser_database::walkers::ModelWalker,
) -> TokenStream {
    let builder_name = format_ident!("{}IncludeBuilder", model_name);

    // Generate method for each relation field (two methods: with and without nested select)
    let relation_methods: Vec<_> = model
        .relation_fields()
        .map(|field| {
            let field_name = field.name();
            let field_ident = format_ident!("{}", field_name);
            let field_ident_with = format_ident!("{}_with", field_name);

            // Get the related model name properly
            let related_model_name = field.related_model().name();
            let related_select_name = format!("{}SelectBuilder", related_model_name);
            let related_select = format_ident!("{}", related_select_name);

            quote! {
                #[inline]
                pub fn #field_ident(&mut self) -> &mut Self {
                    self.relations.push((stringify!(#field_ident), None));
                    self
                }

                #[inline]
                pub fn #field_ident_with<F>(&mut self, f: F) -> &mut Self
                where
                    F: FnOnce(&mut #related_select),
                {
                    let mut nested = #related_select::new();
                    f(&mut nested);
                    self.relations.push((stringify!(#field_ident), Some(Box::new(nested))));
                    self
                }
            }
        })
        .collect();

    quote! {
        pub struct #builder_name {
            pub relations: Vec<(&'static str, Option<Box<dyn std::any::Any>>)>,
        }

        impl #builder_name {
            #[inline]
            pub fn new() -> Self {
                #builder_name {
                    relations: Vec::new(),
                }
            }

            #(#relation_methods)*
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate WhereBuilder for a model
fn generate_where_builder(
    model_name: &str,
    model: parser_database::walkers::ModelWalker,
) -> TokenStream {
    let builder_name = format_ident!("{}WhereBuilder", model_name);

    // Generate a field filter struct for each scalar field
    let field_filters: Vec<_> = model
        .scalar_fields()
        .map(|field| {
            let field_name = field.name();
            let filter_name = format_ident!("{}{}", model_name, capitalize_first(field_name));
            let scalar_type = field.scalar_type();
            let filter_methods = get_scalar_filter_methods(&filter_name, &builder_name, scalar_type);

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
            let filter_name = format_ident!("{}{}", model_name, capitalize_first(field_name));

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
            
            // Get the related model name properly
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

/// Generate FindMany builder
fn generate_find_many_builder(model_name: &str) -> TokenStream {
    let builder_name = format_ident!("FindMany{}Builder", model_name);
    let where_builder = format_ident!("{}WhereBuilder", model_name);
    let select_builder = format_ident!("{}SelectBuilder", model_name);
    let include_builder = format_ident!("{}IncludeBuilder", model_name);

    quote! {
        pub struct #builder_name {
            pub where_clause: Option<#where_builder>,
            pub take: Option<i64>,
            pub skip: Option<i64>,
        }

        impl #builder_name {
            #[inline]
            pub fn new() -> Self {
                #builder_name {
                    where_clause: None,
                    take: None,
                    skip: None,
                }
            }

            #[inline]
            pub fn where_clause<F>(mut self, f: F) -> Self
            where
                F: FnOnce(&mut #where_builder),
            {
                let mut w = #where_builder::new();
                f(&mut w);
                self.where_clause = Some(w);
                self
            }

            #[inline]
            pub fn select<F>(mut self, f: F) -> Self
            where
                F: FnOnce(&mut #select_builder),
            {
                let mut _s = #select_builder::new();
                f(&mut _s);
                self
            }

            #[inline]
            pub fn include<F>(mut self, f: F) -> Self
            where
                F: FnOnce(&mut #include_builder),
            {
                let mut _i = #include_builder::new();
                f(&mut _i);
                self
            }

            #[inline]
            pub fn take(mut self, n: i64) -> Self {
                self.take = Some(n);
                self
            }

            #[inline]
            pub fn skip(mut self, n: i64) -> Self {
                self.skip = Some(n);
                self
            }

            pub async fn exec(self) -> Result<Vec<serde_json::Value>, String> {
                Err("Phase 2: Execution not yet implemented".to_string())
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate FindUnique builder
fn generate_find_unique_builder(model_name: &str) -> TokenStream {
    let builder_name = format_ident!("FindUnique{}Builder", model_name);

    quote! {
        pub struct #builder_name {
            pub where_clause: Option<String>,
        }

        impl #builder_name {
            #[inline]
            pub fn new() -> Self {
                #builder_name {
                    where_clause: None,
                }
            }

            pub async fn exec(self) -> Result<Option<serde_json::Value>, String> {
                Err("Phase 2: Execution not yet implemented".to_string())
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate Create builder
fn generate_create_builder(model_name: &str) -> TokenStream {
    let builder_name = format_ident!("Create{}Builder", model_name);

    quote! {
        pub struct #builder_name {
            pub data: std::collections::HashMap<String, serde_json::Value>,
        }

        impl #builder_name {
            #[inline]
            pub fn new() -> Self {
                #builder_name {
                    data: std::collections::HashMap::new(),
                }
            }

            pub async fn exec(self) -> Result<serde_json::Value, String> {
                Err("Phase 2: Execution not yet implemented".to_string())
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate Update builder
fn generate_update_builder(model_name: &str) -> TokenStream {
    let builder_name = format_ident!("Update{}Builder", model_name);

    quote! {
        pub struct #builder_name {
            pub where_clause: Option<String>,
            pub data: std::collections::HashMap<String, serde_json::Value>,
        }

        impl #builder_name {
            #[inline]
            pub fn new() -> Self {
                #builder_name {
                    where_clause: None,
                    data: std::collections::HashMap::new(),
                }
            }

            pub async fn exec(self) -> Result<serde_json::Value, String> {
                Err("Phase 2: Execution not yet implemented".to_string())
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate Delete builder
fn generate_delete_builder(model_name: &str) -> TokenStream {
    let builder_name = format_ident!("Delete{}Builder", model_name);

    quote! {
        pub struct #builder_name {
            pub where_clause: Option<String>,
        }

        impl #builder_name {
            #[inline]
            pub fn new() -> Self {
                #builder_name {
                    where_clause: None,
                }
            }

            pub async fn exec(self) -> Result<serde_json::Value, String> {
                Err("Phase 2: Execution not yet implemented".to_string())
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}
fn generate_model_namespace(model_name: &str) -> TokenStream {
    let query_struct = format_ident!("{}Query", model_name);
    let find_many = format_ident!("FindMany{}Builder", model_name);
    let find_unique = format_ident!("FindUnique{}Builder", model_name);
    let create = format_ident!("Create{}Builder", model_name);
    let update = format_ident!("Update{}Builder", model_name);
    let delete = format_ident!("Delete{}Builder", model_name);

    quote! {
        pub struct #query_struct;

        impl #query_struct {
            #[inline]
            pub fn find_many(self) -> #find_many {
                #find_many::new()
            }

            #[inline]
            pub fn find_unique(self) -> #find_unique {
                #find_unique::new()
            }

            #[inline]
            pub fn create(self) -> #create {
                #create::new()
            }

            #[inline]
            pub fn update(self) -> #update {
                #update::new()
            }

            #[inline]
            pub fn delete(self) -> #delete {
                #delete::new()
            }
        }
    }
}

/// Generate client root entry point
fn generate_client_root(db: &ParserDatabase) -> TokenStream {
    let mut models = Vec::new();

    for model in db.walk_models() {
        let model_name = model.name();
        let model_snake = to_snake_case(model_name);
        let model_ident = format_ident!("{}", model_snake);
        let query_struct = format_ident!("{}Query", model_name);

        models.push(quote! {
            #[inline]
            pub fn #model_ident() -> #query_struct {
                #query_struct
            }
        });
    }

    quote! {
        #(#models)*
    }
}

// ============== Helper Functions ==============

/// Generate type-aware filter methods based on scalar type
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
        },
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
        },
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
        },
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

// ============== Helper Functions ==============

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap_or(c));
    }
    result
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
