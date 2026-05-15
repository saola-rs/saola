/// Concrete typed filter builders that implement the unified field traits
use crate::{
    FilterBuilder, IndexMap,
    filters::{BoolField, DateTimeField, EnumField, FloatField, IntField, StringField},
    query_structure::PrismaValue,
};
use query_core::ArgumentValue;

pub struct FieldFilter<'a, B, T> {
    pub builder: &'a mut B,
    pub field_name: &'static str,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<'a, B: FilterBuilder, T> FieldFilter<'a, B, T> {
    fn add_op(self, op: &str, value: ArgumentValue) {
        let mut map = IndexMap::new();
        map.insert(op.to_string(), value);
        self.builder
            .add_arg(self.field_name.to_string(), ArgumentValue::Object(map));
    }
}

impl<'a, B: FilterBuilder> StringField<()> for FieldFilter<'a, B, String> {
    fn equals(self, val: impl Into<String>) -> () {
        self.add_op("equals", ArgumentValue::Scalar(PrismaValue::String(val.into())));
    }
    fn contains(self, val: impl Into<String>) -> () {
        self.add_op("contains", ArgumentValue::Scalar(PrismaValue::String(val.into())));
    }
    fn starts_with(self, val: impl Into<String>) -> () {
        self.add_op("startsWith", ArgumentValue::Scalar(PrismaValue::String(val.into())));
    }
    fn ends_with(self, val: impl Into<String>) -> () {
        self.add_op("endsWith", ArgumentValue::Scalar(PrismaValue::String(val.into())));
    }
}

impl<'a, B: FilterBuilder> IntField<()> for FieldFilter<'a, B, i32> {
    fn equals(self, val: i32) -> () {
        self.add_op("equals", ArgumentValue::Scalar(PrismaValue::Int(val as i64)));
    }
    fn gt(self, val: i32) -> () {
        self.add_op("gt", ArgumentValue::Scalar(PrismaValue::Int(val as i64)));
    }
    fn gte(self, val: i32) -> () {
        self.add_op("gte", ArgumentValue::Scalar(PrismaValue::Int(val as i64)));
    }
    fn lt(self, val: i32) -> () {
        self.add_op("lt", ArgumentValue::Scalar(PrismaValue::Int(val as i64)));
    }
    fn lte(self, val: i32) -> () {
        self.add_op("lte", ArgumentValue::Scalar(PrismaValue::Int(val as i64)));
    }
}

impl<'a, B: FilterBuilder> FloatField<()> for FieldFilter<'a, B, f64> {
    fn equals(self, val: f64) -> () {
        use bigdecimal::FromPrimitive;
        self.add_op(
            "equals",
            ArgumentValue::Scalar(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(val).unwrap())),
        );
    }
    fn gt(self, val: f64) -> () {
        use bigdecimal::FromPrimitive;
        self.add_op(
            "gt",
            ArgumentValue::Scalar(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(val).unwrap())),
        );
    }
    fn gte(self, val: f64) -> () {
        use bigdecimal::FromPrimitive;
        self.add_op(
            "gte",
            ArgumentValue::Scalar(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(val).unwrap())),
        );
    }
    fn lt(self, val: f64) -> () {
        use bigdecimal::FromPrimitive;
        self.add_op(
            "lt",
            ArgumentValue::Scalar(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(val).unwrap())),
        );
    }
    fn lte(self, val: f64) -> () {
        use bigdecimal::FromPrimitive;
        self.add_op(
            "lte",
            ArgumentValue::Scalar(PrismaValue::Float(bigdecimal::BigDecimal::from_f64(val).unwrap())),
        );
    }
}

impl<'a, B: FilterBuilder> BoolField<()> for FieldFilter<'a, B, bool> {
    fn equals(self, val: bool) -> () {
        self.add_op("equals", ArgumentValue::Scalar(PrismaValue::Boolean(val)));
    }
}

impl<'a, B: FilterBuilder> DateTimeField<()> for FieldFilter<'a, B, chrono::DateTime<chrono::Utc>> {
    fn equals(self, val: chrono::DateTime<chrono::Utc>) -> () {
        self.add_op("equals", ArgumentValue::Scalar(PrismaValue::DateTime(val.into())));
    }
    fn gt(self, val: chrono::DateTime<chrono::Utc>) -> () {
        self.add_op("gt", ArgumentValue::Scalar(PrismaValue::DateTime(val.into())));
    }
    fn gte(self, val: chrono::DateTime<chrono::Utc>) -> () {
        self.add_op("gte", ArgumentValue::Scalar(PrismaValue::DateTime(val.into())));
    }
    fn lt(self, val: chrono::DateTime<chrono::Utc>) -> () {
        self.add_op("lt", ArgumentValue::Scalar(PrismaValue::DateTime(val.into())));
    }
    fn lte(self, val: chrono::DateTime<chrono::Utc>) -> () {
        self.add_op("lte", ArgumentValue::Scalar(PrismaValue::DateTime(val.into())));
    }
}

impl<'a, B: FilterBuilder, E: Into<PrismaValue>> EnumField<E, ()> for FieldFilter<'a, B, E> {
    fn enum_equals(self, val: E) -> () {
        self.add_op("equals", ArgumentValue::Scalar(val.into()));
    }
}

pub struct RelationFilter<'a, B, WB> {
    pub builder: &'a mut B,
    pub field_name: &'static str,
    pub _phantom: std::marker::PhantomData<WB>,
}

impl<'a, B: FilterBuilder, WB: Default + FilterBuilder> RelationFilter<'a, B, WB> {
    pub fn some<F>(self, f: F)
    where
        F: FnOnce(&mut WB),
    {
        let mut nested = WB::default();
        f(&mut nested);
        self.add_nested_op("some", nested);
    }

    pub fn every<F>(self, f: F)
    where
        F: FnOnce(&mut WB),
    {
        let mut nested = WB::default();
        f(&mut nested);
        self.add_nested_op("every", nested);
    }

    pub fn none<F>(self, f: F)
    where
        F: FnOnce(&mut WB),
    {
        let mut nested = WB::default();
        f(&mut nested);
        self.add_nested_op("none", nested);
    }

    pub fn is<F>(self, f: F)
    where
        F: FnOnce(&mut WB),
    {
        let mut nested = WB::default();
        f(&mut nested);
        self.add_nested_op("is", nested);
    }

    pub fn is_not<F>(self, f: F)
    where
        F: FnOnce(&mut WB),
    {
        let mut nested = WB::default();
        f(&mut nested);
        self.add_nested_op("isNot", nested);
    }

    fn add_nested_op(self, op: &str, nested: WB) {
        let args = nested.build();
        if !args.is_empty() {
            let mut map = IndexMap::new();
            map.insert(op.to_string(), ArgumentValue::Object(args));
            self.builder
                .add_arg(self.field_name.to_string(), ArgumentValue::Object(map));
        }
    }
}

// Re-export specific filters for backward compatibility with generated code (but internally they use FieldFilter)
pub type StringFilter<'a, B> = FieldFilter<'a, B, String>;
pub type IntFilter<'a, B> = FieldFilter<'a, B, i32>;
pub type BoolFilter<'a, B> = FieldFilter<'a, B, bool>;
pub type FloatFilter<'a, B> = FieldFilter<'a, B, f64>;
pub type DateTimeFilter<'a, B> = FieldFilter<'a, B, chrono::DateTime<chrono::Utc>>;
pub type EnumFilter<'a, B, E> = FieldFilter<'a, B, E>;
