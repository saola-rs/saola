use super::super::*;
use ::saola_core::prelude::*;
pub struct ProfileMarker;
impl ::saola_core::ModelMarker for ProfileMarker {
    type Data = ProfileData;
    type Where = ProfileWhereBuilder;
    type UniqueWhere = ProfileUniqueWhereBuilder;
    type OrderBy = ProfileOrderByBuilder;
    type Include = ProfileIncludeBuilder;
    type Select = ProfileSelectBuilder;
    type DataBuilder = ProfileDataBuilder;
    const NAME: &'static str = "Profile";
    const SCALAR_FIELDS: &'static [&'static str] = &["id", "bio", "userId"];
}
#[derive(Default)]
pub struct ProfileWhereBuilder {
    pub args: Vec<(String, ::saola_core::query_core::ArgumentValue)>,
}
impl ::saola_core::FilterBuilder for ProfileWhereBuilder {
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
impl ProfileWhereBuilder {
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
    pub fn bio(&mut self) -> ::saola_core::StringFilter<'_, Self> {
        ::saola_core::StringFilter {
            builder: self,
            field_name: "bio",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn user_id(&mut self) -> ::saola_core::StringFilter<'_, Self> {
        ::saola_core::StringFilter {
            builder: self,
            field_name: "userId",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn user(&mut self) -> ::saola_core::RelationFilter<'_, Self, UserWhereBuilder> {
        ::saola_core::RelationFilter {
            builder: self,
            field_name: "user",
            _phantom: std::marker::PhantomData,
        }
    }
}
#[derive(Default)]
pub struct ProfileUniqueWhereBuilder {
    pub args: Vec<(String, ::saola_core::query_core::ArgumentValue)>,
}
impl ::saola_core::FilterBuilder for ProfileUniqueWhereBuilder {
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
impl ProfileUniqueWhereBuilder {
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
    pub fn user_id<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        use ::saola_core::FilterBuilder;
        self.add_arg(
            "userId".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
}
#[derive(Default)]
pub struct ProfileOrderByBuilder {
    pub args: Vec<::saola_core::ArgumentValue>,
}
impl ProfileOrderByBuilder {
    pub fn id(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("id".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn bio(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("bio".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn user_id(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("userId".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
}
#[derive(Default)]
pub struct ProfileSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl ::saola_core::Selectable for ProfileSelectBuilder {
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
impl ProfileSelectBuilder {
    pub fn all(&mut self) -> &mut Self {
        for field in &["id", "bio", "userId"] {
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
    pub fn _validate_field_bio(&self) {}
    pub fn bio(&mut self) -> ::saola_core::SelectionField<'_, Option<String>, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("bio".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_userId(&self) {}
    pub fn user_id(&mut self) -> ::saola_core::SelectionField<'_, String, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("userId".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_user(&self) {}
    pub fn user<F>(&mut self, f: F) -> ::saola_core::SelectionRelField<'_, Option<()>, Self>
    where
        F: FnOnce(&mut UserSelectBuilder),
    {
        let mut builder = UserSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into_selections();
        let mut sel = ::saola_core::query_core::Selection::with_name("user".to_string());
        for s in selections {
            sel.push_nested_selection(s);
        }
        self.selections.push(sel);
        ::saola_core::SelectionRelField::new(self)
    }
}
impl From<ProfileSelectBuilder> for Vec<::saola_core::query_core::Selection> {
    fn from(b: ProfileSelectBuilder) -> Self {
        b.selections
    }
}
#[derive(Default)]
pub struct ProfileIncludeBuilder {
    pub args: Vec<(String, ::saola_core::query_core::ArgumentValue)>,
}
impl ProfileIncludeBuilder {
    pub fn user(&mut self) -> ProfileIncludeUser {
        let mut sel = ::saola_core::query_core::Selection::with_name("user".to_string());
        for f in super::super::user_dir::builders::UserMarker::SCALAR_FIELDS {
            sel.push_nested_selection(::saola_core::query_core::Selection::with_name(f.to_string()));
        }
        ProfileIncludeUser { selection: sel }
    }
    pub fn user_as<U: ::saola_core::builder::SelectStruct>(&mut self) -> ProfileIncludeUserAs<U> {
        let mut sel = ::saola_core::query_core::Selection::with_name("user".to_string());
        for f in U::selections() {
            sel.push_nested_selection(f);
        }
        ProfileIncludeUserAs {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct ProfileIncludeUser {
    pub selection: ::saola_core::query_core::Selection,
}
impl ::saola_core::builder::IncludeMarker for ProfileIncludeUser {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl ProfileIncludeUser {
    pub fn model_as<U: ::saola_core::builder::SelectStruct>(mut self) -> ProfileIncludeUserAs<U> {
        self.selection.clear_nested_selections();
        for sel in U::selections() {
            self.selection.push_nested_selection(sel);
        }
        ProfileIncludeUserAs {
            selection: self.selection,
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct ProfileIncludeUserAs<U> {
    pub selection: ::saola_core::query_core::Selection,
    pub _phantom: std::marker::PhantomData<U>,
}
impl<U> ::saola_core::builder::IncludeMarker for ProfileIncludeUserAs<U> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl<T0> ::saola_core::builder::IncludeTransition<ProfileIncludeUser> for ProfileData<T0>
where
    T0: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
{
    type Output = ProfileData<Box<UserData>>;
}
impl<U: ::saola_core::builder::SelectStruct, T0> ::saola_core::builder::IncludeTransition<ProfileIncludeUserAs<U>>
    for ProfileData<T0>
where
    T0: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
{
    type Output = ProfileData<U>;
}
impl<U, T0> ::saola_core::builder::SelectAsTransition<U> for ProfileData<T0> {
    type Output = U;
}
impl<SM, T0> ::saola_core::builder::SelectTransition<SM> for ProfileData<T0> {
    type Output = ::saola_core::serde_json::Value;
}
#[derive(Default)]
pub struct ProfileDataBuilder {
    pub data: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl ::saola_core::builder::DataBuilderTrait for ProfileDataBuilder {
    fn build(self) -> ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue> {
        self.data
    }
}
impl ProfileDataBuilder {
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
    pub fn bio<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "bio".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn user_id<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "userId".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn user<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut ProfileUserRelationWriteBuilder),
    {
        let mut builder = ProfileUserRelationWriteBuilder::default();
        f(&mut builder);
        if !builder.data.is_empty() {
            self.data.insert(
                "user".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
            );
        }
        self
    }
}
impl From<ProfileDataBuilder> for ::saola_core::query_structure::PrismaValue {
    fn from(_b: ProfileDataBuilder) -> Self {
        ::saola_core::query_structure::PrismaValue::Null
    }
}
#[derive(Default)]
pub struct ProfileScalarDataBuilder {
    pub data: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl ::saola_core::builder::DataBuilderTrait for ProfileScalarDataBuilder {
    fn build(self) -> ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue> {
        self.data
    }
}
impl ProfileScalarDataBuilder {
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
    pub fn bio<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "bio".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn user_id<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "userId".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
}
#[derive(Default)]
pub struct ProfileUserRelationWriteBuilder {
    pub data: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl ProfileUserRelationWriteBuilder {
    pub fn create<F>(&mut self, email: String, name: String, f: F) -> &mut Self
    where
        F: FnOnce(&mut UserDataBuilder),
    {
        let mut builder = UserDataBuilder::default();
        builder.data.insert(
            "email".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(email)),
        );
        builder.data.insert(
            "name".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(name)),
        );
        f(&mut builder);
        let val = ::saola_core::query_core::ArgumentValue::Object(builder.data);
        let mut wrap = ::saola_core::IndexMap::new();
        wrap.insert("create".to_string(), val);
        self.data = wrap;
        self
    }
    pub fn connect<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut UserUniqueWhereBuilder),
    {
        let mut builder = UserUniqueWhereBuilder::default();
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
        F: FnOnce(&mut UserDataBuilder),
    {
        let mut builder = UserDataBuilder::default();
        f(&mut builder);
        let mut wrap = ::saola_core::IndexMap::new();
        wrap.insert(
            "update".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
        );
        self.data = wrap;
        self
    }
    pub fn upsert<F>(
        &mut self,
        email: String,
        name: String,
        create_f: impl FnOnce(&mut UserDataBuilder),
        update_f: F,
    ) -> &mut Self
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
        create_f(&mut create_builder);
        let mut update_builder = UserDataBuilder::default();
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
pub struct ProfileCountAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl ProfileCountAggregateSelectBuilder {
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
    pub fn bio(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("bio"));
        self
    }
    pub fn user_id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("userId"));
        self
    }
}
#[derive(Default)]
pub struct ProfileSumAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl ProfileSumAggregateSelectBuilder {}
#[derive(Default)]
pub struct ProfileAvgAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl ProfileAvgAggregateSelectBuilder {}
#[derive(Default)]
pub struct ProfileMinAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl ProfileMinAggregateSelectBuilder {
    pub fn id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("id"));
        self
    }
    pub fn bio(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("bio"));
        self
    }
    pub fn user_id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("userId"));
        self
    }
}
#[derive(Default)]
pub struct ProfileMaxAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl ProfileMaxAggregateSelectBuilder {
    pub fn id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("id"));
        self
    }
    pub fn bio(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("bio"));
        self
    }
    pub fn user_id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("userId"));
        self
    }
}
#[derive(Default)]
pub struct ProfileGroupBySelectBuilder {
    pub fields: Vec<String>,
}
impl ProfileGroupBySelectBuilder {
    pub fn id(&mut self) -> &mut Self {
        self.fields.push("id".to_string());
        self
    }
    pub fn bio(&mut self) -> &mut Self {
        self.fields.push("bio".to_string());
        self
    }
    pub fn user_id(&mut self) -> &mut Self {
        self.fields.push("userId".to_string());
        self
    }
}
pub struct ProfileQuery {
    pub provider: std::sync::Arc<dyn ::saola_core::transaction::QueryExecutorProvider>,
}
impl ProfileQuery {
    pub fn find_many(&self) -> ::saola_core::Query<ProfileMarker, ::saola_core::FindMany, Vec<Profile>> {
        ::saola_core::Query::new("findMany").with_provider(self.provider.clone())
    }
    pub fn find_unique(&self) -> ::saola_core::Query<ProfileMarker, ::saola_core::FindUnique, Option<Profile>> {
        ::saola_core::Query::new("findUnique").with_provider(self.provider.clone())
    }
    pub fn find_first(&self) -> ::saola_core::Query<ProfileMarker, ::saola_core::FindFirst, Option<Profile>> {
        ::saola_core::Query::new("findFirst").with_provider(self.provider.clone())
    }
    pub fn find_unique_or_throw(&self) -> ::saola_core::Query<ProfileMarker, ::saola_core::FindUniqueOrThrow, Profile> {
        ::saola_core::Query::new("findUniqueOrThrow").with_provider(self.provider.clone())
    }
    pub fn find_first_or_throw(&self) -> ::saola_core::Query<ProfileMarker, ::saola_core::FindFirstOrThrow, Profile> {
        ::saola_core::Query::new("findFirstOrThrow").with_provider(self.provider.clone())
    }
    pub fn create(
        &self,
        user: impl FnOnce(&mut ProfileUserRelationWriteBuilder),
    ) -> ::saola_core::Query<ProfileMarker, ::saola_core::Create, Profile> {
        let mut query = ::saola_core::Query::new("createOne").with_provider(self.provider.clone());
        let mut data_map = ::saola_core::IndexMap::new();
        {
            let mut rel_builder = ProfileUserRelationWriteBuilder::default();
            user(&mut rel_builder);
            data_map.insert(
                "user".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(rel_builder.data),
            );
        }
        query.state.arguments.insert(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(data_map),
        );
        query
    }
    pub fn update(&self) -> ::saola_core::Query<ProfileMarker, ::saola_core::Update, Profile> {
        ::saola_core::Query::new("updateOne").with_provider(self.provider.clone())
    }
    pub fn delete(&self) -> ::saola_core::Query<ProfileMarker, ::saola_core::Delete, Profile> {
        ::saola_core::Query::new("deleteOne").with_provider(self.provider.clone())
    }
    pub fn upsert(
        &self,
        user: impl FnOnce(&mut ProfileUserRelationWriteBuilder),
    ) -> ::saola_core::Query<ProfileMarker, ::saola_core::Upsert, Profile> {
        let mut query = ::saola_core::Query::new("upsertOne").with_provider(self.provider.clone());
        let mut data_map = ::saola_core::IndexMap::new();
        {
            let mut rel_builder = ProfileUserRelationWriteBuilder::default();
            user(&mut rel_builder);
            data_map.insert(
                "user".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(rel_builder.data),
            );
        }
        query.state.arguments.insert(
            "create".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(data_map),
        );
        query
    }
}
