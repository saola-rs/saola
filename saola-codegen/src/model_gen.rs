use heck::ToSnakeCase;
use psl::parser_database::{ParserDatabase, ScalarFieldType, ScalarType};
use quote::{format_ident, quote};

use crate::model_analysis::ModelMetadata;
use std::collections::HashMap;

pub fn generate_model_struct(
    db: &ParserDatabase,
    walker: psl::parser_database::walkers::ModelWalker<'_>,
    _metadata: &HashMap<String, ModelMetadata>,
) -> String {
    let model_name_str = walker.name();
    let model_name = format_ident!("{}", model_name_str);
    let data_struct_name = format_ident!("{}Data", model_name_str);
    let inner_mod_name = format_ident!("_{}", model_name_str.to_snake_case());

    let mut fields = Vec::new();
    let mut relation_generic_params = Vec::new();
    let mut relation_generic_defaults = Vec::new();
    let mut prisma_scalar_names = Vec::new();
    let mut field_markers = Vec::new();
    let mut from_ir_fields = Vec::new();

    for field in walker.scalar_fields() {
        let prisma_field_name = field.name();
        prisma_scalar_names.push(prisma_field_name.to_string());
        let rust_field_name = format_ident!("{}", prisma_field_name.to_snake_case());
        let const_field_name = format_ident!("{}", prisma_field_name.to_snake_case().to_uppercase());

        let field_type = match field.scalar_field_type() {
            ScalarFieldType::BuiltInScalar(builtin) => match builtin {
                ScalarType::String => quote! { String },
                ScalarType::Int => quote! { i32 },
                ScalarType::BigInt => quote! { i64 },
                ScalarType::Float => quote! { f64 },
                ScalarType::Decimal => quote! { ::saola_core::bigdecimal::BigDecimal },
                ScalarType::Boolean => quote! { bool },
                ScalarType::DateTime => quote! { ::saola_core::chrono::DateTime<::saola_core::chrono::Utc> },
                ScalarType::Json => quote! { ::saola_core::serde_json::Value },
                ScalarType::Bytes => quote! { Vec<u8> },
            },
            ScalarFieldType::Enum(id) => {
                let enum_walker = db.walk(id);
                let enum_name = format_ident!("{}", enum_walker.name());
                quote! { enums::#enum_name }
            }
            _ => quote! { ::saola_core::serde_json::Value },
        };

        let final_type = if !field.ast_field().arity.is_required() {
            quote! { Option<#field_type> }
        } else {
            field_type.clone()
        };

        fields.push(quote! {
            #[serde(rename = #prisma_field_name)]
            pub #rust_field_name: #final_type
        });

        from_ir_fields.push(if !field.ast_field().arity.is_required() {
            quote! {
                #rust_field_name: map.shift_remove(#prisma_field_name)
                    .map(::saola_core::builder::FromResponseIr::from_ir)
                    .transpose()?
                    .flatten()
            }
        } else {
            quote! {
                #rust_field_name: map.shift_remove(#prisma_field_name)
                    .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", #prisma_field_name)))
                    .and_then(::saola_core::builder::FromResponseIr::from_ir)?
            }
        });

        field_markers.push(quote! {
            pub const #const_field_name: ::saola_core::Field<#field_type> = ::saola_core::Field::new(#prisma_field_name);
            pub type #rust_field_name = #final_type;
        });
    }

    for relation in walker.relation_fields() {
        let prisma_field_name = relation.name();
        let rust_field_name = format_ident!("{}", prisma_field_name.to_snake_case());
        let generic_param = format_ident!("{}", heck::AsPascalCase(prisma_field_name).to_string());

        relation_generic_params.push(quote! { #generic_param });
        relation_generic_defaults.push(quote! { #generic_param = ::saola_core::Unloaded });

        fields.push(quote! {
            #[serde(rename = #prisma_field_name, default)]
            pub #rust_field_name: #generic_param
        });

        from_ir_fields.push(quote! {
            #rust_field_name: map.shift_remove(#prisma_field_name)
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default()
        });

        field_markers.push(quote! {
            pub type #rust_field_name = ();
        });
    }

    let struct_generics = if relation_generic_params.is_empty() {
        quote! {}
    } else {
        quote! { <#(#relation_generic_defaults),*> }
    };

    let impl_generics = if relation_generic_params.is_empty() {
        quote! {}
    } else {
        quote! { <#(#relation_generic_params),*> }
    };

    let res = quote! {
        #[derive(Debug, Clone, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde")]
        pub struct #data_struct_name #struct_generics {
            #(#fields),*
        }

        pub type #model_name = #data_struct_name;

        impl ::saola_core::builder::SelectStruct for #data_struct_name {
            fn selections() -> Vec<::saola_core::query_core::Selection> {
                vec![
                    #(::saola_core::query_core::Selection::with_name(#prisma_scalar_names.to_string())),*
                ]
            }
        }

        impl ::saola_core::builder::GetSelections for #data_struct_name {
            fn get_selections() -> Vec<::saola_core::query_core::Selection> {
                <Self as ::saola_core::builder::SelectStruct>::selections()
            }
        }

        impl #impl_generics ::saola_core::builder::FromResponseIr for #data_struct_name #impl_generics
        where
            #(#relation_generic_params: ::saola_core::builder::FromResponseIr + Default),*
        {
            fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
                let mut map = match item {
                    ::saola_core::query_core::response_ir::Item::Map(m) => m,
                    ::saola_core::query_core::response_ir::Item::Ref(r) => match r.as_ref() {
                        ::saola_core::query_core::response_ir::Item::Map(m) => m.clone(),
                        _ => return Err(::saola_core::Error::RuntimeError("Expected map in response ref".to_string())),
                    },
                    _ => return Err(::saola_core::Error::RuntimeError(format!("Expected map in response, got {:?}", item))),
                };

                Ok(Self {
                    #(#from_ir_fields),*
                })
            }
        }

        #[allow(dead_code)]
        pub mod #inner_mod_name {
            #[allow(unused_imports)]
            pub use super::#data_struct_name;
            pub mod fields {
                use super::super::enums;
                #(#field_markers)*
            }
        }
    };

    res.to_string()
}

pub fn generate_enum(_db: &ParserDatabase, walker: psl::parser_database::walkers::EnumWalker<'_>) -> String {
    let enum_name = format_ident!("{}", walker.name());
    let mut variants = Vec::new();
    let mut from_prisma_value_arms = Vec::new();
    let mut to_prisma_value_arms = Vec::new();

    for variant in walker.values() {
        let variant_name_str = variant.name();
        let variant_ident = format_ident!("{}", heck::AsPascalCase(variant_name_str).to_string());

        variants.push(quote! { #variant_ident });

        from_prisma_value_arms.push(quote! {
            #variant_name_str => Ok(Self::#variant_ident)
        });

        to_prisma_value_arms.push(quote! {
            #enum_name::#variant_ident => PrismaValue::Enum(#variant_name_str.to_string())
        });
    }

    let res = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        pub enum #enum_name {
            #[default]
            #(#variants),*
        }

        impl #enum_name {
            pub fn from_str(s: &str) -> Result<Self, String> {
                match s {
                    #(#from_prisma_value_arms),*,
                    _ => Err(format!("Unknown variant: {}", s)),
                }
            }
        }

        impl From<#enum_name> for PrismaValue {
            fn from(e: #enum_name) -> Self {
                match e {
                    #(#to_prisma_value_arms),*
                }
            }
        }

        impl ::saola_core::builder::FromResponseIr for #enum_name {
            fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
                match item {
                    ::saola_core::query_core::response_ir::Item::Value(::saola_core::query_structure::PrismaValue::Enum(s)) => {
                        Self::from_str(&s).map_err(|e| ::saola_core::Error::RuntimeError(e))
                    }
                    _ => Err(::saola_core::Error::RuntimeError("Expected enum in response".to_string())),
                }
            }
        }
    };

    res.to_string()
}
