use heck::ToSnakeCase;
use parser_database::ParserDatabase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_client(db: &ParserDatabase) -> TokenStream {
    let mut enums = Vec::new();
    let mut models = Vec::new();

    // Generate Enums
    for walker in db.walk_enums() {
        let enum_name = format_ident!("{}", walker.name());
        let mut variants = Vec::new();

        for (i, val) in walker.values().enumerate() {
            let variant_name = format_ident!("{}", val.name());
            if i == 0 {
                variants.push(quote! {
                    #[default]
                    #variant_name
                });
            } else {
                variants.push(quote! {
                    #variant_name
                });
            }
        }

        enums.push(quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ::prisma_core::serde::Serialize, ::prisma_core::serde::Deserialize, Default)]
            #[serde(crate = "::prisma_core::serde", rename_all = "UPPERCASE")]
            pub enum #enum_name {
                #(#variants),*
            }

            impl From<#enum_name> for ::prisma_core::query_structure::PrismaValue {
                fn from(val: #enum_name) -> Self {
                    ::prisma_core::query_structure::PrismaValue::Enum(format!("{:?}", val).to_uppercase())
                }
            }
        });
    }

    // Generate Models with #[prisma_model] attribute
    for walker in db.walk_models() {
        let model_name = format_ident!("{}", walker.name());
        let mut fields = Vec::new();

        for field in walker.scalar_fields() {
            let prisma_field_name = field.name();
            let rust_field_name = format_ident!("{}", prisma_field_name.to_snake_case());

            let field_type = match field.scalar_field_type() {
                parser_database::ScalarFieldType::BuiltInScalar(builtin) => match builtin {
                    parser_database::ScalarType::String => quote! { String },
                    parser_database::ScalarType::Int => quote! { i32 },
                    parser_database::ScalarType::Float => quote! { f64 },
                    parser_database::ScalarType::Boolean => quote! { bool },
                    parser_database::ScalarType::DateTime => quote! { chrono::DateTime<chrono::Utc> },
                    parser_database::ScalarType::Json => quote! { serde_json::Value },
                    parser_database::ScalarType::Decimal => quote! { bigdecimal::BigDecimal },
                    parser_database::ScalarType::BigInt => quote! { i64 },
                    parser_database::ScalarType::Bytes => quote! { Vec<u8> },
                },
                parser_database::ScalarFieldType::Enum(enum_id) => {
                    let enum_name = format_ident!("{}", db.walk(enum_id).name());
                    quote! { #enum_name }
                }
                parser_database::ScalarFieldType::CompositeType(comp_id) => {
                    let comp_name = format_ident!("{}", db.walk(comp_id).name());
                    quote! { #comp_name }
                }
                _ => quote! { serde_json::Value },
            };

            let final_type = if field.is_optional() {
                quote! { Option<#field_type> }
            } else {
                field_type
            };

            let mut attrs = Vec::new();
            let mut prisma_meta = Vec::new();

            prisma_meta.push(quote! { name = #prisma_field_name });

            if field.is_single_pk() {
                prisma_meta.push(quote! { id });
            }
            if field.is_unique() {
                prisma_meta.push(quote! { unique });
            }

            attrs.push(quote! { #[prisma(#(#prisma_meta),*)] });
            attrs.push(quote! { #[serde(rename = #prisma_field_name)] });

            fields.push(quote! {
                #(#attrs)*
                pub #rust_field_name: #final_type
            });
        }

        for relation in walker.relation_fields() {
            let prisma_field_name = relation.name();
            let rust_field_name = format_ident!("{}", prisma_field_name.to_snake_case());
            let related_model = format_ident!("{}", relation.related_model().name());

            let final_type = if relation.ast_field().arity.is_list() {
                quote! { Vec<#related_model> }
            } else if !relation.is_required() {
                quote! { Option<#related_model> }
            } else {
                quote! { #related_model }
            };

            fields.push(quote! {
                #[serde(rename = #prisma_field_name)]
                #[prisma(name = #prisma_field_name, relation)]
                pub #rust_field_name: #final_type
            });
        }

        models.push(quote! {
            #[derive(Debug, ::prisma_core::serde::Deserialize, Default)]
            #[serde(crate = "::prisma_core::serde", default)]
            #[prisma_macros::prisma_model]
            pub struct #model_name {
                #(#fields),*
            }
        });
    }

    quote! {
        pub mod client {
            use super::*;
            use ::prisma_core as _prisma_core;

            pub fn client() -> _prisma_core::PrismaClient {
                todo!("Initialize client")
            }

            #(#enums)*

            #(#models)*
        }
    }
}
