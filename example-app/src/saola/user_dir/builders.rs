use super::super::*;
use ::saola_core::prelude::*;
pub struct UserMarker;
impl ::saola_core::ModelMarker for UserMarker {
    type Data = UserData;
    type Where = UserWhereBuilder;
    type UniqueWhere = UserUniqueWhereBuilder;
    type OrderBy = UserOrderByBuilder;
    type Include = UserIncludeBuilder;
    type Select = UserSelectBuilder;
    type DataBuilder = UserDataBuilder;
    const NAME: &'static str = "User";
    const SCALAR_FIELDS: &'static [&'static str] = &["id", "email", "name", "isActive", "score", "level", "createdAt"];
}
#[derive(Default)]
pub struct UserWhereBuilder {
    pub args: Vec<(String, ::saola_core::query_core::ArgumentValue)>,
}
impl ::saola_core::FilterBuilder for UserWhereBuilder {
    fn add_arg(&mut self, name: String, value: ::saola_core::query_core::ArgumentValue) {
        self.args.push((name, value));
    }
    fn build(mut self) -> ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue> {
        let mut map = ::saola_core::IndexMap::new();
        for (k, v) in std::mem::take(&mut self.args) {
            map.insert(k, v);
        }
        map
    }
}
impl UserWhereBuilder {
    pub fn and<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        let mut builder = Self::default();
        f(&mut builder);
        if !builder.args.is_empty() {
            let mut map = ::saola_core::IndexMap::new();
            for (k, v) in std::mem::take(&mut builder.args) {
                map.insert(k, v);
            }
            self.args
                .push(("AND".to_string(), ::saola_core::query_core::ArgumentValue::Object(map)));
        }
        self
    }
    pub fn or<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        let mut builder = Self::default();
        f(&mut builder);
        if !builder.args.is_empty() {
            let mut vec = Vec::new();
            for (k, v) in std::mem::take(&mut builder.args) {
                let mut map = ::saola_core::IndexMap::new();
                map.insert(k, v);
                vec.push(::saola_core::query_core::ArgumentValue::Object(map));
            }
            self.args
                .push(("OR".to_string(), ::saola_core::query_core::ArgumentValue::List(vec)));
        }
        self
    }
    pub fn not<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        let mut builder = Self::default();
        f(&mut builder);
        if !builder.args.is_empty() {
            let mut map = ::saola_core::IndexMap::new();
            for (k, v) in std::mem::take(&mut builder.args) {
                map.insert(k, v);
            }
            self.args
                .push(("NOT".to_string(), ::saola_core::query_core::ArgumentValue::Object(map)));
        }
        self
    }
    pub fn id(&mut self) -> ::saola_core::StringFilter<'_, Self> {
        ::saola_core::StringFilter {
            builder: self,
            field_name: "id",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn email(&mut self) -> ::saola_core::StringFilter<'_, Self> {
        ::saola_core::StringFilter {
            builder: self,
            field_name: "email",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn name(&mut self) -> ::saola_core::StringFilter<'_, Self> {
        ::saola_core::StringFilter {
            builder: self,
            field_name: "name",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn is_active(&mut self) -> ::saola_core::BoolFilter<'_, Self> {
        ::saola_core::BoolFilter {
            builder: self,
            field_name: "isActive",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn score(&mut self) -> ::saola_core::FloatFilter<'_, Self> {
        ::saola_core::FloatFilter {
            builder: self,
            field_name: "score",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn level(&mut self) -> ::saola_core::IntFilter<'_, Self> {
        ::saola_core::IntFilter {
            builder: self,
            field_name: "level",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn created_at(&mut self) -> ::saola_core::DateTimeFilter<'_, Self> {
        ::saola_core::DateTimeFilter {
            builder: self,
            field_name: "createdAt",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn posts(&mut self) -> ::saola_core::RelationFilter<'_, Self, PostWhereBuilder> {
        ::saola_core::RelationFilter {
            builder: self,
            field_name: "posts",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn profile(&mut self) -> ::saola_core::RelationFilter<'_, Self, ProfileWhereBuilder> {
        ::saola_core::RelationFilter {
            builder: self,
            field_name: "profile",
            _phantom: std::marker::PhantomData,
        }
    }
}
#[derive(Default)]
pub struct UserUniqueWhereBuilder {
    pub args: Vec<(String, ::saola_core::query_core::ArgumentValue)>,
}
impl ::saola_core::FilterBuilder for UserUniqueWhereBuilder {
    fn add_arg(&mut self, name: String, value: ::saola_core::query_core::ArgumentValue) {
        self.args.push((name, value));
    }
    fn build(mut self) -> ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue> {
        let mut map = ::saola_core::IndexMap::new();
        for (k, v) in std::mem::take(&mut self.args) {
            map.insert(k, v);
        }
        map
    }
}
impl UserUniqueWhereBuilder {
    pub fn id<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        use ::saola_core::FilterBuilder;
        self.add_arg(
            "id".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn email<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        use ::saola_core::FilterBuilder;
        self.add_arg(
            "email".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
}
#[derive(Default)]
pub struct UserOrderByBuilder {
    pub args: Vec<::saola_core::ArgumentValue>,
}
impl ::saola_core::builder::OrderByBuilderTrait for UserOrderByBuilder {
    fn build(self) -> Vec<::saola_core::ArgumentValue> {
        self.args
    }
}
impl UserOrderByBuilder {
    pub fn id(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("id".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn email(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("email".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn name(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("name".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn is_active(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("isActive".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn score(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("score".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn level(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("level".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn created_at(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("createdAt".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
}
#[derive(Default)]
pub struct UserSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl ::saola_core::Selectable for UserSelectBuilder {
    fn add_nested_selection(&mut self, selection: ::saola_core::query_core::Selection) {
        self.selections.push(selection);
    }
    fn into_selections(mut self) -> Vec<::saola_core::query_core::Selection> {
        if self.selections.is_empty() {
            self.all();
        }
        self.selections
    }
}
impl UserSelectBuilder {
    pub fn all(&mut self) -> &mut Self {
        for field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
            self.selections
                .push(::saola_core::query_core::Selection::with_name(field.to_string()));
        }
        self
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_id(&self) {}
    pub fn id(&mut self) -> ::saola_core::SelectionField<'_, String, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("id".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_email(&self) {}
    pub fn email(&mut self) -> ::saola_core::SelectionField<'_, String, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("email".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_name(&self) {}
    pub fn name(&mut self) -> ::saola_core::SelectionField<'_, String, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("name".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_isActive(&self) {}
    pub fn is_active(&mut self) -> ::saola_core::SelectionField<'_, bool, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("isActive".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_score(&self) {}
    pub fn score(&mut self) -> ::saola_core::SelectionField<'_, f64, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("score".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_level(&self) {}
    pub fn level(&mut self) -> ::saola_core::SelectionField<'_, i32, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("level".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_createdAt(&self) {}
    pub fn created_at(
        &mut self,
    ) -> ::saola_core::SelectionField<'_, ::saola_core::chrono::DateTime<::saola_core::chrono::Utc>, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("createdAt".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_posts(&self) {}
    pub fn posts<F>(&mut self, f: F) -> ::saola_core::SelectionRelField<'_, Vec<()>, Self>
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut builder = PostSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into_selections();
        let mut sel = ::saola_core::query_core::Selection::with_name("posts".to_string());
        for s in selections {
            sel.push_nested_selection(s);
        }
        self.selections.push(sel);
        ::saola_core::SelectionRelField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_profile(&self) {}
    pub fn profile<F>(&mut self, f: F) -> ::saola_core::SelectionRelField<'_, Option<()>, Self>
    where
        F: FnOnce(&mut ProfileSelectBuilder),
    {
        let mut builder = ProfileSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into_selections();
        let mut sel = ::saola_core::query_core::Selection::with_name("profile".to_string());
        for s in selections {
            sel.push_nested_selection(s);
        }
        self.selections.push(sel);
        ::saola_core::SelectionRelField::new(self)
    }
}
impl From<UserSelectBuilder> for Vec<::saola_core::query_core::Selection> {
    fn from(b: UserSelectBuilder) -> Self {
        b.selections
    }
}
#[derive(Default)]
pub struct UserDataBuilder {
    pub data: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl ::saola_core::builder::DataBuilderTrait for UserDataBuilder {
    fn build(self) -> ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue> {
        self.data
    }
}
impl UserDataBuilder {
    pub fn id<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "id".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn email<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "email".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn name<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "name".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn is_active<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "isActive".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn score<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "score".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn score_increment<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "increment".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "score".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn score_decrement<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "decrement".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "score".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn score_multiply<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "multiply".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "score".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn score_divide<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "divide".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "score".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn level<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "level".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn level_increment<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "increment".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "level".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn level_decrement<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "decrement".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "level".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn level_multiply<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "multiply".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "level".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn level_divide<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "divide".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "level".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn created_at<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "createdAt".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn posts<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut UserPostsRelationWriteBuilder),
    {
        let mut builder = UserPostsRelationWriteBuilder::default();
        f(&mut builder);
        if !builder.data.is_empty() {
            self.data.insert(
                "posts".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
            );
        }
        self
    }
    pub fn profile<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut UserProfileRelationWriteBuilder),
    {
        let mut builder = UserProfileRelationWriteBuilder::default();
        f(&mut builder);
        if !builder.data.is_empty() {
            self.data.insert(
                "profile".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
            );
        }
        self
    }
}
impl From<UserDataBuilder> for ::saola_core::query_structure::PrismaValue {
    fn from(_b: UserDataBuilder) -> Self {
        ::saola_core::query_structure::PrismaValue::Null
    }
}
#[derive(Default)]
pub struct UserScalarDataBuilder {
    pub data: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl ::saola_core::builder::DataBuilderTrait for UserScalarDataBuilder {
    fn build(self) -> ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue> {
        self.data
    }
}
impl UserScalarDataBuilder {
    pub fn id<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "id".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn email<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "email".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn name<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "name".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn is_active<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "isActive".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn score<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "score".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn score_increment<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "increment".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "score".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn score_decrement<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "decrement".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "score".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn score_multiply<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "multiply".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "score".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn score_divide<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "divide".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "score".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn level<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "level".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn level_increment<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "increment".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "level".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn level_decrement<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "decrement".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "level".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn level_multiply<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "multiply".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "level".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn level_divide<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "divide".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self.data.insert(
            "level".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn created_at<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "createdAt".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
}
#[derive(Default)]
pub struct UserPostsRelationWriteBuilder {
    pub data: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl UserPostsRelationWriteBuilder {
    pub fn create<F>(&mut self, title: String, f: F) -> &mut Self
    where
        F: FnOnce(&mut PostDataBuilder),
    {
        let mut builder = PostDataBuilder::default();
        builder.data.insert(
            "title".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(title)),
        );
        f(&mut builder);
        let val = ::saola_core::query_core::ArgumentValue::Object(builder.data);
        let list = self
            .data
            .entry("create".to_string())
            .or_insert_with(|| ::saola_core::query_core::ArgumentValue::List(Vec::new()));
        if let ::saola_core::query_core::ArgumentValue::List(l) = list {
            l.push(val.clone());
        }
        self
    }
    pub fn create_many<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut PostDataBuilder),
    {
        let mut builder = PostDataBuilder::default();
        f(&mut builder);
        let val = ::saola_core::query_core::ArgumentValue::Object(builder.data);
        let list = self.data.entry("createMany".to_string()).or_insert_with(|| {
            let mut map = ::saola_core::IndexMap::new();
            map.insert(
                "data".to_string(),
                ::saola_core::query_core::ArgumentValue::List(Vec::new()),
            );
            ::saola_core::query_core::ArgumentValue::Object(map)
        });
        if let ::saola_core::query_core::ArgumentValue::Object(map) = list {
            if let Some(::saola_core::query_core::ArgumentValue::List(l)) = map.get_mut("data") {
                l.push(val);
            }
        }
        self
    }
    pub fn connect<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let val = ::saola_core::query_core::ArgumentValue::Object(builder.build());
        let list = self
            .data
            .entry("connect".to_string())
            .or_insert_with(|| ::saola_core::query_core::ArgumentValue::List(Vec::new()));
        if let ::saola_core::query_core::ArgumentValue::List(l) = list {
            l.push(val.clone());
        }
        self
    }
    pub fn set<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let val = ::saola_core::query_core::ArgumentValue::Object(builder.build());
        let list = self
            .data
            .entry("set".to_string())
            .or_insert_with(|| ::saola_core::query_core::ArgumentValue::List(Vec::new()));
        if let ::saola_core::query_core::ArgumentValue::List(l) = list {
            l.push(val.clone());
        }
        self
    }
    pub fn disconnect<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let val = ::saola_core::query_core::ArgumentValue::Object(builder.build());
        let list = self
            .data
            .entry("disconnect".to_string())
            .or_insert_with(|| ::saola_core::query_core::ArgumentValue::List(Vec::new()));
        if let ::saola_core::query_core::ArgumentValue::List(l) = list {
            l.push(val.clone());
        }
        self
    }
    pub fn delete<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let val = ::saola_core::query_core::ArgumentValue::Object(builder.build());
        let list = self
            .data
            .entry("delete".to_string())
            .or_insert_with(|| ::saola_core::query_core::ArgumentValue::List(Vec::new()));
        if let ::saola_core::query_core::ArgumentValue::List(l) = list {
            l.push(val.clone());
        }
        self
    }
    pub fn update<F>(&mut self, where_f: impl FnOnce(&mut PostUniqueWhereBuilder), data_f: F) -> &mut Self
    where
        F: FnOnce(&mut PostDataBuilder),
    {
        let mut w_builder = PostUniqueWhereBuilder::default();
        where_f(&mut w_builder);
        let mut d_builder = PostDataBuilder::default();
        data_f(&mut d_builder);
        let mut map = ::saola_core::IndexMap::new();
        use ::saola_core::FilterBuilder;
        map.insert(
            "where".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(w_builder.build()),
        );
        map.insert(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut d_builder.data)),
        );
        let list = self
            .data
            .entry("update".to_string())
            .or_insert_with(|| ::saola_core::query_core::ArgumentValue::List(Vec::new()));
        if let ::saola_core::query_core::ArgumentValue::List(l) = list {
            l.push(::saola_core::query_core::ArgumentValue::Object(map));
        }
        self
    }
    pub fn update_many<F>(&mut self, where_f: impl FnOnce(&mut PostWhereBuilder), data_f: F) -> &mut Self
    where
        F: FnOnce(&mut PostDataBuilder),
    {
        let mut w_builder = PostWhereBuilder::default();
        where_f(&mut w_builder);
        let mut d_builder = PostDataBuilder::default();
        data_f(&mut d_builder);
        let mut map = ::saola_core::IndexMap::new();
        use ::saola_core::FilterBuilder;
        map.insert(
            "where".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(w_builder.build()),
        );
        map.insert(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut d_builder.data)),
        );
        self.data.insert(
            "updateMany".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self
    }
    pub fn delete_many(&mut self, f: impl FnOnce(&mut PostWhereBuilder)) -> &mut Self {
        let mut w_builder = PostWhereBuilder::default();
        f(&mut w_builder);
        use ::saola_core::FilterBuilder;
        self.data.insert(
            "deleteMany".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(w_builder.build()),
        );
        self
    }
    pub fn upsert<F>(
        &mut self,
        where_f: impl FnOnce(&mut PostUniqueWhereBuilder),
        title: String,
        create_f: impl FnOnce(&mut PostDataBuilder),
        update_f: F,
    ) -> &mut Self
    where
        F: FnOnce(&mut PostDataBuilder),
    {
        let mut w_builder = PostUniqueWhereBuilder::default();
        where_f(&mut w_builder);
        let mut create_builder = PostDataBuilder::default();
        create_builder.data.insert(
            "title".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(title)),
        );
        create_f(&mut create_builder);
        let mut update_builder = PostDataBuilder::default();
        update_f(&mut update_builder);
        let mut map = ::saola_core::IndexMap::new();
        use ::saola_core::FilterBuilder;
        map.insert(
            "where".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(w_builder.build()),
        );
        map.insert(
            "create".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut create_builder.data)),
        );
        map.insert(
            "update".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut update_builder.data)),
        );
        let list = self
            .data
            .entry("upsert".to_string())
            .or_insert_with(|| ::saola_core::query_core::ArgumentValue::List(Vec::new()));
        if let ::saola_core::query_core::ArgumentValue::List(l) = list {
            l.push(::saola_core::query_core::ArgumentValue::Object(map));
        }
        self
    }
}
#[derive(Default)]
pub struct UserProfileRelationWriteBuilder {
    pub data: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl UserProfileRelationWriteBuilder {
    pub fn create<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut ProfileDataBuilder),
    {
        let mut builder = ProfileDataBuilder::default();
        f(&mut builder);
        let val = ::saola_core::query_core::ArgumentValue::Object(builder.data);
        let mut wrap = ::saola_core::IndexMap::new();
        wrap.insert("create".to_string(), val);
        self.data = wrap;
        self
    }
    pub fn connect<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        let mut builder = ProfileUniqueWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let val = ::saola_core::query_core::ArgumentValue::Object(builder.build());
        let mut wrap = ::saola_core::IndexMap::new();
        wrap.insert("connect".to_string(), val);
        self.data = wrap;
        self
    }
    pub fn disconnect(&mut self) -> &mut Self {
        let mut wrap = ::saola_core::IndexMap::new();
        wrap.insert(
            "disconnect".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Boolean(true)),
        );
        self.data = wrap;
        self
    }
    pub fn delete(&mut self) -> &mut Self {
        let mut wrap = ::saola_core::IndexMap::new();
        wrap.insert(
            "delete".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Boolean(true)),
        );
        self.data = wrap;
        self
    }
    pub fn update<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut ProfileDataBuilder),
    {
        let mut builder = ProfileDataBuilder::default();
        f(&mut builder);
        let mut wrap = ::saola_core::IndexMap::new();
        wrap.insert(
            "update".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
        );
        self.data = wrap;
        self
    }
    pub fn upsert<F>(&mut self, create_f: impl FnOnce(&mut ProfileDataBuilder), update_f: F) -> &mut Self
    where
        F: FnOnce(&mut ProfileDataBuilder),
    {
        let mut create_builder = ProfileDataBuilder::default();
        create_f(&mut create_builder);
        let mut update_builder = ProfileDataBuilder::default();
        update_f(&mut update_builder);
        let mut map = ::saola_core::IndexMap::new();
        map.insert(
            "create".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut create_builder.data)),
        );
        map.insert(
            "update".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut update_builder.data)),
        );
        let mut wrap = ::saola_core::IndexMap::new();
        wrap.insert(
            "upsert".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(map),
        );
        self.data = wrap;
        self
    }
}
#[derive(Default)]
pub struct UserCountAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl UserCountAggregateSelectBuilder {
    pub fn _all(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("_all"));
        self
    }
    pub fn id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("id"));
        self
    }
    pub fn email(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("email"));
        self
    }
    pub fn name(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("name"));
        self
    }
    pub fn is_active(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("isActive"));
        self
    }
    pub fn score(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("score"));
        self
    }
    pub fn level(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("level"));
        self
    }
    pub fn created_at(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("createdAt"));
        self
    }
}
#[derive(Default)]
pub struct UserSumAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl UserSumAggregateSelectBuilder {
    pub fn score(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("score"));
        self
    }
    pub fn level(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("level"));
        self
    }
}
#[derive(Default)]
pub struct UserAvgAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl UserAvgAggregateSelectBuilder {
    pub fn score(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("score"));
        self
    }
    pub fn level(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("level"));
        self
    }
}
#[derive(Default)]
pub struct UserMinAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl UserMinAggregateSelectBuilder {
    pub fn id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("id"));
        self
    }
    pub fn email(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("email"));
        self
    }
    pub fn name(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("name"));
        self
    }
    pub fn is_active(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("isActive"));
        self
    }
    pub fn score(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("score"));
        self
    }
    pub fn level(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("level"));
        self
    }
    pub fn created_at(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("createdAt"));
        self
    }
}
#[derive(Default)]
pub struct UserMaxAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl UserMaxAggregateSelectBuilder {
    pub fn id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("id"));
        self
    }
    pub fn email(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("email"));
        self
    }
    pub fn name(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("name"));
        self
    }
    pub fn is_active(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("isActive"));
        self
    }
    pub fn score(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("score"));
        self
    }
    pub fn level(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("level"));
        self
    }
    pub fn created_at(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("createdAt"));
        self
    }
}
#[derive(Default)]
pub struct UserGroupBySelectBuilder {
    pub fields: Vec<String>,
}
impl UserGroupBySelectBuilder {
    pub fn id(&mut self) -> &mut Self {
        self.fields.push("id".to_string());
        self
    }
    pub fn email(&mut self) -> &mut Self {
        self.fields.push("email".to_string());
        self
    }
    pub fn name(&mut self) -> &mut Self {
        self.fields.push("name".to_string());
        self
    }
    pub fn is_active(&mut self) -> &mut Self {
        self.fields.push("isActive".to_string());
        self
    }
    pub fn score(&mut self) -> &mut Self {
        self.fields.push("score".to_string());
        self
    }
    pub fn level(&mut self) -> &mut Self {
        self.fields.push("level".to_string());
        self
    }
    pub fn created_at(&mut self) -> &mut Self {
        self.fields.push("createdAt".to_string());
        self
    }
}
pub struct UserQuery {
    pub provider: std::sync::Arc<dyn ::saola_core::transaction::QueryExecutorProvider>,
}
impl UserQuery {
    pub fn find_many(&self) -> UserManyReadBuilder<User> {
        UserManyReadBuilder {
            inner: ::saola_core::ReadBuilder::find_many(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_unique(&self) -> UserUniqueReadBuilder<User> {
        UserUniqueReadBuilder {
            inner: ::saola_core::ReadBuilder::find_unique(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_first(&self) -> UserFirstReadBuilder<User> {
        UserFirstReadBuilder {
            inner: ::saola_core::ReadBuilder::find_first(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_unique_or_throw(&self) -> UserUniqueOrThrowReadBuilder<User> {
        UserUniqueOrThrowReadBuilder {
            inner: ::saola_core::ReadBuilder::find_unique_or_throw(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_first_or_throw(&self) -> UserFirstOrThrowReadBuilder<User> {
        UserFirstOrThrowReadBuilder {
            inner: ::saola_core::ReadBuilder::find_first_or_throw(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn create(&self, email: String, name: String) -> UserWriteBuilder<User> {
        let mut inner = ::saola_core::WriteBuilder::create(
            UserMarker::NAME.to_string(),
            UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
        )
        .with_provider(self.provider.clone());
        let mut data_map = ::saola_core::IndexMap::new();
        data_map.insert(
            "email".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(email)),
        );
        data_map.insert(
            "name".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(name)),
        );
        inner.state.arguments.insert(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(data_map),
        );
        UserWriteBuilder {
            inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn update(&self) -> UserWriteBuilder<User> {
        UserWriteBuilder {
            inner: ::saola_core::WriteBuilder::update(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn delete(&self) -> UserWriteBuilder<User> {
        UserWriteBuilder {
            inner: ::saola_core::WriteBuilder::delete(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn upsert(&self, email: String, name: String) -> UserUpsertBuilder {
        let mut inner = ::saola_core::WriteBuilder::upsert(
            UserMarker::NAME.to_string(),
            UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
        )
        .with_provider(self.provider.clone());
        let mut data_map = ::saola_core::IndexMap::new();
        data_map.insert(
            "email".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(email)),
        );
        data_map.insert(
            "name".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(name)),
        );
        inner.state.arguments.insert(
            "create".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(data_map),
        );
        UserUpsertBuilder { inner }
    }
    pub fn create_many(&self) -> UserCreateManyBuilder {
        UserCreateManyBuilder {
            inner: ::saola_core::CreateManyBuilder::new(UserMarker::NAME.to_string())
                .with_provider(self.provider.clone()),
        }
    }
    pub fn create_many_and_return(&self) -> UserCreateManyAndReturnBuilder {
        UserCreateManyAndReturnBuilder {
            inner: ::saola_core::CreateManyAndReturnBuilder::new(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn update_many(&self) -> UserUpdateManyBuilder {
        UserUpdateManyBuilder {
            inner: ::saola_core::UpdateManyBuilder::new(UserMarker::NAME.to_string())
                .with_provider(self.provider.clone()),
        }
    }
    pub fn update_many_and_return(&self) -> UserUpdateManyAndReturnBuilder {
        UserUpdateManyAndReturnBuilder {
            inner: ::saola_core::UpdateManyAndReturnBuilder::new(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn delete_many(&self) -> UserDeleteManyBuilder {
        UserDeleteManyBuilder {
            inner: ::saola_core::DeleteManyBuilder::new(UserMarker::NAME.to_string())
                .with_provider(self.provider.clone()),
        }
    }
    pub fn count(&self) -> UserCountBuilder {
        UserCountBuilder {
            inner: ::saola_core::CountBuilder::new(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
        }
    }
    pub fn aggregate(&self) -> UserAggregateBuilder {
        UserAggregateBuilder {
            inner: ::saola_core::AggregateBuilder::new(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
        }
    }
    pub fn group_by(&self) -> UserGroupByBuilder {
        UserGroupByBuilder {
            inner: ::saola_core::GroupByBuilder::new(
                UserMarker::NAME.to_string(),
                UserMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
        }
    }
}
pub trait UserIncludeMarker {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection>;
}
pub trait UserIncludeTransition<M> {
    type Output: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default;
}
pub struct UserIncludeEmpty;
impl UserIncludeMarker for UserIncludeEmpty {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        None
    }
}
pub struct UserIncludePosts {
    pub selection: ::saola_core::query_core::Selection,
}
impl UserIncludeMarker for UserIncludePosts {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl UserIncludePosts {
    pub fn model_as<U: ::saola_core::builder::SelectStruct>(mut self) -> UserIncludePostsAs<U> {
        self.selection.clear_nested_selections();
        for sel in U::selections() {
            self.selection.push_nested_selection(sel);
        }
        UserIncludePostsAs {
            selection: self.selection,
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct UserIncludePostsWith<M> {
    pub selection: ::saola_core::query_core::Selection,
    pub _phantom: std::marker::PhantomData<M>,
}
impl<M> UserIncludeMarker for UserIncludePostsWith<M> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl<M> UserIncludePostsWith<M> {
    pub fn model_as<U: ::saola_core::builder::SelectStruct>(mut self) -> UserIncludePostsAs<U> {
        self.selection.clear_nested_selections();
        for sel in U::selections() {
            self.selection.push_nested_selection(sel);
        }
        UserIncludePostsAs {
            selection: self.selection,
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct UserIncludePostsAs<U> {
    pub selection: ::saola_core::query_core::Selection,
    pub _phantom: std::marker::PhantomData<U>,
}
impl<U> UserIncludeMarker for UserIncludePostsAs<U> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
pub struct UserIncludeProfile {
    pub selection: ::saola_core::query_core::Selection,
}
impl UserIncludeMarker for UserIncludeProfile {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl UserIncludeProfile {
    pub fn model_as<U: ::saola_core::builder::SelectStruct>(mut self) -> UserIncludeProfileAs<U> {
        self.selection.clear_nested_selections();
        for sel in U::selections() {
            self.selection.push_nested_selection(sel);
        }
        UserIncludeProfileAs {
            selection: self.selection,
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct UserIncludeProfileWith<M> {
    pub selection: ::saola_core::query_core::Selection,
    pub _phantom: std::marker::PhantomData<M>,
}
impl<M> UserIncludeMarker for UserIncludeProfileWith<M> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl<M> UserIncludeProfileWith<M> {
    pub fn model_as<U: ::saola_core::builder::SelectStruct>(mut self) -> UserIncludeProfileAs<U> {
        self.selection.clear_nested_selections();
        for sel in U::selections() {
            self.selection.push_nested_selection(sel);
        }
        UserIncludeProfileAs {
            selection: self.selection,
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct UserIncludeProfileAs<U> {
    pub selection: ::saola_core::query_core::Selection,
    pub _phantom: std::marker::PhantomData<U>,
}
impl<U> UserIncludeMarker for UserIncludeProfileAs<U> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl<T0, T1> UserIncludeTransition<UserIncludePosts> for UserData<T0, T1>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    T1: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = UserData<Vec<PostData>, T1>;
}
impl<T0, T1, M> UserIncludeTransition<UserIncludePostsWith<M>> for UserData<T0, T1>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    T1: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    PostData: PostIncludeTransition<M>,
{
    type Output = UserData<Vec<<PostData as PostIncludeTransition<M>>::Output>, T1>;
}
impl<T0, T1, U: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default>
    UserIncludeTransition<UserIncludePostsAs<U>> for UserData<T0, T1>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    T1: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = UserData<U, T1>;
}
impl<T0, T1> UserIncludeTransition<UserIncludeProfile> for UserData<T0, T1>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    T1: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = UserData<T0, Option<Box<ProfileData>>>;
}
impl<T0, T1, M> UserIncludeTransition<UserIncludeProfileWith<M>> for UserData<T0, T1>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    T1: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    ProfileData: ProfileIncludeTransition<M>,
{
    type Output = UserData<T0, Option<Box<<ProfileData as ProfileIncludeTransition<M>>::Output>>>;
}
impl<T0, T1, U: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default>
    UserIncludeTransition<UserIncludeProfileAs<U>> for UserData<T0, T1>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    T1: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = UserData<T0, U>;
}
impl<T0, T1> UserIncludeTransition<UserIncludeEmpty> for UserData<T0, T1>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    T1: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = UserData<T0, T1>;
}
impl UserIncludeTransition<UserIncludeEmpty> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl UserIncludeTransition<UserIncludePosts> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl<M> UserIncludeTransition<UserIncludePostsWith<M>> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl<U> UserIncludeTransition<UserIncludePostsAs<U>> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl UserIncludeTransition<UserIncludeProfile> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl<M> UserIncludeTransition<UserIncludeProfileWith<M>> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl<U> UserIncludeTransition<UserIncludeProfileAs<U>> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
#[derive(Default)]
pub struct UserIncludeBuilder {
    pub args: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl UserIncludeBuilder {
    pub fn scalar_selections() -> Vec<::saola_core::query_core::Selection> {
        vec![
            ::saola_core::query_core::Selection::with_name("id".to_string()),
            ::saola_core::query_core::Selection::with_name("email".to_string()),
            ::saola_core::query_core::Selection::with_name("name".to_string()),
            ::saola_core::query_core::Selection::with_name("isActive".to_string()),
            ::saola_core::query_core::Selection::with_name("score".to_string()),
            ::saola_core::query_core::Selection::with_name("level".to_string()),
            ::saola_core::query_core::Selection::with_name("createdAt".to_string()),
        ]
    }
    pub fn where_clause<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        let map = builder.build();
        if !map.is_empty() {
            self.args.insert(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn take(&mut self, take: i64) -> &mut Self {
        self.args.insert(
            "take".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Int(take)),
        );
        self
    }
    pub fn skip(&mut self, skip: i64) -> &mut Self {
        self.args.insert(
            "skip".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Int(skip)),
        );
        self
    }
    pub fn order_by<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut UserOrderByBuilder),
    {
        let mut builder = UserOrderByBuilder::default();
        f(&mut builder);
        if !builder.args.is_empty() {
            self.args.insert(
                "orderBy".to_string(),
                ::saola_core::query_core::ArgumentValue::List(builder.args),
            );
        }
        self
    }
    pub fn cursor<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        let mut builder = UserUniqueWhereBuilder::default();
        f(&mut builder);
        let map = builder.build();
        if !map.is_empty() {
            self.args.insert(
                "cursor".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn empty(&mut self) -> UserIncludeEmpty {
        UserIncludeEmpty
    }
    pub fn posts(&mut self) -> UserIncludePosts {
        let mut builder = PostSelectBuilder::default();
        builder.all();
        let mut sel = ::saola_core::query_core::Selection::with_name("posts".to_string());
        let fields: Vec<::saola_core::query_core::Selection> = builder.into();
        for f in fields {
            sel.push_nested_selection(f);
        }
        UserIncludePosts { selection: sel }
    }
    pub fn posts_with<M, F>(&mut self, f: F) -> UserIncludePostsWith<M>
    where
        F: FnOnce(&mut PostIncludeBuilder) -> M,
        M: PostIncludeMarker,
    {
        let mut builder = PostIncludeBuilder::default();
        let marker = f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("posts".to_string());
        for scalar in PostIncludeBuilder::scalar_selections() {
            sel.push_nested_selection(scalar);
        }
        if let Some(nested) = marker.into_selection() {
            sel.push_nested_selection(nested);
        }
        for (k, v) in std::mem::take(&mut builder.args) {
            sel.push_argument(k, v);
        }
        UserIncludePostsWith {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn posts_as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync, F>(
        &mut self,
        selection: (std::marker::PhantomData<U>, F),
    ) -> UserIncludePostsAs<U>
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut builder = PostSelectBuilder::default();
        (selection.1)(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("posts".to_string());
        let fields: Vec<::saola_core::query_core::Selection> = builder.into();
        for f in fields {
            sel.push_nested_selection(f);
        }
        UserIncludePostsAs {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn profile(&mut self) -> UserIncludeProfile {
        let mut builder = ProfileSelectBuilder::default();
        builder.all();
        let mut sel = ::saola_core::query_core::Selection::with_name("profile".to_string());
        let fields: Vec<::saola_core::query_core::Selection> = builder.into();
        for f in fields {
            sel.push_nested_selection(f);
        }
        UserIncludeProfile { selection: sel }
    }
    pub fn profile_with<M, F>(&mut self, f: F) -> UserIncludeProfileWith<M>
    where
        F: FnOnce(&mut ProfileIncludeBuilder) -> M,
        M: ProfileIncludeMarker,
    {
        let mut builder = ProfileIncludeBuilder::default();
        let marker = f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("profile".to_string());
        for scalar in ProfileIncludeBuilder::scalar_selections() {
            sel.push_nested_selection(scalar);
        }
        if let Some(nested) = marker.into_selection() {
            sel.push_nested_selection(nested);
        }
        for (k, v) in std::mem::take(&mut builder.args) {
            sel.push_argument(k, v);
        }
        UserIncludeProfileWith {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn profile_as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync, F>(
        &mut self,
        selection: (std::marker::PhantomData<U>, F),
    ) -> UserIncludeProfileAs<U>
    where
        F: FnOnce(&mut ProfileSelectBuilder),
    {
        let mut builder = ProfileSelectBuilder::default();
        (selection.1)(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("profile".to_string());
        let fields: Vec<::saola_core::query_core::Selection> = builder.into();
        for f in fields {
            sel.push_nested_selection(f);
        }
        UserIncludeProfileAs {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl UserIncludeMarker for UserIncludeBuilder {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        None
    }
}
pub struct UserManyReadBuilder<T = User> {
    pub inner: ::saola_core::ReadBuilder<Vec<T>>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    UserManyReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn where_unique<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn order_by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserOrderByBuilder),
    {
        let mut builder = UserOrderByBuilder::default();
        f(&mut builder);
        if !builder.args.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "orderBy".to_string(),
                ::saola_core::query_core::ArgumentValue::List(builder.args),
            );
        }
        self
    }
    pub fn skip(mut self, skip: i64) -> Self {
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "skip".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Int(skip)),
        );
        self
    }
    pub fn take(mut self, take: i64) -> Self {
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "take".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Int(take)),
        );
        self
    }
    pub fn cursor<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        let mut builder = UserUniqueWhereBuilder::default();
        f(&mut builder);
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "cursor".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn select<F>(mut self, f: F) -> UserManyReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut UserSelectBuilder),
    {
        let mut builder = UserSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        UserManyReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> UserManyReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, Vec<U>>();
        UserManyReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> UserManyReadBuilder<<T as UserIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut UserIncludeBuilder) -> M,
        M: UserIncludeMarker,
        T: UserIncludeTransition<M>,
    {
        let mut builder = UserIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
                self.inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(scalar_field.to_string()));
            }
        }
        if let Some(nested) = marker.into_selection() {
            self.inner.add_nested_selection(nested);
        }
        for (k, v) in std::mem::take(&mut builder.args) {
            self.inner.add_filter_arg(k, v);
        }
        UserManyReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> UserManyReadBuilder<U> {
        UserManyReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<Vec<T>> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<Vec<T>> {
        self.inner.exec().await
    }
}
pub struct UserUniqueReadBuilder<T = User> {
    pub inner: ::saola_core::ReadBuilder<Option<T>>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    UserUniqueReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        let mut builder = UserUniqueWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn where_unique<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn select<F>(mut self, f: F) -> UserUniqueReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut UserSelectBuilder),
    {
        let mut builder = UserSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        UserUniqueReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> UserUniqueReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, Option<U>>();
        UserUniqueReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> UserUniqueReadBuilder<<T as UserIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut UserIncludeBuilder) -> M,
        M: UserIncludeMarker,
        T: UserIncludeTransition<M>,
    {
        let mut builder = UserIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
                self.inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(scalar_field.to_string()));
            }
        }
        if let Some(nested) = marker.into_selection() {
            self.inner.add_nested_selection(nested);
        }
        for (k, v) in std::mem::take(&mut builder.args) {
            self.inner.add_filter_arg(k, v);
        }
        UserUniqueReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> UserUniqueReadBuilder<U> {
        UserUniqueReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<Option<T>> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<Option<T>> {
        self.inner.exec().await
    }
}
pub struct UserFirstReadBuilder<T = User> {
    pub inner: ::saola_core::ReadBuilder<Option<T>>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    UserFirstReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn where_unique<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn order_by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserOrderByBuilder),
    {
        let mut builder = UserOrderByBuilder::default();
        f(&mut builder);
        if !builder.args.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "orderBy".to_string(),
                ::saola_core::query_core::ArgumentValue::List(builder.args),
            );
        }
        self
    }
    pub fn skip(mut self, skip: i64) -> Self {
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "skip".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Int(skip)),
        );
        self
    }
    pub fn take(mut self, take: i64) -> Self {
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "take".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Int(take)),
        );
        self
    }
    pub fn cursor<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        let mut builder = UserUniqueWhereBuilder::default();
        f(&mut builder);
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "cursor".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn select<F>(mut self, f: F) -> UserFirstReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut UserSelectBuilder),
    {
        let mut builder = UserSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        UserFirstReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> UserFirstReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, Option<U>>();
        UserFirstReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> UserFirstReadBuilder<<T as UserIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut UserIncludeBuilder) -> M,
        M: UserIncludeMarker,
        T: UserIncludeTransition<M>,
    {
        let mut builder = UserIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
                self.inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(scalar_field.to_string()));
            }
        }
        if let Some(nested) = marker.into_selection() {
            self.inner.add_nested_selection(nested);
        }
        for (k, v) in std::mem::take(&mut builder.args) {
            self.inner.add_filter_arg(k, v);
        }
        UserFirstReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> UserFirstReadBuilder<U> {
        UserFirstReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<Option<T>> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<Option<T>> {
        self.inner.exec().await
    }
}
pub struct UserUniqueOrThrowReadBuilder<T = User> {
    pub inner: ::saola_core::ReadBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    UserUniqueOrThrowReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        let mut builder = UserUniqueWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn where_unique<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn select<F>(mut self, f: F) -> UserUniqueOrThrowReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut UserSelectBuilder),
    {
        let mut builder = UserSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        UserUniqueOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> UserUniqueOrThrowReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        UserUniqueOrThrowReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> UserUniqueOrThrowReadBuilder<<T as UserIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut UserIncludeBuilder) -> M,
        M: UserIncludeMarker,
        T: UserIncludeTransition<M>,
    {
        let mut builder = UserIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
                self.inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(scalar_field.to_string()));
            }
        }
        if let Some(nested) = marker.into_selection() {
            self.inner.add_nested_selection(nested);
        }
        for (k, v) in std::mem::take(&mut builder.args) {
            self.inner.add_filter_arg(k, v);
        }
        UserUniqueOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> UserUniqueOrThrowReadBuilder<U> {
        UserUniqueOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<T> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<T> {
        self.inner.exec().await
    }
}
pub struct UserFirstOrThrowReadBuilder<T = User> {
    pub inner: ::saola_core::ReadBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    UserFirstOrThrowReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn where_unique<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn order_by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserOrderByBuilder),
    {
        let mut builder = UserOrderByBuilder::default();
        f(&mut builder);
        if !builder.args.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "orderBy".to_string(),
                ::saola_core::query_core::ArgumentValue::List(builder.args),
            );
        }
        self
    }
    pub fn skip(mut self, skip: i64) -> Self {
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "skip".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Int(skip)),
        );
        self
    }
    pub fn take(mut self, take: i64) -> Self {
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "take".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Int(take)),
        );
        self
    }
    pub fn cursor<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        let mut builder = UserUniqueWhereBuilder::default();
        f(&mut builder);
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "cursor".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn select<F>(mut self, f: F) -> UserFirstOrThrowReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut UserSelectBuilder),
    {
        let mut builder = UserSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        UserFirstOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> UserFirstOrThrowReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        UserFirstOrThrowReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> UserFirstOrThrowReadBuilder<<T as UserIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut UserIncludeBuilder) -> M,
        M: UserIncludeMarker,
        T: UserIncludeTransition<M>,
    {
        let mut builder = UserIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
                self.inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(scalar_field.to_string()));
            }
        }
        if let Some(nested) = marker.into_selection() {
            self.inner.add_nested_selection(nested);
        }
        for (k, v) in std::mem::take(&mut builder.args) {
            self.inner.add_filter_arg(k, v);
        }
        UserFirstOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> UserFirstOrThrowReadBuilder<U> {
        UserFirstOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<T> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<T> {
        self.inner.exec().await
    }
}
#[doc = r" Builder for write operations (returns T)"]
pub struct UserWriteBuilder<T = User> {
    pub inner: ::saola_core::WriteBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    UserWriteBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        let mut builder = UserUniqueWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn where_unique<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserDataBuilder),
    {
        let mut builder = UserDataBuilder::default();
        f(&mut builder);
        let mut merged_data = std::mem::take(&mut builder.data);
        if let Some(::saola_core::query_core::ArgumentValue::Object(existing_map)) =
            self.inner.state.arguments.get("data")
        {
            let mut new_map = existing_map.clone();
            for (k, v) in merged_data {
                new_map.insert(k, v);
            }
            merged_data = new_map;
        }
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(merged_data),
        );
        self
    }
    pub fn select<F>(mut self, f: F) -> UserWriteBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut UserSelectBuilder),
    {
        let mut builder = UserSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        UserWriteBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> UserWriteBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        UserWriteBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> UserWriteBuilder<<T as UserIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut UserIncludeBuilder) -> M,
        M: UserIncludeMarker,
        T: UserIncludeTransition<M>,
    {
        let mut builder = UserIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
                self.inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(scalar_field.to_string()));
            }
        }
        if let Some(nested) = marker.into_selection() {
            self.inner.add_nested_selection(nested);
        }
        for (k, v) in std::mem::take(&mut builder.args) {
            self.inner.add_filter_arg(k, v);
        }
        UserWriteBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> UserWriteBuilder<U> {
        UserWriteBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<T> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<T> {
        self.inner.exec().await
    }
}
pub struct UserUpsertBuilder {
    pub inner: ::saola_core::WriteBuilder<User>,
}
impl UserUpsertBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        let mut builder = UserUniqueWhereBuilder::default();
        f(&mut builder);
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::FilterBuilder;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn where_unique<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn update<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserDataBuilder),
    {
        let mut builder = UserDataBuilder::default();
        f(&mut builder);
        self.inner.add_filter_arg(
            "update".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
        );
        self
    }
    pub fn create<F>(mut self, email: String, name: String, f: F) -> Self
    where
        F: FnOnce(&mut UserDataBuilder),
    {
        let mut create_builder = UserDataBuilder::default();
        create_builder.data.insert(
            "email".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(email)),
        );
        create_builder.data.insert(
            "name".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(name)),
        );
        f(&mut create_builder);
        self.inner.add_filter_arg(
            "create".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut create_builder.data)),
        );
        self
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<User> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<User> {
        self.inner.exec().await
    }
}
pub struct UserCreateManyBuilder {
    pub inner: ::saola_core::CreateManyBuilder,
}
impl UserCreateManyBuilder {
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserScalarDataBuilder),
    {
        let mut builder = UserScalarDataBuilder::default();
        f(&mut builder);
        let mut list = match self.inner.state.arguments.get("data") {
            Some(::saola_core::query_core::ArgumentValue::List(l)) => l.clone(),
            _ => Vec::new(),
        };
        list.push(::saola_core::query_core::ArgumentValue::Object(std::mem::take(
            &mut builder.data,
        )));
        use ::saola_core::Filterable;
        self.inner
            .add_filter_arg("data".to_string(), ::saola_core::query_core::ArgumentValue::List(list));
        self
    }
    pub fn skip_duplicates(mut self, skip: bool) -> Self {
        self.inner = self.inner.skip_duplicates(skip);
        self
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<i64> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<i64> {
        self.inner.exec().await
    }
}
pub struct UserCreateManyAndReturnBuilder<T = User> {
    pub inner: ::saola_core::CreateManyAndReturnBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    UserCreateManyAndReturnBuilder<T>
{
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserScalarDataBuilder),
    {
        let mut builder = UserScalarDataBuilder::default();
        f(&mut builder);
        let mut list = match self.inner.state.arguments.get("data") {
            Some(::saola_core::query_core::ArgumentValue::List(l)) => l.clone(),
            _ => Vec::new(),
        };
        list.push(::saola_core::query_core::ArgumentValue::Object(std::mem::take(
            &mut builder.data,
        )));
        use ::saola_core::Filterable;
        self.inner
            .add_filter_arg("data".to_string(), ::saola_core::query_core::ArgumentValue::List(list));
        self
    }
    pub fn skip_duplicates(mut self, skip: bool) -> Self {
        self.inner = self.inner.skip_duplicates(skip);
        self
    }
    pub fn select<F>(mut self, f: F) -> UserCreateManyAndReturnBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut UserSelectBuilder),
    {
        let mut builder = UserSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        UserCreateManyAndReturnBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> UserCreateManyAndReturnBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        UserCreateManyAndReturnBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<Vec<T>> {
        let mut builder = self;
        if builder.inner.state.selection.nested_selections().is_empty() {
            for field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
                use ::saola_core::Selectable;
                builder
                    .inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(field.to_string()));
            }
        }
        builder.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<Vec<T>> {
        let mut builder = self;
        if builder.inner.state.selection.nested_selections().is_empty() {
            for field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
                use ::saola_core::Selectable;
                builder
                    .inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(field.to_string()));
            }
        }
        builder.inner.exec().await
    }
}
pub struct UserUpdateManyBuilder {
    pub inner: ::saola_core::UpdateManyBuilder,
}
impl UserUpdateManyBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserScalarDataBuilder),
    {
        let mut builder = UserScalarDataBuilder::default();
        f(&mut builder);
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
        );
        self
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<i64> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<i64> {
        self.inner.exec().await
    }
}
pub struct UserUpdateManyAndReturnBuilder<T = User> {
    pub inner: ::saola_core::UpdateManyAndReturnBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    UserUpdateManyAndReturnBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserScalarDataBuilder),
    {
        let mut builder = UserScalarDataBuilder::default();
        f(&mut builder);
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
        );
        self
    }
    pub fn select<F>(mut self, f: F) -> UserUpdateManyAndReturnBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut UserSelectBuilder),
    {
        let mut builder = UserSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        UserUpdateManyAndReturnBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> UserUpdateManyAndReturnBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        UserUpdateManyAndReturnBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<Vec<T>> {
        let mut builder = self;
        if builder.inner.state.selection.nested_selections().is_empty() {
            for field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
                use ::saola_core::Selectable;
                builder
                    .inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(field.to_string()));
            }
        }
        builder.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<Vec<T>> {
        let mut builder = self;
        if builder.inner.state.selection.nested_selections().is_empty() {
            for field in &["id", "email", "name", "isActive", "score", "level", "createdAt"] {
                use ::saola_core::Selectable;
                builder
                    .inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(field.to_string()));
            }
        }
        builder.inner.exec().await
    }
}
pub struct UserDeleteManyBuilder {
    pub inner: ::saola_core::DeleteManyBuilder,
}
impl UserDeleteManyBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<i64> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<i64> {
        self.inner.exec().await
    }
}
pub struct UserCountBuilder {
    pub inner: ::saola_core::CountBuilder,
}
impl UserCountBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<i64> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<i64> {
        self.inner.exec().await
    }
}
pub struct UserAggregateBuilder {
    pub inner: ::saola_core::AggregateBuilder<UserAggregateResult>,
}
impl UserAggregateBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn count<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserCountAggregateSelectBuilder),
    {
        let mut builder = UserCountAggregateSelectBuilder::default();
        f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("_count");
        for s in builder.selections {
            sel.push_nested_selection(s);
        }
        use ::saola_core::Selectable;
        self.inner.add_nested_selection(sel);
        self
    }
    pub fn sum<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserSumAggregateSelectBuilder),
    {
        let mut builder = UserSumAggregateSelectBuilder::default();
        f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("_sum");
        for s in builder.selections {
            sel.push_nested_selection(s);
        }
        use ::saola_core::Selectable;
        self.inner.add_nested_selection(sel);
        self
    }
    pub fn avg<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserAvgAggregateSelectBuilder),
    {
        let mut builder = UserAvgAggregateSelectBuilder::default();
        f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("_avg");
        for s in builder.selections {
            sel.push_nested_selection(s);
        }
        use ::saola_core::Selectable;
        self.inner.add_nested_selection(sel);
        self
    }
    pub fn min<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserMinAggregateSelectBuilder),
    {
        let mut builder = UserMinAggregateSelectBuilder::default();
        f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("_min");
        for s in builder.selections {
            sel.push_nested_selection(s);
        }
        use ::saola_core::Selectable;
        self.inner.add_nested_selection(sel);
        self
    }
    pub fn max<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserMaxAggregateSelectBuilder),
    {
        let mut builder = UserMaxAggregateSelectBuilder::default();
        f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("_max");
        for s in builder.selections {
            sel.push_nested_selection(s);
        }
        use ::saola_core::Selectable;
        self.inner.add_nested_selection(sel);
        self
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<UserAggregateResult> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<UserAggregateResult> {
        self.inner.exec().await
    }
}
pub struct UserGroupByBuilder {
    pub inner: ::saola_core::GroupByBuilder<UserGroupByResult>,
}
impl UserGroupByBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "where".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn having<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserWhereBuilder),
    {
        let mut builder = UserWhereBuilder::default();
        f(&mut builder);
        use ::saola_core::FilterBuilder;
        let map = builder.build();
        if !map.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "having".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(map),
            );
        }
        self
    }
    pub fn take(mut self, take: i64) -> Self {
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "take".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Int(take)),
        );
        self
    }
    pub fn skip(mut self, skip: i64) -> Self {
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "skip".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::Int(skip)),
        );
        self
    }
    pub fn order_by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserOrderByBuilder),
    {
        let mut builder = UserOrderByBuilder::default();
        f(&mut builder);
        if !builder.args.is_empty() {
            use ::saola_core::Filterable;
            self.inner.add_filter_arg(
                "orderBy".to_string(),
                ::saola_core::query_core::ArgumentValue::List(builder.args),
            );
        }
        self
    }
    pub fn by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserGroupBySelectBuilder),
    {
        let mut builder = UserGroupBySelectBuilder::default();
        f(&mut builder);
        let fields = builder.fields.clone();
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "by".to_string(),
            ::saola_core::query_core::ArgumentValue::List(
                fields
                    .iter()
                    .map(|f| {
                        ::saola_core::query_core::ArgumentValue::Scalar(
                            ::saola_core::query_structure::PrismaValue::String(f.clone()),
                        )
                    })
                    .collect(),
            ),
        );
        use ::saola_core::Selectable;
        for f in fields {
            self.inner
                .add_nested_selection(::saola_core::query_core::Selection::with_name(f));
        }
        self
    }
    pub fn count<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserCountAggregateSelectBuilder),
    {
        let mut builder = UserCountAggregateSelectBuilder::default();
        f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("_count");
        for s in builder.selections {
            sel.push_nested_selection(s);
        }
        use ::saola_core::Selectable;
        self.inner.add_nested_selection(sel);
        self
    }
    pub fn sum<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserSumAggregateSelectBuilder),
    {
        let mut builder = UserSumAggregateSelectBuilder::default();
        f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("_sum");
        for s in builder.selections {
            sel.push_nested_selection(s);
        }
        use ::saola_core::Selectable;
        self.inner.add_nested_selection(sel);
        self
    }
    pub fn avg<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserAvgAggregateSelectBuilder),
    {
        let mut builder = UserAvgAggregateSelectBuilder::default();
        f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("_avg");
        for s in builder.selections {
            sel.push_nested_selection(s);
        }
        use ::saola_core::Selectable;
        self.inner.add_nested_selection(sel);
        self
    }
    pub fn min<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserMinAggregateSelectBuilder),
    {
        let mut builder = UserMinAggregateSelectBuilder::default();
        f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("_min");
        for s in builder.selections {
            sel.push_nested_selection(s);
        }
        use ::saola_core::Selectable;
        self.inner.add_nested_selection(sel);
        self
    }
    pub fn max<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut UserMaxAggregateSelectBuilder),
    {
        let mut builder = UserMaxAggregateSelectBuilder::default();
        f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("_max");
        for s in builder.selections {
            sel.push_nested_selection(s);
        }
        use ::saola_core::Selectable;
        self.inner.add_nested_selection(sel);
        self
    }
    pub async fn exec_with(
        self,
        provider: &(dyn ::saola_core::transaction::QueryExecutorProvider + '_),
    ) -> ::saola_core::Result<Vec<UserGroupByResult>> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<Vec<UserGroupByResult>> {
        self.inner.exec().await
    }
}
