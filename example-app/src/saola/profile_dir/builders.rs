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
impl ::saola_core::builder::OrderByBuilderTrait for ProfileOrderByBuilder {
    fn build(self) -> Vec<::saola_core::ArgumentValue> {
        self.args
    }
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
    pub fn find_many(&self) -> ProfileManyReadBuilder<Profile> {
        ProfileManyReadBuilder {
            inner: ::saola_core::ReadBuilder::find_many(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_unique(&self) -> ProfileUniqueReadBuilder<Profile> {
        ProfileUniqueReadBuilder {
            inner: ::saola_core::ReadBuilder::find_unique(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_first(&self) -> ProfileFirstReadBuilder<Profile> {
        ProfileFirstReadBuilder {
            inner: ::saola_core::ReadBuilder::find_first(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_unique_or_throw(&self) -> ProfileUniqueOrThrowReadBuilder<Profile> {
        ProfileUniqueOrThrowReadBuilder {
            inner: ::saola_core::ReadBuilder::find_unique_or_throw(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_first_or_throw(&self) -> ProfileFirstOrThrowReadBuilder<Profile> {
        ProfileFirstOrThrowReadBuilder {
            inner: ::saola_core::ReadBuilder::find_first_or_throw(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn create(&self, user: impl FnOnce(&mut ProfileUserRelationWriteBuilder)) -> ProfileWriteBuilder<Profile> {
        let mut inner = ::saola_core::WriteBuilder::create(
            ProfileMarker::NAME.to_string(),
            ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
        )
        .with_provider(self.provider.clone());
        let mut data_map = ::saola_core::IndexMap::new();
        {
            let mut rel_builder = ProfileUserRelationWriteBuilder::default();
            user(&mut rel_builder);
            data_map.insert(
                "user".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(rel_builder.data),
            );
        }
        inner.state.arguments.insert(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(data_map),
        );
        ProfileWriteBuilder {
            inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn update(&self) -> ProfileWriteBuilder<Profile> {
        ProfileWriteBuilder {
            inner: ::saola_core::WriteBuilder::update(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn delete(&self) -> ProfileWriteBuilder<Profile> {
        ProfileWriteBuilder {
            inner: ::saola_core::WriteBuilder::delete(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn upsert(&self, user: impl FnOnce(&mut ProfileUserRelationWriteBuilder)) -> ProfileUpsertBuilder {
        let mut inner = ::saola_core::WriteBuilder::upsert(
            ProfileMarker::NAME.to_string(),
            ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
        )
        .with_provider(self.provider.clone());
        let mut data_map = ::saola_core::IndexMap::new();
        {
            let mut rel_builder = ProfileUserRelationWriteBuilder::default();
            user(&mut rel_builder);
            data_map.insert(
                "user".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(rel_builder.data),
            );
        }
        inner.state.arguments.insert(
            "create".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(data_map),
        );
        ProfileUpsertBuilder { inner }
    }
    pub fn create_many(&self) -> ProfileCreateManyBuilder {
        ProfileCreateManyBuilder {
            inner: ::saola_core::CreateManyBuilder::new(ProfileMarker::NAME.to_string())
                .with_provider(self.provider.clone()),
        }
    }
    pub fn create_many_and_return(&self) -> ProfileCreateManyAndReturnBuilder {
        ProfileCreateManyAndReturnBuilder {
            inner: ::saola_core::CreateManyAndReturnBuilder::new(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn update_many(&self) -> ProfileUpdateManyBuilder {
        ProfileUpdateManyBuilder {
            inner: ::saola_core::UpdateManyBuilder::new(ProfileMarker::NAME.to_string())
                .with_provider(self.provider.clone()),
        }
    }
    pub fn update_many_and_return(&self) -> ProfileUpdateManyAndReturnBuilder {
        ProfileUpdateManyAndReturnBuilder {
            inner: ::saola_core::UpdateManyAndReturnBuilder::new(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn delete_many(&self) -> ProfileDeleteManyBuilder {
        ProfileDeleteManyBuilder {
            inner: ::saola_core::DeleteManyBuilder::new(ProfileMarker::NAME.to_string())
                .with_provider(self.provider.clone()),
        }
    }
    pub fn count(&self) -> ProfileCountBuilder {
        ProfileCountBuilder {
            inner: ::saola_core::CountBuilder::new(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
        }
    }
    pub fn aggregate(&self) -> ProfileAggregateBuilder {
        ProfileAggregateBuilder {
            inner: ::saola_core::AggregateBuilder::new(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
        }
    }
    pub fn group_by(&self) -> ProfileGroupByBuilder {
        ProfileGroupByBuilder {
            inner: ::saola_core::GroupByBuilder::new(
                ProfileMarker::NAME.to_string(),
                ProfileMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
        }
    }
}
pub trait ProfileIncludeMarker {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection>;
}
pub trait ProfileIncludeTransition<M> {
    type Output: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default;
}
pub struct ProfileIncludeEmpty;
impl ProfileIncludeMarker for ProfileIncludeEmpty {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        None
    }
}
pub struct ProfileIncludeUser {
    pub selection: ::saola_core::query_core::Selection,
}
impl ProfileIncludeMarker for ProfileIncludeUser {
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
pub struct ProfileIncludeUserWith<M> {
    pub selection: ::saola_core::query_core::Selection,
    pub _phantom: std::marker::PhantomData<M>,
}
impl<M> ProfileIncludeMarker for ProfileIncludeUserWith<M> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl<M> ProfileIncludeUserWith<M> {
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
impl<U> ProfileIncludeMarker for ProfileIncludeUserAs<U> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl<T0> ProfileIncludeTransition<ProfileIncludeUser> for ProfileData<T0>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = ProfileData<Box<UserData>>;
}
impl<T0, M> ProfileIncludeTransition<ProfileIncludeUserWith<M>> for ProfileData<T0>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    UserData: UserIncludeTransition<M>,
{
    type Output = ProfileData<Box<<UserData as UserIncludeTransition<M>>::Output>>;
}
impl<T0, U: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default>
    ProfileIncludeTransition<ProfileIncludeUserAs<U>> for ProfileData<T0>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = ProfileData<U>;
}
impl<T0> ProfileIncludeTransition<ProfileIncludeEmpty> for ProfileData<T0>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = ProfileData<T0>;
}
impl ProfileIncludeTransition<ProfileIncludeEmpty> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl ProfileIncludeTransition<ProfileIncludeUser> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl<M> ProfileIncludeTransition<ProfileIncludeUserWith<M>> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl<U> ProfileIncludeTransition<ProfileIncludeUserAs<U>> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
#[derive(Default)]
pub struct ProfileIncludeBuilder {
    pub args: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl ProfileIncludeBuilder {
    pub fn scalar_selections() -> Vec<::saola_core::query_core::Selection> {
        vec![
            ::saola_core::query_core::Selection::with_name("id".to_string()),
            ::saola_core::query_core::Selection::with_name("bio".to_string()),
            ::saola_core::query_core::Selection::with_name("userId".to_string()),
        ]
    }
    pub fn where_clause<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
        F: FnOnce(&mut ProfileOrderByBuilder),
    {
        let mut builder = ProfileOrderByBuilder::default();
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
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        let mut builder = ProfileUniqueWhereBuilder::default();
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
    pub fn empty(&mut self) -> ProfileIncludeEmpty {
        ProfileIncludeEmpty
    }
    pub fn user(&mut self) -> ProfileIncludeUser {
        let mut builder = UserSelectBuilder::default();
        builder.all();
        let mut sel = ::saola_core::query_core::Selection::with_name("user".to_string());
        let fields: Vec<::saola_core::query_core::Selection> = builder.into();
        for f in fields {
            sel.push_nested_selection(f);
        }
        ProfileIncludeUser { selection: sel }
    }
    pub fn user_with<M, F>(&mut self, f: F) -> ProfileIncludeUserWith<M>
    where
        F: FnOnce(&mut UserIncludeBuilder) -> M,
        M: UserIncludeMarker,
    {
        let mut builder = UserIncludeBuilder::default();
        let marker = f(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("user".to_string());
        for scalar in UserIncludeBuilder::scalar_selections() {
            sel.push_nested_selection(scalar);
        }
        if let Some(nested) = marker.into_selection() {
            sel.push_nested_selection(nested);
        }
        for (k, v) in std::mem::take(&mut builder.args) {
            sel.push_argument(k, v);
        }
        ProfileIncludeUserWith {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn user_as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync, F>(
        &mut self,
        selection: (std::marker::PhantomData<U>, F),
    ) -> ProfileIncludeUserAs<U>
    where
        F: FnOnce(&mut UserSelectBuilder),
    {
        let mut builder = UserSelectBuilder::default();
        (selection.1)(&mut builder);
        let mut sel = ::saola_core::query_core::Selection::with_name("user".to_string());
        let fields: Vec<::saola_core::query_core::Selection> = builder.into();
        for f in fields {
            sel.push_nested_selection(f);
        }
        ProfileIncludeUserAs {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl ProfileIncludeMarker for ProfileIncludeBuilder {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        None
    }
}
pub struct ProfileManyReadBuilder<T = Profile> {
    pub inner: ::saola_core::ReadBuilder<Vec<T>>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    ProfileManyReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn order_by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileOrderByBuilder),
    {
        let mut builder = ProfileOrderByBuilder::default();
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
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        let mut builder = ProfileUniqueWhereBuilder::default();
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
    pub fn select<F>(mut self, f: F) -> ProfileManyReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut ProfileSelectBuilder),
    {
        let mut builder = ProfileSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        ProfileManyReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> ProfileManyReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, Vec<U>>();
        ProfileManyReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> ProfileManyReadBuilder<<T as ProfileIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut ProfileIncludeBuilder) -> M,
        M: ProfileIncludeMarker,
        T: ProfileIncludeTransition<M>,
    {
        let mut builder = ProfileIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "bio", "userId"] {
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
        ProfileManyReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> ProfileManyReadBuilder<U> {
        ProfileManyReadBuilder {
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
pub struct ProfileUniqueReadBuilder<T = Profile> {
    pub inner: ::saola_core::ReadBuilder<Option<T>>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    ProfileUniqueReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        let mut builder = ProfileUniqueWhereBuilder::default();
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
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn select<F>(mut self, f: F) -> ProfileUniqueReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut ProfileSelectBuilder),
    {
        let mut builder = ProfileSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        ProfileUniqueReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> ProfileUniqueReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, Option<U>>();
        ProfileUniqueReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> ProfileUniqueReadBuilder<<T as ProfileIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut ProfileIncludeBuilder) -> M,
        M: ProfileIncludeMarker,
        T: ProfileIncludeTransition<M>,
    {
        let mut builder = ProfileIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "bio", "userId"] {
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
        ProfileUniqueReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> ProfileUniqueReadBuilder<U> {
        ProfileUniqueReadBuilder {
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
pub struct ProfileFirstReadBuilder<T = Profile> {
    pub inner: ::saola_core::ReadBuilder<Option<T>>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    ProfileFirstReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn order_by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileOrderByBuilder),
    {
        let mut builder = ProfileOrderByBuilder::default();
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
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        let mut builder = ProfileUniqueWhereBuilder::default();
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
    pub fn select<F>(mut self, f: F) -> ProfileFirstReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut ProfileSelectBuilder),
    {
        let mut builder = ProfileSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        ProfileFirstReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> ProfileFirstReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, Option<U>>();
        ProfileFirstReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> ProfileFirstReadBuilder<<T as ProfileIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut ProfileIncludeBuilder) -> M,
        M: ProfileIncludeMarker,
        T: ProfileIncludeTransition<M>,
    {
        let mut builder = ProfileIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "bio", "userId"] {
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
        ProfileFirstReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> ProfileFirstReadBuilder<U> {
        ProfileFirstReadBuilder {
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
pub struct ProfileUniqueOrThrowReadBuilder<T = Profile> {
    pub inner: ::saola_core::ReadBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    ProfileUniqueOrThrowReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        let mut builder = ProfileUniqueWhereBuilder::default();
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
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn select<F>(mut self, f: F) -> ProfileUniqueOrThrowReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut ProfileSelectBuilder),
    {
        let mut builder = ProfileSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        ProfileUniqueOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> ProfileUniqueOrThrowReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        ProfileUniqueOrThrowReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> ProfileUniqueOrThrowReadBuilder<<T as ProfileIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut ProfileIncludeBuilder) -> M,
        M: ProfileIncludeMarker,
        T: ProfileIncludeTransition<M>,
    {
        let mut builder = ProfileIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "bio", "userId"] {
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
        ProfileUniqueOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(
        self,
    ) -> ProfileUniqueOrThrowReadBuilder<U> {
        ProfileUniqueOrThrowReadBuilder {
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
pub struct ProfileFirstOrThrowReadBuilder<T = Profile> {
    pub inner: ::saola_core::ReadBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    ProfileFirstOrThrowReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn order_by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileOrderByBuilder),
    {
        let mut builder = ProfileOrderByBuilder::default();
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
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        let mut builder = ProfileUniqueWhereBuilder::default();
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
    pub fn select<F>(mut self, f: F) -> ProfileFirstOrThrowReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut ProfileSelectBuilder),
    {
        let mut builder = ProfileSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        ProfileFirstOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> ProfileFirstOrThrowReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        ProfileFirstOrThrowReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> ProfileFirstOrThrowReadBuilder<<T as ProfileIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut ProfileIncludeBuilder) -> M,
        M: ProfileIncludeMarker,
        T: ProfileIncludeTransition<M>,
    {
        let mut builder = ProfileIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "bio", "userId"] {
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
        ProfileFirstOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> ProfileFirstOrThrowReadBuilder<U> {
        ProfileFirstOrThrowReadBuilder {
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
pub struct ProfileWriteBuilder<T = Profile> {
    pub inner: ::saola_core::WriteBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    ProfileWriteBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        let mut builder = ProfileUniqueWhereBuilder::default();
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
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileDataBuilder),
    {
        let mut builder = ProfileDataBuilder::default();
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
    pub fn select<F>(mut self, f: F) -> ProfileWriteBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut ProfileSelectBuilder),
    {
        let mut builder = ProfileSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        ProfileWriteBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> ProfileWriteBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        ProfileWriteBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> ProfileWriteBuilder<<T as ProfileIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut ProfileIncludeBuilder) -> M,
        M: ProfileIncludeMarker,
        T: ProfileIncludeTransition<M>,
    {
        let mut builder = ProfileIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "bio", "userId"] {
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
        ProfileWriteBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> ProfileWriteBuilder<U> {
        ProfileWriteBuilder {
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
pub struct ProfileUpsertBuilder {
    pub inner: ::saola_core::WriteBuilder<Profile>,
}
impl ProfileUpsertBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        let mut builder = ProfileUniqueWhereBuilder::default();
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
        F: FnOnce(&mut ProfileUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn update<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileDataBuilder),
    {
        let mut builder = ProfileDataBuilder::default();
        f(&mut builder);
        self.inner.add_filter_arg(
            "update".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
        );
        self
    }
    pub fn create<F>(mut self, user: impl FnOnce(&mut ProfileUserRelationWriteBuilder), f: F) -> Self
    where
        F: FnOnce(&mut ProfileDataBuilder),
    {
        let mut create_builder = ProfileDataBuilder::default();
        {
            let mut rel_builder = ProfileUserRelationWriteBuilder::default();
            user(&mut rel_builder);
            create_builder.data.insert(
                "user".to_string(),
                ::saola_core::query_core::ArgumentValue::Object(rel_builder.data),
            );
        }
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
    ) -> ::saola_core::Result<Profile> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<Profile> {
        self.inner.exec().await
    }
}
pub struct ProfileCreateManyBuilder {
    pub inner: ::saola_core::CreateManyBuilder,
}
impl ProfileCreateManyBuilder {
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileScalarDataBuilder),
    {
        let mut builder = ProfileScalarDataBuilder::default();
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
pub struct ProfileCreateManyAndReturnBuilder<T = Profile> {
    pub inner: ::saola_core::CreateManyAndReturnBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    ProfileCreateManyAndReturnBuilder<T>
{
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileScalarDataBuilder),
    {
        let mut builder = ProfileScalarDataBuilder::default();
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
    pub fn select<F>(mut self, f: F) -> ProfileCreateManyAndReturnBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut ProfileSelectBuilder),
    {
        let mut builder = ProfileSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        ProfileCreateManyAndReturnBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> ProfileCreateManyAndReturnBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        ProfileCreateManyAndReturnBuilder {
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
            for field in &["id", "bio", "userId"] {
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
            for field in &["id", "bio", "userId"] {
                use ::saola_core::Selectable;
                builder
                    .inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(field.to_string()));
            }
        }
        builder.inner.exec().await
    }
}
pub struct ProfileUpdateManyBuilder {
    pub inner: ::saola_core::UpdateManyBuilder,
}
impl ProfileUpdateManyBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
        F: FnOnce(&mut ProfileScalarDataBuilder),
    {
        let mut builder = ProfileScalarDataBuilder::default();
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
pub struct ProfileUpdateManyAndReturnBuilder<T = Profile> {
    pub inner: ::saola_core::UpdateManyAndReturnBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    ProfileUpdateManyAndReturnBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
        F: FnOnce(&mut ProfileScalarDataBuilder),
    {
        let mut builder = ProfileScalarDataBuilder::default();
        f(&mut builder);
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
        );
        self
    }
    pub fn select<F>(mut self, f: F) -> ProfileUpdateManyAndReturnBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut ProfileSelectBuilder),
    {
        let mut builder = ProfileSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        ProfileUpdateManyAndReturnBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> ProfileUpdateManyAndReturnBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        ProfileUpdateManyAndReturnBuilder {
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
            for field in &["id", "bio", "userId"] {
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
            for field in &["id", "bio", "userId"] {
                use ::saola_core::Selectable;
                builder
                    .inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(field.to_string()));
            }
        }
        builder.inner.exec().await
    }
}
pub struct ProfileDeleteManyBuilder {
    pub inner: ::saola_core::DeleteManyBuilder,
}
impl ProfileDeleteManyBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
pub struct ProfileCountBuilder {
    pub inner: ::saola_core::CountBuilder,
}
impl ProfileCountBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
pub struct ProfileAggregateBuilder {
    pub inner: ::saola_core::AggregateBuilder<ProfileAggregateResult>,
}
impl ProfileAggregateBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
        F: FnOnce(&mut ProfileCountAggregateSelectBuilder),
    {
        let mut builder = ProfileCountAggregateSelectBuilder::default();
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
        F: FnOnce(&mut ProfileSumAggregateSelectBuilder),
    {
        let mut builder = ProfileSumAggregateSelectBuilder::default();
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
        F: FnOnce(&mut ProfileAvgAggregateSelectBuilder),
    {
        let mut builder = ProfileAvgAggregateSelectBuilder::default();
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
        F: FnOnce(&mut ProfileMinAggregateSelectBuilder),
    {
        let mut builder = ProfileMinAggregateSelectBuilder::default();
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
        F: FnOnce(&mut ProfileMaxAggregateSelectBuilder),
    {
        let mut builder = ProfileMaxAggregateSelectBuilder::default();
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
    ) -> ::saola_core::Result<ProfileAggregateResult> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<ProfileAggregateResult> {
        self.inner.exec().await
    }
}
pub struct ProfileGroupByBuilder {
    pub inner: ::saola_core::GroupByBuilder<ProfileGroupByResult>,
}
impl ProfileGroupByBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
        F: FnOnce(&mut ProfileWhereBuilder),
    {
        let mut builder = ProfileWhereBuilder::default();
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
        F: FnOnce(&mut ProfileOrderByBuilder),
    {
        let mut builder = ProfileOrderByBuilder::default();
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
        F: FnOnce(&mut ProfileGroupBySelectBuilder),
    {
        let mut builder = ProfileGroupBySelectBuilder::default();
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
        F: FnOnce(&mut ProfileCountAggregateSelectBuilder),
    {
        let mut builder = ProfileCountAggregateSelectBuilder::default();
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
        F: FnOnce(&mut ProfileSumAggregateSelectBuilder),
    {
        let mut builder = ProfileSumAggregateSelectBuilder::default();
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
        F: FnOnce(&mut ProfileAvgAggregateSelectBuilder),
    {
        let mut builder = ProfileAvgAggregateSelectBuilder::default();
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
        F: FnOnce(&mut ProfileMinAggregateSelectBuilder),
    {
        let mut builder = ProfileMinAggregateSelectBuilder::default();
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
        F: FnOnce(&mut ProfileMaxAggregateSelectBuilder),
    {
        let mut builder = ProfileMaxAggregateSelectBuilder::default();
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
    ) -> ::saola_core::Result<Vec<ProfileGroupByResult>> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<Vec<ProfileGroupByResult>> {
        self.inner.exec().await
    }
}
