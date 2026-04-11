use parser_database::walkers::ModelWalker;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate IncludeBuilder for all models (relation fields only)
pub fn generate_include_builders(db: &parser_database::ParserDatabase) -> TokenStream {
    let mut output = TokenStream::new();

    for model in db.walk_models() {
        output.extend(generate_include_builder(model.name(), model));
    }

    output
}

/// Generate IncludeBuilder for a single model
fn generate_include_builder(model_name: &str, model: ModelWalker) -> TokenStream {
    let builder_name = format_ident!("{}IncludeBuilder", model_name);

    // Generate method for each relation field (two methods: bare and with nested select)
    let relation_methods: Vec<_> = model
        .relation_fields()
        .map(|field| {
            let field_name = field.name();
            let field_ident = format_ident!("{}", field_name);
            let field_ident_with = format_ident!("{}_with", field_name);

            // Get the related model name
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
