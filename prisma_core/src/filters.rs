/// Type-safe filter operators per field type
/// This module provides trait-based operator constraints to catch invalid operations at compile time

use query_core::ArgumentValue;
use crate::IndexMap;
use crate::query_structure::PrismaValue;

/// Base filter trait - all filters implement this
pub trait FilterOp: Sized {
    fn add_op(&mut self, op: &str, value: ArgumentValue);
}

/// Operators available on String fields
pub trait StringFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn contains<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn starts_with<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn ends_with<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
}

/// Operators available on Int fields
pub trait IntFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
}

/// Operators available on Boolean fields
pub trait BoolFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
}

/// Operators available on Enum fields
pub trait EnumFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
}

/// Operators available on Float/Decimal fields
pub trait FloatFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
}

/// Operators available on DateTime fields
pub trait DateTimeFieldOps {
    fn eq<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn not<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn gte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lt<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn lte<T: Into<PrismaValue>>(self, value: T) -> Self;
    fn in_list<T: Into<PrismaValue>>(self, values: Vec<T>) -> Self;
}

/// Operators available on Relations - some/every/none for filtering related records
pub trait RelationFilterOps {
    fn some(self) -> Self;
    fn every(self) -> Self;
    fn none(self) -> Self;
    fn is(self) -> Self;
    fn is_not(self) -> Self;
}

// ============ HELPER FUNCTIONS ============

/// Helper to create a filter argument with operator
pub fn create_filter_arg<T: Into<PrismaValue>>(
    field_name: &str,
    op: &str,
    value: T,
) -> (String, ArgumentValue) {
    let mut map = IndexMap::new();
    map.insert(op.to_string(), ArgumentValue::Scalar(value.into()));
    (field_name.to_string(), ArgumentValue::Object(map))
}

/// Helper to create a list filter argument
pub fn create_list_filter_arg<T: Into<PrismaValue>>(
    field_name: &str,
    op: &str,
    values: Vec<T>,
) -> (String, ArgumentValue) {
    let list: Vec<ArgumentValue> = values
        .into_iter()
        .map(|v| ArgumentValue::Scalar(v.into()))
        .collect();

    let mut map = IndexMap::new();
    map.insert(op.to_string(), ArgumentValue::List(list));
    (field_name.to_string(), ArgumentValue::Object(map))
}
