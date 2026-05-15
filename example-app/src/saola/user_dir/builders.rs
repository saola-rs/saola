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
    pub fn score(&mut self) -> ::saola_core::StringFilter<'_, Self> {
        ::saola_core::StringFilter {
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
    pub fn created_at(&mut self) -> ::saola_core::StringFilter<'_, Self> {
        ::saola_core::StringFilter {
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
    pub fn score(&mut self) -> ::saola_core::SelectionField<'_, String, Self> {
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
    pub fn created_at(&mut self) -> ::saola_core::SelectionField<'_, String, Self> {
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
pub struct UserIncludeBuilder {
    pub args: Vec<(String, ::saola_core::query_core::ArgumentValue)>,
}
impl UserIncludeBuilder {
    pub fn posts(&mut self) -> UserIncludePosts {
        let mut sel = ::saola_core::query_core::Selection::with_name("posts".to_string());
        for f in super::super::post_dir::builders::PostMarker::SCALAR_FIELDS {
            sel.push_nested_selection(::saola_core::query_core::Selection::with_name(f.to_string()));
        }
        UserIncludePosts { selection: sel }
    }
    pub fn posts_as<U: ::saola_core::builder::SelectStruct>(&mut self) -> UserIncludePostsAs<U> {
        let mut sel = ::saola_core::query_core::Selection::with_name("posts".to_string());
        for f in U::selections() {
            sel.push_nested_selection(f);
        }
        UserIncludePostsAs {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn profile(&mut self) -> UserIncludeProfile {
        let mut sel = ::saola_core::query_core::Selection::with_name("profile".to_string());
        for f in super::super::profile_dir::builders::ProfileMarker::SCALAR_FIELDS {
            sel.push_nested_selection(::saola_core::query_core::Selection::with_name(f.to_string()));
        }
        UserIncludeProfile { selection: sel }
    }
    pub fn profile_as<U: ::saola_core::builder::SelectStruct>(&mut self) -> UserIncludeProfileAs<U> {
        let mut sel = ::saola_core::query_core::Selection::with_name("profile".to_string());
        for f in U::selections() {
            sel.push_nested_selection(f);
        }
        UserIncludeProfileAs {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct UserIncludePosts {
    pub selection: ::saola_core::query_core::Selection,
}
impl ::saola_core::builder::IncludeMarker for UserIncludePosts {
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
pub struct UserIncludePostsAs<U> {
    pub selection: ::saola_core::query_core::Selection,
    pub _phantom: std::marker::PhantomData<U>,
}
impl<U> ::saola_core::builder::IncludeMarker for UserIncludePostsAs<U> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
pub struct UserIncludeProfile {
    pub selection: ::saola_core::query_core::Selection,
}
impl ::saola_core::builder::IncludeMarker for UserIncludeProfile {
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
pub struct UserIncludeProfileAs<U> {
    pub selection: ::saola_core::query_core::Selection,
    pub _phantom: std::marker::PhantomData<U>,
}
impl<U> ::saola_core::builder::IncludeMarker for UserIncludeProfileAs<U> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl<T0, T1> ::saola_core::builder::IncludeTransition<UserIncludePosts> for UserData<T0, T1>
where
    T0: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
    T1: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
{
    type Output = UserData<Vec<PostData>, T1>;
}
impl<U: ::saola_core::builder::SelectStruct, T0, T1> ::saola_core::builder::IncludeTransition<UserIncludePostsAs<U>>
    for UserData<T0, T1>
where
    T0: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
    T1: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
{
    type Output = UserData<Vec<U>, T1>;
}
impl<T0, T1> ::saola_core::builder::IncludeTransition<UserIncludeProfile> for UserData<T0, T1>
where
    T0: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
    T1: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
{
    type Output = UserData<T0, Option<Box<ProfileData>>>;
}
impl<U: ::saola_core::builder::SelectStruct, T0, T1> ::saola_core::builder::IncludeTransition<UserIncludeProfileAs<U>>
    for UserData<T0, T1>
where
    T0: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
    T1: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
{
    type Output = UserData<T0, Option<U>>;
}
impl<U, T0, T1> ::saola_core::builder::SelectAsTransition<U> for UserData<T0, T1> {
    type Output = U;
}
impl<SM, T0, T1> ::saola_core::builder::SelectTransition<SM> for UserData<T0, T1> {
    type Output = ::saola_core::serde_json::Value;
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
    pub fn find_many(&self) -> ::saola_core::Query<UserMarker, ::saola_core::FindMany, Vec<User>> {
        ::saola_core::Query::new("findMany").with_provider(self.provider.clone())
    }
    pub fn find_unique(&self) -> ::saola_core::Query<UserMarker, ::saola_core::FindUnique, Option<User>> {
        ::saola_core::Query::new("findUnique").with_provider(self.provider.clone())
    }
    pub fn find_first(&self) -> ::saola_core::Query<UserMarker, ::saola_core::FindFirst, Option<User>> {
        ::saola_core::Query::new("findFirst").with_provider(self.provider.clone())
    }
    pub fn find_unique_or_throw(&self) -> ::saola_core::Query<UserMarker, ::saola_core::FindUniqueOrThrow, User> {
        ::saola_core::Query::new("findUniqueOrThrow").with_provider(self.provider.clone())
    }
    pub fn find_first_or_throw(&self) -> ::saola_core::Query<UserMarker, ::saola_core::FindFirstOrThrow, User> {
        ::saola_core::Query::new("findFirstOrThrow").with_provider(self.provider.clone())
    }
    pub fn create(&self, email: String, name: String) -> ::saola_core::Query<UserMarker, ::saola_core::Create, User> {
        let mut query = ::saola_core::Query::new("createOne").with_provider(self.provider.clone());
        let mut data_map = ::saola_core::IndexMap::new();
        data_map.insert(
            "email".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(email)),
        );
        data_map.insert(
            "name".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(name)),
        );
        query.state.arguments.insert(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(data_map),
        );
        query
    }
    pub fn update(&self) -> ::saola_core::Query<UserMarker, ::saola_core::Update, User> {
        ::saola_core::Query::new("updateOne").with_provider(self.provider.clone())
    }
    pub fn delete(&self) -> ::saola_core::Query<UserMarker, ::saola_core::Delete, User> {
        ::saola_core::Query::new("deleteOne").with_provider(self.provider.clone())
    }
    pub fn upsert(&self, email: String, name: String) -> ::saola_core::Query<UserMarker, ::saola_core::Upsert, User> {
        let mut query = ::saola_core::Query::new("upsertOne").with_provider(self.provider.clone());
        let mut data_map = ::saola_core::IndexMap::new();
        data_map.insert(
            "email".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(email)),
        );
        data_map.insert(
            "name".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(name)),
        );
        query.state.arguments.insert(
            "create".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(data_map),
        );
        query
    }
}
