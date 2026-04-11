use parser_database::walkers::ModelWalker;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate SelectBuilder for all models (scalar fields only)
pub fn generate_select_builders(db: &parser_database::ParserDatabase) -> TokenStream {
    let mut output = TokenStream::new();

    for model in db.walk_models() {
        output.extend(generate_select_builder(model.name(), model));
    }

    output
}

/// Generate SelectBuilder for a single model
fn generate_select_builder(model_name: &str, model: ModelWalker) -> TokenStream {
    let builder_name = format_ident!("{}SelectBuilder", model_name);

    // Collect all scalar field names for the .all() method
    let all_field_names: Vec<_> = model
        .scalar_fields()
        .map(|f| f.name())
        .collect();

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

            /// Select all scalar fields
            #[inline]
            pub fn all(&mut self) -> &mut Self {
                self.fields = vec![#(#all_field_names),*];
                self
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
