use heck::ToSnakeCase;
use psl::parser_database::{ParserDatabase, ScalarFieldType, ScalarType};
use quote::{format_ident, quote};

use crate::model_analysis::ModelMetadata;
use std::collections::HashMap;

pub fn generate_model_struct(
    db: &ParserDatabase,
    walker: psl::parser_database::walkers::ModelWalker<'_>,
    all_metadata: &HashMap<String, ModelMetadata>,
) -> proc_macro2::TokenStream {
    let model_name_str = walker.name().to_string();
    let _model_metadata = all_metadata.get(&model_name_str).unwrap();
    let model_name = format_ident!("{}", model_name_str);

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
                ScalarType::DateTime => quote! { ::saola_core::chrono::DateTime<::saola_core::chrono::Utc> },
                ScalarType::Json => quote! { ::saola_core::serde_json::Value },
                ScalarType::Decimal => quote! { ::saola_core::bigdecimal::BigDecimal },
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
            _ => quote! { ::saola_core::serde_json::Value },
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

        let (final_type, attrs) = if relation.ast_field().arity.is_list() {
            (
                quote! { Vec<#related_model> },
                quote! { #[serde(default, skip_serializing_if = "Vec::is_empty")] },
            )
        } else {
            (
                quote! { Option<Box<#related_model>> },
                quote! { #[serde(default, skip_serializing_if = "Option::is_none")] },
            )
        };

        fields.push(quote! {
            #attrs
            pub #rust_field_name: #final_type
        });
    }

    quote! {
        #[derive(Debug, Clone, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde", default)]
        pub struct #model_name {
            #(#fields),*
        }
    }
}

pub fn generate_enum(
    _db: &ParserDatabase,
    walker: psl::parser_database::walkers::EnumWalker<'_>,
) -> proc_macro2::TokenStream {
    let enum_name = format_ident!("{}", walker.name());
    let mut variants = Vec::new();
    let mut arms = Vec::new();

    for variant in walker.values() {
        let variant_name = format_ident!("{}", variant.name());
        let prisma_name = variant.name();
        variants.push(quote! {
            #[serde(rename = #prisma_name)]
            #variant_name
        });
        arms.push(quote! {
            #enum_name::#variant_name => ::saola_core::query_structure::PrismaValue::Enum(#prisma_name.to_string())
        });
    }

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde")]
        pub enum #enum_name {
            #[default]
            #(#variants),*
        }

        impl From<#enum_name> for ::saola_core::query_structure::PrismaValue {
            fn from(val: #enum_name) -> Self {
                match val {
                    #(#arms),*
                }
            }
        }
    }
}

pub fn generate_relation_types(
    _db: &ParserDatabase,
    walker: psl::parser_database::walkers::ModelWalker<'_>,
) -> proc_macro2::TokenStream {
    let model_name = walker.name();
    let mut types = Vec::new();

    let mut relations = Vec::new();
    for rf in walker.relation_fields() {
        relations.push(rf);
    }

    // Generate powerset of relation inclusions
    let mut subsets = vec![vec![]];
    for rel in relations {
        let mut new_subsets = Vec::new();
        for subset in &subsets {
            let mut new_subset = subset.clone();
            new_subset.push(rel);
            new_subsets.push(new_subset);
        }
        subsets.extend(new_subsets);
    }

    for subset in subsets {
        if subset.is_empty() {
            continue;
        }

        let mut sorted_subset = subset.clone();
        sorted_subset.sort_by_key(|rf| rf.name());

        let names: Vec<_> = sorted_subset.iter().map(|rf| pascal_case(rf.name())).collect();
        let type_name = format_ident!("{}With{}", model_name, names.join("And"));

        let mut scalar_fields = Vec::new();
        for field in walker.scalar_fields() {
            let prisma_name = field.name();
            let rust_name = format_ident!("{}", prisma_name.to_snake_case());
            let field_type = match field.scalar_field_type() {
                ScalarFieldType::BuiltInScalar(builtin) => match builtin {
                    ScalarType::String => quote! { String },
                    ScalarType::Int => quote! { i32 },
                    ScalarType::Float => quote! { f64 },
                    ScalarType::Boolean => quote! { bool },
                    ScalarType::DateTime => quote! { ::saola_core::chrono::DateTime<::saola_core::chrono::Utc> },
                    ScalarType::Json => quote! { ::saola_core::serde_json::Value },
                    ScalarType::Decimal => quote! { ::saola_core::bigdecimal::BigDecimal },
                    ScalarType::BigInt => quote! { i64 },
                    ScalarType::Bytes => quote! { Vec<u8> },
                },
                ScalarFieldType::Enum(enum_id) => {
                    let enum_name = format_ident!("{}", _db.walk(enum_id).name());
                    quote! { #enum_name }
                }
                _ => quote! { ::saola_core::serde_json::Value },
            };

            let final_type = if field.is_optional() {
                quote! { Option<#field_type> }
            } else {
                field_type
            };

            scalar_fields.push(quote! {
                #[serde(rename = #prisma_name)]
                pub #rust_name: #final_type,
            });
        }

        let mut relation_fields = Vec::new();
        let mut type_params = Vec::new();

        for rf in &sorted_subset {
            let relation_name = rf.name();
            let rust_relation_name = format_ident!("{}", relation_name.to_snake_case());
            let type_param = format_ident!("T{}", pascal_case(relation_name));

            type_params.push(quote! { #type_param });

            let related_type = if rf.ast_field().arity.is_list() {
                quote! { Vec<#type_param> }
            } else if rf.ast_field().arity.is_required() {
                quote! { #type_param }
            } else {
                quote! { Option<#type_param> }
            };

            relation_fields.push(quote! {
                #[serde(rename = #relation_name)]
                pub #rust_relation_name: #related_type,
            });
        }

        let generics_decl = if type_params.is_empty() {
            quote! {}
        } else {
            quote! { <#(#type_params),*> }
        };

        types.push(quote! {
            #[derive(Debug, Clone, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize)]
            #[serde(crate = "::saola_core::serde")]
            pub struct #type_name #generics_decl {
                #(#scalar_fields)*
                #(#relation_fields)*
            }
        });
    }

    quote! {
        #(#types)*
    }
}

pub fn generate_aggregation_structs(
    _db: &ParserDatabase,
    walker: psl::parser_database::walkers::ModelWalker<'_>,
) -> proc_macro2::TokenStream {
    let model_name_str = walker.name();
    let model_name = format_ident!("{}", model_name_str);

    let mut count_fields = Vec::new();
    let mut sum_fields = Vec::new();
    let mut avg_fields = Vec::new();
    let mut min_fields = Vec::new();
    let mut max_fields = Vec::new();

    count_fields.push(quote! {
        #[serde(rename = "_all")]
        pub _all: Option<i32>
    });

    for field in walker.scalar_fields() {
        let prisma_name = field.name();
        let rust_name = format_ident!("{}", prisma_name.to_snake_case());
        let field_type = field.scalar_field_type();

        count_fields.push(quote! {
            #[serde(rename = #prisma_name)]
            pub #rust_name: Option<i32>
        });

        let is_numeric = match field_type {
            ScalarFieldType::BuiltInScalar(builtin) => match builtin {
                ScalarType::Int | ScalarType::Float | ScalarType::Decimal | ScalarType::BigInt => true,
                _ => false,
            },
            _ => false,
        };

        let rust_type = match field_type {
            ScalarFieldType::BuiltInScalar(builtin) => match builtin {
                ScalarType::Int => quote! { i32 },
                ScalarType::Float => quote! { f64 },
                ScalarType::Decimal => quote! { ::saola_core::bigdecimal::BigDecimal },
                ScalarType::BigInt => quote! { i64 },
                _ => quote! { String },
            },
            _ => quote! { String },
        };

        if is_numeric {
            sum_fields.push(quote! {
                #[serde(rename = #prisma_name)]
                pub #rust_name: Option<#rust_type>
            });
            avg_fields.push(quote! {
                #[serde(rename = #prisma_name)]
                pub #rust_name: Option<f64>
            });
        }

        // Min/Max for most scalars
        let can_min_max = match field_type {
            ScalarFieldType::BuiltInScalar(builtin) => match builtin {
                ScalarType::Json | ScalarType::Bytes => false,
                _ => true,
            },
            ScalarFieldType::Enum(_) => true,
            _ => false,
        };

        if can_min_max {
            let min_max_type = match field_type {
                ScalarFieldType::BuiltInScalar(builtin) => match builtin {
                    ScalarType::String => quote! { String },
                    ScalarType::Int => quote! { i32 },
                    ScalarType::Float => quote! { f64 },
                    ScalarType::Boolean => quote! { bool },
                    ScalarType::DateTime => quote! { ::saola_core::chrono::DateTime<::saola_core::chrono::Utc> },
                    ScalarType::Decimal => quote! { ::saola_core::bigdecimal::BigDecimal },
                    ScalarType::BigInt => quote! { i64 },
                    _ => quote! { ::saola_core::serde_json::Value },
                },
                ScalarFieldType::Enum(enum_id) => {
                    let enum_name = format_ident!("{}", _db.walk(enum_id).name());
                    quote! { #enum_name }
                }
                _ => quote! { ::saola_core::serde_json::Value },
            };

            min_fields.push(quote! {
                #[serde(rename = #prisma_name)]
                pub #rust_name: Option<#min_max_type>
            });
            max_fields.push(quote! {
                #[serde(rename = #prisma_name)]
                pub #rust_name: Option<#min_max_type>
            });
        }
    }

    let aggregate_result = format_ident!("{}AggregateResult", model_name);
    let count_struct = format_ident!("{}CountAggregate", model_name);
    let sum_struct = format_ident!("{}SumAggregate", model_name);
    let avg_struct = format_ident!("{}AvgAggregate", model_name);
    let min_struct = format_ident!("{}MinAggregate", model_name);
    let max_struct = format_ident!("{}MaxAggregate", model_name);

    quote! {
        #[derive(Debug, Clone, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde", default)]
        pub struct #aggregate_result {
            #[serde(rename = "_count")]
            pub _count: Option<#count_struct>,
            #[serde(rename = "_sum")]
            pub _sum: Option<#sum_struct>,
            #[serde(rename = "_avg")]
            pub _avg: Option<#avg_struct>,
            #[serde(rename = "_min")]
            pub _min: Option<#min_struct>,
            #[serde(rename = "_max")]
            pub _max: Option<#max_struct>,
        }

        #[derive(Debug, Clone, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde", default)]
        pub struct #count_struct {
            #(#count_fields),*
        }

        #[derive(Debug, Clone, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde", default)]
        pub struct #sum_struct {
            #(#sum_fields),*
        }

        #[derive(Debug, Clone, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde", default)]
        pub struct #avg_struct {
            #(#avg_fields),*
        }

        #[derive(Debug, Clone, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde", default)]
        pub struct #min_struct {
            #(#min_fields),*
        }

        #[derive(Debug, Clone, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde", default)]
        pub struct #max_struct {
            #(#max_fields),*
        }
    }
}

pub fn generate_group_by_struct(
    _db: &ParserDatabase,
    walker: psl::parser_database::walkers::ModelWalker<'_>,
) -> proc_macro2::TokenStream {
    let model_name_str = walker.name();
    let model_name = format_ident!("{}", model_name_str);
    let group_by_result = format_ident!("{}GroupByResult", model_name);

    let mut fields = Vec::new();

    for field in walker.scalar_fields() {
        let prisma_name = field.name();
        let rust_name = format_ident!("{}", prisma_name.to_snake_case());
        let field_type = field.scalar_field_type();

        let rust_type = match field_type {
            ScalarFieldType::BuiltInScalar(builtin) => match builtin {
                ScalarType::String => quote! { String },
                ScalarType::Int => quote! { i32 },
                ScalarType::Float => quote! { f64 },
                ScalarType::Boolean => quote! { bool },
                ScalarType::DateTime => quote! { ::saola_core::chrono::DateTime<::saola_core::chrono::Utc> },
                ScalarType::Decimal => quote! { ::saola_core::bigdecimal::BigDecimal },
                ScalarType::BigInt => quote! { i64 },
                ScalarType::Bytes => quote! { Vec<u8> },
                ScalarType::Json => quote! { ::saola_core::serde_json::Value },
            },
            ScalarFieldType::Enum(enum_id) => {
                let enum_name = format_ident!("{}", _db.walk(enum_id).name());
                quote! { #enum_name }
            }
            _ => quote! { ::saola_core::serde_json::Value },
        };

        fields.push(quote! {
            #[serde(rename = #prisma_name)]
            pub #rust_name: Option<#rust_type>
        });
    }

    let count_struct = format_ident!("{}CountAggregate", model_name);
    let sum_struct = format_ident!("{}SumAggregate", model_name);
    let avg_struct = format_ident!("{}AvgAggregate", model_name);
    let min_struct = format_ident!("{}MinAggregate", model_name);
    let max_struct = format_ident!("{}MaxAggregate", model_name);

    quote! {
        #[derive(Debug, Clone, ::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde", default)]
        pub struct #group_by_result {
            #(#fields),*,

            #[serde(rename = "_count")]
            pub _count: Option<#count_struct>,
            #[serde(rename = "_sum")]
            pub _sum: Option<#sum_struct>,
            #[serde(rename = "_avg")]
            pub _avg: Option<#avg_struct>,
            #[serde(rename = "_min")]
            pub _min: Option<#min_struct>,
            #[serde(rename = "_max")]
            pub _max: Option<#max_struct>,
        }
    }
}

fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
