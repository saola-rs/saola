use crate::IndexMap;
use crate::query_structure::PrismaValue;
/// Type-safe filter operators per field type
/// This module provides trait-based operator constraints to catch invalid operations at compile time
use query_core::ArgumentValue;
use std::marker::PhantomData;

/// Marker for a field in a model
#[derive(Debug, Clone, Copy)]
pub struct Field<T>(pub &'static str, pub PhantomData<T>);

impl<T> Field<T> {
    pub const fn new(name: &'static str) -> Self {
        Self(name, PhantomData)
    }

    pub fn select<'a, B: crate::builder::Selectable>(
        self,
        builder: &'a mut B,
    ) -> crate::builder::SelectionField<'a, T, B> {
        builder.add_nested_selection(query_core::Selection::with_name(self.0));
        crate::builder::SelectionField::new(builder)
    }
}

/// A constructed where clause
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub map: IndexMap<String, ArgumentValue>,
}

impl WhereClause {
    pub fn empty() -> Self {
        Self { map: IndexMap::new() }
    }

    pub fn new(field: &str, op: &str, value: ArgumentValue) -> Self {
        let mut inner = IndexMap::new();
        inner.insert(op.to_string(), value);

        let mut map = IndexMap::new();
        map.insert(field.to_string(), ArgumentValue::Object(inner));
        Self { map }
    }

    pub fn and(self, other: WhereClause) -> Self {
        if self.map.is_empty() {
            return other;
        }
        if other.map.is_empty() {
            return self;
        }

        let mut and_list = Vec::new();
        and_list.push(ArgumentValue::Object(self.map));
        and_list.push(ArgumentValue::Object(other.map));

        let mut new_map = IndexMap::new();
        new_map.insert("AND".to_string(), ArgumentValue::List(and_list));
        Self { map: new_map }
    }

    pub fn or(self, other: WhereClause) -> Self {
        if self.map.is_empty() {
            return other;
        }
        if other.map.is_empty() {
            return self;
        }

        let mut or_list = Vec::new();
        or_list.push(ArgumentValue::Object(self.map));
        or_list.push(ArgumentValue::Object(other.map));

        let mut new_map = IndexMap::new();
        new_map.insert("OR".to_string(), ArgumentValue::List(or_list));
        Self { map: new_map }
    }
}

// New Trait-based API for Fields and Builders
pub trait StringField<R> {
    fn equals(self, val: impl Into<String>) -> R;
    fn contains(self, val: impl Into<String>) -> R;
    fn starts_with(self, val: impl Into<String>) -> R;
    fn ends_with(self, val: impl Into<String>) -> R;
}

impl StringField<WhereClause> for Field<String> {
    fn equals(self, val: impl Into<String>) -> WhereClause {
        WhereClause::new(self.0, "equals", ArgumentValue::Scalar(PrismaValue::String(val.into())))
    }
    fn contains(self, val: impl Into<String>) -> WhereClause {
        WhereClause::new(
            self.0,
            "contains",
            ArgumentValue::Scalar(PrismaValue::String(val.into())),
        )
    }
    fn starts_with(self, val: impl Into<String>) -> WhereClause {
        WhereClause::new(
            self.0,
            "startsWith",
            ArgumentValue::Scalar(PrismaValue::String(val.into())),
        )
    }
    fn ends_with(self, val: impl Into<String>) -> WhereClause {
        WhereClause::new(
            self.0,
            "endsWith",
            ArgumentValue::Scalar(PrismaValue::String(val.into())),
        )
    }
}

pub trait IntField<R> {
    fn equals(self, val: i32) -> R;
    fn gt(self, val: i32) -> R;
    fn gte(self, val: i32) -> R;
    fn lt(self, val: i32) -> R;
    fn lte(self, val: i32) -> R;
}

impl IntField<WhereClause> for Field<i32> {
    fn equals(self, val: i32) -> WhereClause {
        WhereClause::new(self.0, "equals", ArgumentValue::Scalar(PrismaValue::Int(val as i64)))
    }
    fn gt(self, val: i32) -> WhereClause {
        WhereClause::new(self.0, "gt", ArgumentValue::Scalar(PrismaValue::Int(val as i64)))
    }
    fn gte(self, val: i32) -> WhereClause {
        WhereClause::new(self.0, "gte", ArgumentValue::Scalar(PrismaValue::Int(val as i64)))
    }
    fn lt(self, val: i32) -> WhereClause {
        WhereClause::new(self.0, "lt", ArgumentValue::Scalar(PrismaValue::Int(val as i64)))
    }
    fn lte(self, val: i32) -> WhereClause {
        WhereClause::new(self.0, "lte", ArgumentValue::Scalar(PrismaValue::Int(val as i64)))
    }
}

pub trait FloatField<R> {
    fn equals(self, val: f64) -> R;
    fn gt(self, val: f64) -> R;
    fn gte(self, val: f64) -> R;
    fn lt(self, val: f64) -> R;
    fn lte(self, val: f64) -> R;
}

impl FloatField<WhereClause> for Field<f64> {
    fn equals(self, val: f64) -> WhereClause {
        use bigdecimal::FromPrimitive;
        WhereClause::new(
            self.0,
            "equals",
            ArgumentValue::Scalar(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(val).unwrap())),
        )
    }
    fn gt(self, val: f64) -> WhereClause {
        use bigdecimal::FromPrimitive;
        WhereClause::new(
            self.0,
            "gt",
            ArgumentValue::Scalar(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(val).unwrap())),
        )
    }
    fn gte(self, val: f64) -> WhereClause {
        use bigdecimal::FromPrimitive;
        WhereClause::new(
            self.0,
            "gte",
            ArgumentValue::Scalar(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(val).unwrap())),
        )
    }
    fn lt(self, val: f64) -> WhereClause {
        use bigdecimal::FromPrimitive;
        WhereClause::new(
            self.0,
            "lt",
            ArgumentValue::Scalar(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(val).unwrap())),
        )
    }
    fn lte(self, val: f64) -> WhereClause {
        use bigdecimal::FromPrimitive;
        WhereClause::new(
            self.0,
            "lte",
            ArgumentValue::Scalar(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(val).unwrap())),
        )
    }
}

pub trait BoolField<R> {
    fn equals(self, val: bool) -> R;
}

impl BoolField<WhereClause> for Field<bool> {
    fn equals(self, val: bool) -> WhereClause {
        WhereClause::new(self.0, "equals", ArgumentValue::Scalar(PrismaValue::Boolean(val)))
    }
}

pub trait DateTimeField<R> {
    fn equals(self, val: chrono::DateTime<chrono::Utc>) -> R;
    fn gt(self, val: chrono::DateTime<chrono::Utc>) -> R;
    fn gte(self, val: chrono::DateTime<chrono::Utc>) -> R;
    fn lt(self, val: chrono::DateTime<chrono::Utc>) -> R;
    fn lte(self, val: chrono::DateTime<chrono::Utc>) -> R;
}

impl DateTimeField<WhereClause> for Field<chrono::DateTime<chrono::Utc>> {
    fn equals(self, val: chrono::DateTime<chrono::Utc>) -> WhereClause {
        WhereClause::new(
            self.0,
            "equals",
            ArgumentValue::Scalar(PrismaValue::DateTime(val.into())),
        )
    }
    fn gt(self, val: chrono::DateTime<chrono::Utc>) -> WhereClause {
        WhereClause::new(self.0, "gt", ArgumentValue::Scalar(PrismaValue::DateTime(val.into())))
    }
    fn gte(self, val: chrono::DateTime<chrono::Utc>) -> WhereClause {
        WhereClause::new(self.0, "gte", ArgumentValue::Scalar(PrismaValue::DateTime(val.into())))
    }
    fn lt(self, val: chrono::DateTime<chrono::Utc>) -> WhereClause {
        WhereClause::new(self.0, "lt", ArgumentValue::Scalar(PrismaValue::DateTime(val.into())))
    }
    fn lte(self, val: chrono::DateTime<chrono::Utc>) -> WhereClause {
        WhereClause::new(self.0, "lte", ArgumentValue::Scalar(PrismaValue::DateTime(val.into())))
    }
}

pub trait EnumField<E, R> {
    fn enum_equals(self, val: E) -> R;
}

// Remove blanket impl to avoid ambiguity with StringField::equals
/*
impl<E: Into<PrismaValue>> EnumField<E, WhereClause> for Field<E> {
    fn equals(self, val: E) -> WhereClause {
        WhereClause::new(self.0, "equals", ArgumentValue::Scalar(val.into()))
    }
}
*/

pub trait SelectField {
    fn into_selection(self) -> query_core::Selection;
}

impl<T> SelectField for Field<T> {
    fn into_selection(self) -> query_core::Selection {
        query_core::Selection::with_name(self.0)
    }
}

pub trait RelationFilterOps {
    fn is<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut dyn crate::builder::FilterBuilder);
    fn is_not<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut dyn crate::builder::FilterBuilder);
}
