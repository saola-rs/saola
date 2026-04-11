use parser_database::ParserDatabase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Helper function for snake_case conversion
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

/// Generate all query builders for all models
pub fn generate_query_builders(db: &ParserDatabase) -> TokenStream {
    let mut output = TokenStream::new();

    for model in db.walk_models() {
        let model_name = model.name();
        output.extend(generate_find_many_builder(model_name));
        output.extend(generate_find_unique_builder(model_name));
        output.extend(generate_create_builder(model_name));
        output.extend(generate_update_builder(model_name));
        output.extend(generate_delete_builder(model_name));
    }

    output
}

/// Generate model namespaces for all models
pub fn generate_model_namespaces(db: &ParserDatabase) -> TokenStream {
    let mut output = TokenStream::new();

    for model in db.walk_models() {
        output.extend(generate_model_namespace(model.name()));
    }

    output
}

/// Generate client root entry point with model accessors
pub fn generate_client_root(db: &ParserDatabase) -> TokenStream {
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

/// Generate FindMany builder for a model
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
                Err("Phase 3: Execution not yet implemented".to_string())
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate FindUnique builder for a model
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
                Err("Phase 3: Execution not yet implemented".to_string())
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate Create builder for a model
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
                Err("Phase 3: Execution not yet implemented".to_string())
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate Update builder for a model
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
                Err("Phase 3: Execution not yet implemented".to_string())
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate Delete builder for a model
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
                Err("Phase 3: Execution not yet implemented".to_string())
            }
        }

        impl Default for #builder_name {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

/// Generate model namespace (QueryStruct + methods for each builder type)
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
