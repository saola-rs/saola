//! Logic for the select! macro which generates ad-hoc structs for partial selection

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use syn::{
    Result, Token, Type, braced,
    parse::{Parse, ParseStream},
};

/// Parsed shape of the ad-hoc struct
pub struct SelectShape {
    pub fields: Vec<FieldShape>,
}

pub struct FieldShape {
    pub name: Ident,
    pub is_optional: bool,
    pub ty: FieldType,
}

pub enum FieldType {
    Scalar(Type),
    Nested(SelectShapeNested),
}

pub struct SelectShapeNested {
    pub is_array: bool,
    pub fields: Vec<FieldShape>,
}

impl Parse for SelectShape {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut fields = Vec::new();
        while !content.is_empty() {
            fields.push(content.parse()?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(SelectShape { fields })
    }
}

impl Parse for FieldShape {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let is_optional = if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            true
        } else {
            false
        };
        input.parse::<Token![:]>()?;

        if input.peek(Token![struct]) || input.peek(syn::token::Brace) {
            if input.peek(Token![struct]) {
                input.parse::<Token![struct]>()?;
            }

            let content;
            braced!(content in input);

            let mut fields = Vec::new();
            while !content.is_empty() {
                fields.push(content.parse()?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }

            let is_array = if input.peek(syn::token::Bracket) {
                let _brackets;
                syn::bracketed!(_brackets in input);
                true
            } else {
                false
            };

            Ok(FieldShape {
                name,
                is_optional,
                ty: FieldType::Nested(SelectShapeNested { is_array, fields }),
            })
        } else {
            let ty: Type = input.parse()?;
            Ok(FieldShape {
                name,
                is_optional,
                ty: FieldType::Scalar(ty),
            })
        }
    }
}

fn generate_struct_and_selects(
    fields: &[FieldShape],
    prefix: &str,
) -> (TokenStream, TokenStream, TokenStream, Vec<Ident>, Vec<String>) {
    let mut struct_fields = Vec::new();
    let mut select_calls = Vec::new();
    let mut nested_structs = Vec::new();
    let mut names = Vec::new();
    let mut prisma_names = Vec::new();

    for field in fields {
        let name = &field.name;
        let prisma_name = ::heck::ToLowerCamelCase::to_lower_camel_case(name.to_string().as_str());
        names.push(name.clone());
        prisma_names.push(prisma_name.clone());

        match &field.ty {
            FieldType::Scalar(ty) => {
                let final_ty = if field.is_optional {
                    quote! { Option<#ty> }
                } else {
                    quote! { #ty }
                };

                struct_fields.push(quote! {
                    #[serde(rename = #prisma_name)]
                    pub #name: #final_ty
                });
                // In select builder, scalars are methods that return SelectionField
                select_calls.push(quote! { s.#name().assert_type(&_check.#name); });
            }
            FieldType::Nested(nested) => {
                let sub_prefix = format!("{}_{}", prefix, name);
                let sub_struct_name = format_ident!("_AdHoc_{}", sub_prefix);

                let (sub_fields, sub_selects, sub_nested, sub_names, sub_prisma_names) =
                    generate_struct_and_selects(&nested.fields, &sub_prefix);

                nested_structs.push(sub_nested);
                nested_structs.push(quote! {
                    #[derive(Clone, ::saola_core::serde::Deserialize, ::saola_core::serde::Serialize, Default)]
                    #[serde(crate = "::saola_core::serde", default)]
                    pub struct #sub_struct_name {
                        #sub_fields
                    }

                    impl std::fmt::Debug for #sub_struct_name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.debug_struct("Selected")
                                #(.field(#sub_prisma_names, &self.#sub_names))*
                                .finish()
                        }
                    }
                });

                let final_nested_ty = if nested.is_array {
                    quote! { Vec<#sub_struct_name> }
                } else {
                    quote! { Option<Box<#sub_struct_name>> }
                };

                if field.is_optional {
                    // If the relation itself is marked optional in select macro, wrap in Option
                    // Note: for to-one it's already Option, but this allows explicitly showing it's optional
                    // Actually for relations, optionality is usually determined by the schema.
                    // But if they write `user?: { ... }`, we should ensure it's Option.
                    // To-one is already Option<Box<...>>.
                    // To-many is Vec<...>. We don't usually make to-many optional.
                }

                struct_fields.push(quote! {
                    #[serde(rename = #prisma_name)]
                    pub #name: #final_nested_ty
                });

                // In select builder, relations return SelectionRelField
                select_calls.push(quote! {
                    s.#name(|s| {
                        let _check: #sub_struct_name = Default::default();
                        #sub_selects
                    }).assert_rel_type(&_check.#name);
                });
            }
        }
    }

    (
        quote! { #(#struct_fields),* },
        quote! { #(#select_calls)* },
        quote! {
            #(#nested_structs)*
        },
        names,
        prisma_names,
    )
}

pub fn select_macro_impl(input: SelectShape) -> TokenStream {
    // Generate a unique hash based on the field names
    let mut hasher = DefaultHasher::new();
    for field in &input.fields {
        field.name.to_string().hash(&mut hasher);
    }
    let hash = hasher.finish();

    let root_struct_name = format_ident!("_AdHocRoot_{:x}", hash);
    let (fields, selects, nested, names, prisma_names) =
        generate_struct_and_selects(&input.fields, &format!("{:x}", hash));

    quote! {
        {
            #nested

            #[derive(Clone, ::saola_core::serde::Deserialize, ::saola_core::serde::Serialize, Default)]
            #[serde(crate = "::saola_core::serde", default)]
            pub struct #root_struct_name {
                #fields
            }

            impl std::fmt::Debug for #root_struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct("Selected")
                        #(.field(#prisma_names, &self.#names))*
                        .finish()
                }
            }

            impl std::fmt::Display for #root_struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let json = ::saola_core::serde_json::to_string_pretty(self).unwrap_or_else(|_| "Error serializing".to_string());
                    write!(f, "{}", json)
                }
            }

            (
                std::marker::PhantomData::<#root_struct_name>,
                |s| {
                    let _check: #root_struct_name = Default::default();
                    #selects
                }
            )
        }
    }
}

pub fn generate_as_type(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input_str = input.to_string();
    let mut hasher = DefaultHasher::new();
    input_str.hash(&mut hasher);
    let hash = hasher.finish();
    let struct_name = format_ident!("_AdHocType_{:x}", hash);

    quote! { #struct_name }
}
