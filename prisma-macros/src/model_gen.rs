use psl::parser_database::{ParserDatabase, ScalarType, ScalarFieldType};
use quote::{format_ident, quote};
use heck::ToSnakeCase;

pub fn generate_model_struct(db: &ParserDatabase, walker: psl::parser_database::walkers::ModelWalker<'_>) -> proc_macro2::TokenStream {
    let model_name = format_ident!("{}", walker.name());

    let mut fields = Vec::new();

    for field in walker.scalar_fields() {
        let prisma_field_name = field.name();
        let rust_field_name = format_ident!("{}", prisma_field_name.to_snake_case());

        let field_type = match field.scalar_field_type() {
            ScalarFieldType::BuiltInScalar(builtin) => match builtin {
                ScalarType::String => quote! { String },
                ScalarType::Int => quote! { i32 },
                ScalarType::Float => quote! { f64 },
                ScalarType::Boolean => quote! { bool },
                ScalarType::DateTime => quote! { ::prisma_core::chrono::DateTime<::prisma_core::chrono::Utc> },
                ScalarType::Json => quote! { ::prisma_core::serde_json::Value },
                ScalarType::Decimal => quote! { ::prisma_core::bigdecimal::BigDecimal },
                ScalarType::BigInt => quote! { i64 },
                ScalarType::Bytes => quote! { Vec<u8> },
            },
            ScalarFieldType::Enum(enum_id) => {
                let enum_name = format_ident!("{}", db.walk(enum_id).name());
                quote! { #enum_name }
            }
            ScalarFieldType::CompositeType(comp_id) => {
                let comp_name = format_ident!("{}", db.walk(comp_id).name());
                quote! { #comp_name }
            }
            _ => quote! { ::prisma_core::serde_json::Value },
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
            quote! { Option<Box<#related_model>> }
        } else {
            quote! { Box<#related_model> }
        };

        fields.push(quote! {
            #[serde(default, skip_serializing)]
            #[prisma(name = #prisma_field_name, relation)]
            pub #rust_field_name: #final_type
        });
    }

    quote! {
        #[::prisma_macros::prisma_model]
        #[derive(Debug, Clone, ::prisma_core::serde::Serialize, ::prisma_core::serde::Deserialize, Default)]
        #[serde(crate = "::prisma_core::serde", default)]
        pub struct #model_name {
            #(#fields),*
        }
    }
}

pub fn generate_enum(db: &ParserDatabase, walker: psl::parser_database::walkers::EnumWalker<'_>) -> proc_macro2::TokenStream {
    let enum_name = format_ident!("{}", walker.name());
    let mut variants = Vec::new();

    let mut match_arms_display = Vec::new();

    for (i, val) in walker.values().enumerate() {
        let variant_name = format_ident!("{}", val.name());
        let s = val.name();
        
        match_arms_display.push(quote! {
            #enum_name::#variant_name => write!(f, #s)
        });

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

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ::prisma_core::serde::Serialize, ::prisma_core::serde::Deserialize, Default)]
        #[serde(crate = "::prisma_core::serde", rename_all = "UPPERCASE")]
        pub enum #enum_name {
            #(#variants),*
        }

        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#match_arms_display),*
                }
            }
        }

        impl From<#enum_name> for ::prisma_core::query_structure::PrismaValue {
            fn from(val: #enum_name) -> Self {
                ::prisma_core::query_structure::PrismaValue::Enum(format!("{:?}", val).to_uppercase())
            }
        }
    }
}

/// Generate relation combination types for a model
/// E.g., UserWithPosts, UserWithPostsAndComments
pub fn generate_relation_types(db: &ParserDatabase, walker: psl::parser_database::walkers::ModelWalker<'_>) -> proc_macro2::TokenStream {
    let model_name = walker.name();
    let mut types = Vec::new();

    let relations: Vec<_> = walker.relation_fields().collect();

    if relations.is_empty() {
        return quote! {};
    }

    // Generate single relation types: UserWithPosts, UserWithComments
    for relation in &relations {
        let relation_name = relation.name();
        let pascal_relation = pascal_case(relation_name);
        let type_name = format_ident!("{}With{}", model_name, pascal_relation);
        let related_model = relation.related_model().name();
        let related_ident = format_ident!("{}", related_model);
        let rust_relation_name = format_ident!("{}", relation_name.to_snake_case());

        let related_type = if relation.ast_field().arity.is_list() {
            quote! { Vec<#related_ident> }
        } else if !relation.is_required() {
            quote! { Option<Box<#related_ident>> }
        } else {
            quote! { Box<#related_ident> }
        };

        // Get scalar fields from parent model
        let scalar_fields: Vec<_> = walker
            .scalar_fields()
            .map(|f| {
                let field_name = f.name();
                let rust_name = format_ident!("{}", field_name.to_snake_case());
                let field_type = match f.scalar_field_type() {
                    ScalarFieldType::BuiltInScalar(builtin) => match builtin {
                        ScalarType::String => quote! { String },
                        ScalarType::Int => quote! { i32 },
                        ScalarType::Float => quote! { f64 },
                        ScalarType::Boolean => quote! { bool },
                        ScalarType::DateTime => quote! { ::prisma_core::chrono::DateTime<::prisma_core::chrono::Utc> },
                        ScalarType::Json => quote! { ::prisma_core::serde_json::Value },
                        ScalarType::Decimal => quote! { ::prisma_core::bigdecimal::BigDecimal },
                        ScalarType::BigInt => quote! { i64 },
                        ScalarType::Bytes => quote! { Vec<u8> },
                    },
                    ScalarFieldType::Enum(enum_id) => {
                        let enum_name = format_ident!("{}", db.walk(enum_id).name());
                        quote! { #enum_name }
                    }
                    _ => quote! { String },
                };

                let final_type = if f.is_optional() {
                    quote! { Option<#field_type> }
                } else {
                    field_type
                };

                quote! {
                    pub #rust_name: #final_type,
                }
            })
            .collect();

        types.push(quote! {
            #[derive(Debug, Clone, ::prisma_core::serde::Serialize, ::prisma_core::serde::Deserialize)]
            #[serde(crate = "::prisma_core::serde")]
            pub struct #type_name {
                #(#scalar_fields)*
                pub #rust_relation_name: #related_type,
            }
        });
    }

    // Combine all types
    quote! {
        #(#types)*
    }
}

fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + chars.as_str()
                }
            }
        })
        .collect()
}
