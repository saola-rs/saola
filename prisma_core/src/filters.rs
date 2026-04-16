use crate::IndexMap;
use crate::query_structure::PrismaValue;
/// Type-safe filter operators per field type
/// This module provides trait-based operator constraints to catch invalid operations at compile time
use query_core::ArgumentValue;

pub trait StringFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn contains<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn starts_with<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn ends_with<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
    fn not_in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
    fn gt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lte<T: Into<PrismaValue>>(self, value: T) -> Self;
}

pub trait IntFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
    fn not_in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
}

pub trait FloatFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
    fn not_in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
}

pub trait BoolFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
}

pub trait EnumFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
    fn not_in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
}

pub trait DateTimeFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
    fn not_in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
}

pub trait RelationFilterOps {
    fn is<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut dyn crate::builder::FilterBuilder);
    fn is_not<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut dyn crate::builder::FilterBuilder);
}

/// Helper to build nested filter maps (e.g., { equals: value })
pub fn build_filter_map(op: &str, value: impl Into<PrismaValue>) -> (String, ArgumentValue) {
    let mut map = IndexMap::new();
    map.insert(op.to_string(), ArgumentValue::Scalar(value.into()));
    (op.to_string(), ArgumentValue::Object(map))
}

pub fn build_list_filter(field_name: &str, op: &str, values: Vec<impl Into<PrismaValue>>) -> (String, ArgumentValue) {
    let list: Vec<ArgumentValue> = values.into_iter().map(|v| ArgumentValue::Scalar(v.into())).collect();

    let mut map = IndexMap::new();
    map.insert(op.to_string(), ArgumentValue::List(list));
    (field_name.to_string(), ArgumentValue::Object(map))
}
