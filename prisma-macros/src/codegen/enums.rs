use parser_database::ParserDatabase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate Rust enums from schema enums
pub fn generate_enums(db: &ParserDatabase) -> TokenStream {
    let mut output = TokenStream::new();

    // Iterate through all enums in the schema
    for enm in db.walk_enums() {
        let enum_name = enm.name();
        let enum_ident = format_ident!("{}", enum_name);

        // Generate enum variants (PascalCase from SCREAMING_SNAKE_CASE)
        let variants: Vec<_> = enm
            .values()
            .map(|variant| {
                let variant_name = variant.name();
                let variant_ident = format_ident!("{}", to_pascal_case(variant_name));
                quote! { #variant_ident }
            })
            .collect();

        // Generate as_str() method
        let as_str_arms: Vec<_> = enm
            .values()
            .map(|variant| {
                let variant_name = variant.name();
                let variant_ident = format_ident!("{}", to_pascal_case(variant_name));
                quote! {
                    #enum_ident::#variant_ident => #variant_name,
                }
            })
            .collect();

        // Generate From<&str> impl
        let from_str_arms: Vec<_> = enm
            .values()
            .map(|variant| {
                let variant_name = variant.name();
                let variant_ident = format_ident!("{}", to_pascal_case(variant_name));
                quote! {
                    #variant_name => #enum_ident::#variant_ident,
                }
            })
            .collect();

        output.extend(quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
            #[serde(rename_all = "UPPERCASE")]
            pub enum #enum_ident {
                #(#variants),*
            }

            impl #enum_ident {
                pub fn as_str(&self) -> &'static str {
                    match self {
                        #(#as_str_arms)*
                    }
                }
            }

            impl From<&str> for #enum_ident {
                fn from(s: &str) -> Self {
                    match s {
                        #(#from_str_arms)*
                        _ => panic!("Unknown {}: {}", stringify!(#enum_ident), s),
                    }
                }
            }

            impl From<String> for #enum_ident {
                fn from(s: String) -> Self {
                    #enum_ident::from(s.as_str())
                }
            }

            impl std::fmt::Display for #enum_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.as_str())
                }
            }
        });
    }

    output
}

/// Convert SCREAMING_SNAKE_CASE to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let lowercase = word.to_lowercase();
            let mut chars = lowercase.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
