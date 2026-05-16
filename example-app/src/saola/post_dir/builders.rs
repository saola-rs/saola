use super::super::*;
use ::saola_core::prelude::*;
pub struct PostMarker;
impl ::saola_core::ModelMarker for PostMarker {
    type Data = PostData;
    type Where = PostWhereBuilder;
    type UniqueWhere = PostUniqueWhereBuilder;
    type OrderBy = PostOrderByBuilder;
    type Include = PostIncludeBuilder;
    type Select = PostSelectBuilder;
    type DataBuilder = PostDataBuilder;
    const NAME: &'static str = "Post";
    const SCALAR_FIELDS: &'static [&'static str] = &["id", "title", "content", "published", "userId", "createdAt"];
}
#[derive(Default)]
pub struct PostWhereBuilder {
    pub args: Vec<(String, ::saola_core::query_core::ArgumentValue)>,
}
impl ::saola_core::FilterBuilder for PostWhereBuilder {
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
impl PostWhereBuilder {
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
    pub fn title(&mut self) -> ::saola_core::StringFilter<'_, Self> {
        ::saola_core::StringFilter {
            builder: self,
            field_name: "title",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn content(&mut self) -> ::saola_core::StringFilter<'_, Self> {
        ::saola_core::StringFilter {
            builder: self,
            field_name: "content",
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn published(&mut self) -> ::saola_core::BoolFilter<'_, Self> {
        ::saola_core::BoolFilter {
            builder: self,
            field_name: "published",
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
    pub fn created_at(&mut self) -> ::saola_core::DateTimeFilter<'_, Self> {
        ::saola_core::DateTimeFilter {
            builder: self,
            field_name: "createdAt",
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
pub struct PostUniqueWhereBuilder {
    pub args: Vec<(String, ::saola_core::query_core::ArgumentValue)>,
}
impl ::saola_core::FilterBuilder for PostUniqueWhereBuilder {
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
impl PostUniqueWhereBuilder {
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
}
#[derive(Default)]
pub struct PostOrderByBuilder {
    pub args: Vec<::saola_core::ArgumentValue>,
}
impl ::saola_core::builder::OrderByBuilderTrait for PostOrderByBuilder {
    fn build(self) -> Vec<::saola_core::ArgumentValue> {
        self.args
    }
}
impl PostOrderByBuilder {
    pub fn id(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("id".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn title(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("title".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn content(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("content".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn published(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("published".to_string(), ::saola_core::ArgumentValue::from(order));
        self.args.push(::saola_core::ArgumentValue::Object(map));
        self
    }
    pub fn user_id(&mut self, order: ::saola_core::SortOrder) -> &mut Self {
        let mut map = ::saola_core::IndexMap::new();
        map.insert("userId".to_string(), ::saola_core::ArgumentValue::from(order));
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
pub struct PostSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl ::saola_core::Selectable for PostSelectBuilder {
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
impl PostSelectBuilder {
    pub fn all(&mut self) -> &mut Self {
        for field in &["id", "title", "content", "published", "userId", "createdAt"] {
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
    pub fn _validate_field_title(&self) {}
    pub fn title(&mut self) -> ::saola_core::SelectionField<'_, String, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("title".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_content(&self) {}
    pub fn content(&mut self) -> ::saola_core::SelectionField<'_, Option<String>, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("content".to_string()));
        ::saola_core::SelectionField::new(self)
    }
    #[allow(non_snake_case)]
    pub fn _validate_field_published(&self) {}
    pub fn published(&mut self) -> ::saola_core::SelectionField<'_, bool, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("published".to_string()));
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
    pub fn _validate_field_createdAt(&self) {}
    pub fn created_at(
        &mut self,
    ) -> ::saola_core::SelectionField<'_, ::saola_core::chrono::DateTime<::saola_core::chrono::Utc>, Self> {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("createdAt".to_string()));
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
impl From<PostSelectBuilder> for Vec<::saola_core::query_core::Selection> {
    fn from(b: PostSelectBuilder) -> Self {
        b.selections
    }
}
#[derive(Default)]
pub struct PostDataBuilder {
    pub data: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl ::saola_core::builder::DataBuilderTrait for PostDataBuilder {
    fn build(self) -> ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue> {
        self.data
    }
}
impl PostDataBuilder {
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
    pub fn title<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "title".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn content<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "content".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn published<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "published".to_string(),
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
    pub fn user<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut PostUserRelationWriteBuilder),
    {
        let mut builder = PostUserRelationWriteBuilder::default();
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
impl From<PostDataBuilder> for ::saola_core::query_structure::PrismaValue {
    fn from(_b: PostDataBuilder) -> Self {
        ::saola_core::query_structure::PrismaValue::Null
    }
}
#[derive(Default)]
pub struct PostScalarDataBuilder {
    pub data: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl ::saola_core::builder::DataBuilderTrait for PostScalarDataBuilder {
    fn build(self) -> ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue> {
        self.data
    }
}
impl PostScalarDataBuilder {
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
    pub fn title<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "title".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn content<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "content".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(value.into()),
        );
        self
    }
    pub fn published<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<::saola_core::query_structure::PrismaValue>,
    {
        self.data.insert(
            "published".to_string(),
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
pub struct PostUserRelationWriteBuilder {
    pub data: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl PostUserRelationWriteBuilder {
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
pub struct PostCountAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl PostCountAggregateSelectBuilder {
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
    pub fn title(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("title"));
        self
    }
    pub fn content(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("content"));
        self
    }
    pub fn published(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("published"));
        self
    }
    pub fn user_id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("userId"));
        self
    }
    pub fn created_at(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("createdAt"));
        self
    }
}
#[derive(Default)]
pub struct PostSumAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl PostSumAggregateSelectBuilder {}
#[derive(Default)]
pub struct PostAvgAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl PostAvgAggregateSelectBuilder {}
#[derive(Default)]
pub struct PostMinAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl PostMinAggregateSelectBuilder {
    pub fn id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("id"));
        self
    }
    pub fn title(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("title"));
        self
    }
    pub fn content(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("content"));
        self
    }
    pub fn published(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("published"));
        self
    }
    pub fn user_id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("userId"));
        self
    }
    pub fn created_at(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("createdAt"));
        self
    }
}
#[derive(Default)]
pub struct PostMaxAggregateSelectBuilder {
    pub selections: Vec<::saola_core::query_core::Selection>,
}
impl PostMaxAggregateSelectBuilder {
    pub fn id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("id"));
        self
    }
    pub fn title(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("title"));
        self
    }
    pub fn content(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("content"));
        self
    }
    pub fn published(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("published"));
        self
    }
    pub fn user_id(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("userId"));
        self
    }
    pub fn created_at(&mut self) -> &mut Self {
        self.selections
            .push(::saola_core::query_core::Selection::with_name("createdAt"));
        self
    }
}
#[derive(Default)]
pub struct PostGroupBySelectBuilder {
    pub fields: Vec<String>,
}
impl PostGroupBySelectBuilder {
    pub fn id(&mut self) -> &mut Self {
        self.fields.push("id".to_string());
        self
    }
    pub fn title(&mut self) -> &mut Self {
        self.fields.push("title".to_string());
        self
    }
    pub fn content(&mut self) -> &mut Self {
        self.fields.push("content".to_string());
        self
    }
    pub fn published(&mut self) -> &mut Self {
        self.fields.push("published".to_string());
        self
    }
    pub fn user_id(&mut self) -> &mut Self {
        self.fields.push("userId".to_string());
        self
    }
    pub fn created_at(&mut self) -> &mut Self {
        self.fields.push("createdAt".to_string());
        self
    }
}
pub struct PostQuery {
    pub provider: std::sync::Arc<dyn ::saola_core::transaction::QueryExecutorProvider>,
}
impl PostQuery {
    pub fn find_many(&self) -> PostManyReadBuilder<Post> {
        PostManyReadBuilder {
            inner: ::saola_core::ReadBuilder::find_many(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_unique(&self) -> PostUniqueReadBuilder<Post> {
        PostUniqueReadBuilder {
            inner: ::saola_core::ReadBuilder::find_unique(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_first(&self) -> PostFirstReadBuilder<Post> {
        PostFirstReadBuilder {
            inner: ::saola_core::ReadBuilder::find_first(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_unique_or_throw(&self) -> PostUniqueOrThrowReadBuilder<Post> {
        PostUniqueOrThrowReadBuilder {
            inner: ::saola_core::ReadBuilder::find_unique_or_throw(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn find_first_or_throw(&self) -> PostFirstOrThrowReadBuilder<Post> {
        PostFirstOrThrowReadBuilder {
            inner: ::saola_core::ReadBuilder::find_first_or_throw(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn create(
        &self,
        title: String,
        user: impl FnOnce(&mut PostUserRelationWriteBuilder),
    ) -> PostWriteBuilder<Post> {
        let mut inner = ::saola_core::WriteBuilder::create(
            PostMarker::NAME.to_string(),
            PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
        )
        .with_provider(self.provider.clone());
        let mut data_map = ::saola_core::IndexMap::new();
        data_map.insert(
            "title".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(title)),
        );
        {
            let mut rel_builder = PostUserRelationWriteBuilder::default();
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
        PostWriteBuilder {
            inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn update(&self) -> PostWriteBuilder<Post> {
        PostWriteBuilder {
            inner: ::saola_core::WriteBuilder::update(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn delete(&self) -> PostWriteBuilder<Post> {
        PostWriteBuilder {
            inner: ::saola_core::WriteBuilder::delete(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn upsert(&self, title: String, user: impl FnOnce(&mut PostUserRelationWriteBuilder)) -> PostUpsertBuilder {
        let mut inner = ::saola_core::WriteBuilder::upsert(
            PostMarker::NAME.to_string(),
            PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
        )
        .with_provider(self.provider.clone());
        let mut data_map = ::saola_core::IndexMap::new();
        data_map.insert(
            "title".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(title)),
        );
        {
            let mut rel_builder = PostUserRelationWriteBuilder::default();
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
        PostUpsertBuilder { inner }
    }
    pub fn create_many(&self) -> PostCreateManyBuilder {
        PostCreateManyBuilder {
            inner: ::saola_core::CreateManyBuilder::new(PostMarker::NAME.to_string())
                .with_provider(self.provider.clone()),
        }
    }
    pub fn create_many_and_return(&self) -> PostCreateManyAndReturnBuilder {
        PostCreateManyAndReturnBuilder {
            inner: ::saola_core::CreateManyAndReturnBuilder::new(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn update_many(&self) -> PostUpdateManyBuilder {
        PostUpdateManyBuilder {
            inner: ::saola_core::UpdateManyBuilder::new(PostMarker::NAME.to_string())
                .with_provider(self.provider.clone()),
        }
    }
    pub fn update_many_and_return(&self) -> PostUpdateManyAndReturnBuilder {
        PostUpdateManyAndReturnBuilder {
            inner: ::saola_core::UpdateManyAndReturnBuilder::new(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn delete_many(&self) -> PostDeleteManyBuilder {
        PostDeleteManyBuilder {
            inner: ::saola_core::DeleteManyBuilder::new(PostMarker::NAME.to_string())
                .with_provider(self.provider.clone()),
        }
    }
    pub fn count(&self) -> PostCountBuilder {
        PostCountBuilder {
            inner: ::saola_core::CountBuilder::new(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
        }
    }
    pub fn aggregate(&self) -> PostAggregateBuilder {
        PostAggregateBuilder {
            inner: ::saola_core::AggregateBuilder::new(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
        }
    }
    pub fn group_by(&self) -> PostGroupByBuilder {
        PostGroupByBuilder {
            inner: ::saola_core::GroupByBuilder::new(
                PostMarker::NAME.to_string(),
                PostMarker::SCALAR_FIELDS.iter().map(|s| s.to_string()).collect(),
            )
            .with_provider(self.provider.clone()),
        }
    }
}
pub trait PostIncludeMarker {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection>;
}
pub trait PostIncludeTransition<M> {
    type Output: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default;
}
pub struct PostIncludeEmpty;
impl PostIncludeMarker for PostIncludeEmpty {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        None
    }
}
pub struct PostIncludeUser {
    pub selection: ::saola_core::query_core::Selection,
}
impl PostIncludeMarker for PostIncludeUser {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl PostIncludeUser {
    pub fn model_as<U: ::saola_core::builder::SelectStruct>(mut self) -> PostIncludeUserAs<U> {
        self.selection.clear_nested_selections();
        for sel in U::selections() {
            self.selection.push_nested_selection(sel);
        }
        PostIncludeUserAs {
            selection: self.selection,
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct PostIncludeUserWith<M> {
    pub selection: ::saola_core::query_core::Selection,
    pub _phantom: std::marker::PhantomData<M>,
}
impl<M> PostIncludeMarker for PostIncludeUserWith<M> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl<M> PostIncludeUserWith<M> {
    pub fn model_as<U: ::saola_core::builder::SelectStruct>(mut self) -> PostIncludeUserAs<U> {
        self.selection.clear_nested_selections();
        for sel in U::selections() {
            self.selection.push_nested_selection(sel);
        }
        PostIncludeUserAs {
            selection: self.selection,
            _phantom: std::marker::PhantomData,
        }
    }
}
pub struct PostIncludeUserAs<U> {
    pub selection: ::saola_core::query_core::Selection,
    pub _phantom: std::marker::PhantomData<U>,
}
impl<U> PostIncludeMarker for PostIncludeUserAs<U> {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        Some(self.selection)
    }
}
impl<T0> PostIncludeTransition<PostIncludeUser> for PostData<T0>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = PostData<Box<UserData>>;
}
impl<T0, M> PostIncludeTransition<PostIncludeUserWith<M>> for PostData<T0>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
    UserData: UserIncludeTransition<M>,
{
    type Output = PostData<Box<<UserData as UserIncludeTransition<M>>::Output>>;
}
impl<T0, U: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default>
    PostIncludeTransition<PostIncludeUserAs<U>> for PostData<T0>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = PostData<U>;
}
impl<T0> PostIncludeTransition<PostIncludeEmpty> for PostData<T0>
where
    T0: ::saola_core::serde::de::DeserializeOwned + Send + Sync + Default,
{
    type Output = PostData<T0>;
}
impl PostIncludeTransition<PostIncludeEmpty> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl PostIncludeTransition<PostIncludeUser> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl<M> PostIncludeTransition<PostIncludeUserWith<M>> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
impl<U> PostIncludeTransition<PostIncludeUserAs<U>> for ::saola_core::serde_json::Value {
    type Output = ::saola_core::serde_json::Value;
}
#[derive(Default)]
pub struct PostIncludeBuilder {
    pub args: ::saola_core::IndexMap<String, ::saola_core::query_core::ArgumentValue>,
}
impl PostIncludeBuilder {
    pub fn scalar_selections() -> Vec<::saola_core::query_core::Selection> {
        vec![
            ::saola_core::query_core::Selection::with_name("id".to_string()),
            ::saola_core::query_core::Selection::with_name("title".to_string()),
            ::saola_core::query_core::Selection::with_name("content".to_string()),
            ::saola_core::query_core::Selection::with_name("published".to_string()),
            ::saola_core::query_core::Selection::with_name("userId".to_string()),
            ::saola_core::query_core::Selection::with_name("createdAt".to_string()),
        ]
    }
    pub fn where_clause<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
        F: FnOnce(&mut PostOrderByBuilder),
    {
        let mut builder = PostOrderByBuilder::default();
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
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
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
    pub fn empty(&mut self) -> PostIncludeEmpty {
        PostIncludeEmpty
    }
    pub fn user(&mut self) -> PostIncludeUser {
        let mut builder = UserSelectBuilder::default();
        builder.all();
        let mut sel = ::saola_core::query_core::Selection::with_name("user".to_string());
        let fields: Vec<::saola_core::query_core::Selection> = builder.into();
        for f in fields {
            sel.push_nested_selection(f);
        }
        PostIncludeUser { selection: sel }
    }
    pub fn user_with<M, F>(&mut self, f: F) -> PostIncludeUserWith<M>
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
        PostIncludeUserWith {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn user_as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync, F>(
        &mut self,
        selection: (std::marker::PhantomData<U>, F),
    ) -> PostIncludeUserAs<U>
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
        PostIncludeUserAs {
            selection: sel,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl PostIncludeMarker for PostIncludeBuilder {
    fn into_selection(self) -> Option<::saola_core::query_core::Selection> {
        None
    }
}
pub struct PostManyReadBuilder<T = Post> {
    pub inner: ::saola_core::ReadBuilder<Vec<T>>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    PostManyReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
        F: FnOnce(&mut PostWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn order_by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostOrderByBuilder),
    {
        let mut builder = PostOrderByBuilder::default();
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
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
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
    pub fn select<F>(mut self, f: F) -> PostManyReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut builder = PostSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        PostManyReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> PostManyReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, Vec<U>>();
        PostManyReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> PostManyReadBuilder<<T as PostIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut PostIncludeBuilder) -> M,
        M: PostIncludeMarker,
        T: PostIncludeTransition<M>,
    {
        let mut builder = PostIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "title", "content", "published", "userId", "createdAt"] {
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
        PostManyReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> PostManyReadBuilder<U> {
        PostManyReadBuilder {
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
pub struct PostUniqueReadBuilder<T = Post> {
    pub inner: ::saola_core::ReadBuilder<Option<T>>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    PostUniqueReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
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
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn select<F>(mut self, f: F) -> PostUniqueReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut builder = PostSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        PostUniqueReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> PostUniqueReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, Option<U>>();
        PostUniqueReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> PostUniqueReadBuilder<<T as PostIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut PostIncludeBuilder) -> M,
        M: PostIncludeMarker,
        T: PostIncludeTransition<M>,
    {
        let mut builder = PostIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "title", "content", "published", "userId", "createdAt"] {
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
        PostUniqueReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> PostUniqueReadBuilder<U> {
        PostUniqueReadBuilder {
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
pub struct PostFirstReadBuilder<T = Post> {
    pub inner: ::saola_core::ReadBuilder<Option<T>>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    PostFirstReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
        F: FnOnce(&mut PostWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn order_by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostOrderByBuilder),
    {
        let mut builder = PostOrderByBuilder::default();
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
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
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
    pub fn select<F>(mut self, f: F) -> PostFirstReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut builder = PostSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        PostFirstReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> PostFirstReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, Option<U>>();
        PostFirstReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> PostFirstReadBuilder<<T as PostIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut PostIncludeBuilder) -> M,
        M: PostIncludeMarker,
        T: PostIncludeTransition<M>,
    {
        let mut builder = PostIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "title", "content", "published", "userId", "createdAt"] {
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
        PostFirstReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> PostFirstReadBuilder<U> {
        PostFirstReadBuilder {
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
pub struct PostUniqueOrThrowReadBuilder<T = Post> {
    pub inner: ::saola_core::ReadBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    PostUniqueOrThrowReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
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
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn select<F>(mut self, f: F) -> PostUniqueOrThrowReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut builder = PostSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        PostUniqueOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> PostUniqueOrThrowReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        PostUniqueOrThrowReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> PostUniqueOrThrowReadBuilder<<T as PostIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut PostIncludeBuilder) -> M,
        M: PostIncludeMarker,
        T: PostIncludeTransition<M>,
    {
        let mut builder = PostIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "title", "content", "published", "userId", "createdAt"] {
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
        PostUniqueOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> PostUniqueOrThrowReadBuilder<U> {
        PostUniqueOrThrowReadBuilder {
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
pub struct PostFirstOrThrowReadBuilder<T = Post> {
    pub inner: ::saola_core::ReadBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    PostFirstOrThrowReadBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
        F: FnOnce(&mut PostWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn order_by<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostOrderByBuilder),
    {
        let mut builder = PostOrderByBuilder::default();
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
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
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
    pub fn select<F>(mut self, f: F) -> PostFirstOrThrowReadBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut builder = PostSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        PostFirstOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> PostFirstOrThrowReadBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        PostFirstOrThrowReadBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> PostFirstOrThrowReadBuilder<<T as PostIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut PostIncludeBuilder) -> M,
        M: PostIncludeMarker,
        T: PostIncludeTransition<M>,
    {
        let mut builder = PostIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "title", "content", "published", "userId", "createdAt"] {
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
        PostFirstOrThrowReadBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> PostFirstOrThrowReadBuilder<U> {
        PostFirstOrThrowReadBuilder {
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
pub struct PostWriteBuilder<T = Post> {
    pub inner: ::saola_core::WriteBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    PostWriteBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
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
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostDataBuilder),
    {
        let mut builder = PostDataBuilder::default();
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
    pub fn select<F>(mut self, f: F) -> PostWriteBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut builder = PostSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        PostWriteBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> PostWriteBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        PostWriteBuilder {
            inner: new_inner,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn include<M, F>(mut self, f: F) -> PostWriteBuilder<<T as PostIncludeTransition<M>>::Output>
    where
        F: FnOnce(&mut PostIncludeBuilder) -> M,
        M: PostIncludeMarker,
        T: PostIncludeTransition<M>,
    {
        let mut builder = PostIncludeBuilder::default();
        let marker = f(&mut builder);
        use ::saola_core::Selectable;
        if self.inner.state.selection.nested_selections().is_empty() {
            for scalar_field in &["id", "title", "content", "published", "userId", "createdAt"] {
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
        PostWriteBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn r#as<U: ::saola_core::serde::de::DeserializeOwned + Send + Sync>(self) -> PostWriteBuilder<U> {
        PostWriteBuilder {
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
pub struct PostUpsertBuilder {
    pub inner: ::saola_core::WriteBuilder<Post>,
}
impl PostUpsertBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        let mut builder = PostUniqueWhereBuilder::default();
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
        F: FnOnce(&mut PostUniqueWhereBuilder),
    {
        self.where_clause(f)
    }
    pub fn update<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostDataBuilder),
    {
        let mut builder = PostDataBuilder::default();
        f(&mut builder);
        self.inner.add_filter_arg(
            "update".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
        );
        self
    }
    pub fn create<F>(mut self, title: String, user: impl FnOnce(&mut PostUserRelationWriteBuilder), f: F) -> Self
    where
        F: FnOnce(&mut PostDataBuilder),
    {
        let mut create_builder = PostDataBuilder::default();
        create_builder.data.insert(
            "title".to_string(),
            ::saola_core::query_core::ArgumentValue::Scalar(::saola_core::query_structure::PrismaValue::from(title)),
        );
        {
            let mut rel_builder = PostUserRelationWriteBuilder::default();
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
    ) -> ::saola_core::Result<Post> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<Post> {
        self.inner.exec().await
    }
}
pub struct PostCreateManyBuilder {
    pub inner: ::saola_core::CreateManyBuilder,
}
impl PostCreateManyBuilder {
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostScalarDataBuilder),
    {
        let mut builder = PostScalarDataBuilder::default();
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
pub struct PostCreateManyAndReturnBuilder<T = Post> {
    pub inner: ::saola_core::CreateManyAndReturnBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    PostCreateManyAndReturnBuilder<T>
{
    pub fn data<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostScalarDataBuilder),
    {
        let mut builder = PostScalarDataBuilder::default();
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
    pub fn select<F>(mut self, f: F) -> PostCreateManyAndReturnBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut builder = PostSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        PostCreateManyAndReturnBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> PostCreateManyAndReturnBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        PostCreateManyAndReturnBuilder {
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
            for field in &["id", "title", "content", "published", "userId", "createdAt"] {
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
            for field in &["id", "title", "content", "published", "userId", "createdAt"] {
                use ::saola_core::Selectable;
                builder
                    .inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(field.to_string()));
            }
        }
        builder.inner.exec().await
    }
}
pub struct PostUpdateManyBuilder {
    pub inner: ::saola_core::UpdateManyBuilder,
}
impl PostUpdateManyBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
        F: FnOnce(&mut PostScalarDataBuilder),
    {
        let mut builder = PostScalarDataBuilder::default();
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
pub struct PostUpdateManyAndReturnBuilder<T = Post> {
    pub inner: ::saola_core::UpdateManyAndReturnBuilder<T>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<T: ::saola_core::serde::de::DeserializeOwned + Send + Sync + ::saola_core::builder::FromResponseIr>
    PostUpdateManyAndReturnBuilder<T>
{
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
        F: FnOnce(&mut PostScalarDataBuilder),
    {
        let mut builder = PostScalarDataBuilder::default();
        f(&mut builder);
        use ::saola_core::Filterable;
        self.inner.add_filter_arg(
            "data".to_string(),
            ::saola_core::query_core::ArgumentValue::Object(std::mem::take(&mut builder.data)),
        );
        self
    }
    pub fn select<F>(mut self, f: F) -> PostUpdateManyAndReturnBuilder<::saola_core::serde_json::Value>
    where
        F: FnOnce(&mut PostSelectBuilder),
    {
        let mut builder = PostSelectBuilder::default();
        f(&mut builder);
        let selections: Vec<::saola_core::query_core::Selection> = builder.into();
        use ::saola_core::Selectable;
        self.inner.state.selection.clear_nested_selections();
        for sel in selections {
            self.inner.add_nested_selection(sel);
        }
        PostUpdateManyAndReturnBuilder {
            inner: self.inner.with_type(),
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn select_as<U: ::saola_core::builder::SelectStruct>(self) -> PostUpdateManyAndReturnBuilder<U> {
        let new_inner = self.inner.select_as::<U, U>();
        PostUpdateManyAndReturnBuilder {
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
            for field in &["id", "title", "content", "published", "userId", "createdAt"] {
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
            for field in &["id", "title", "content", "published", "userId", "createdAt"] {
                use ::saola_core::Selectable;
                builder
                    .inner
                    .add_nested_selection(::saola_core::query_core::Selection::with_name(field.to_string()));
            }
        }
        builder.inner.exec().await
    }
}
pub struct PostDeleteManyBuilder {
    pub inner: ::saola_core::DeleteManyBuilder,
}
impl PostDeleteManyBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
pub struct PostCountBuilder {
    pub inner: ::saola_core::CountBuilder,
}
impl PostCountBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
pub struct PostAggregateBuilder {
    pub inner: ::saola_core::AggregateBuilder<PostAggregateResult>,
}
impl PostAggregateBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
        F: FnOnce(&mut PostCountAggregateSelectBuilder),
    {
        let mut builder = PostCountAggregateSelectBuilder::default();
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
        F: FnOnce(&mut PostSumAggregateSelectBuilder),
    {
        let mut builder = PostSumAggregateSelectBuilder::default();
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
        F: FnOnce(&mut PostAvgAggregateSelectBuilder),
    {
        let mut builder = PostAvgAggregateSelectBuilder::default();
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
        F: FnOnce(&mut PostMinAggregateSelectBuilder),
    {
        let mut builder = PostMinAggregateSelectBuilder::default();
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
        F: FnOnce(&mut PostMaxAggregateSelectBuilder),
    {
        let mut builder = PostMaxAggregateSelectBuilder::default();
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
    ) -> ::saola_core::Result<PostAggregateResult> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<PostAggregateResult> {
        self.inner.exec().await
    }
}
pub struct PostGroupByBuilder {
    pub inner: ::saola_core::GroupByBuilder<PostGroupByResult>,
}
impl PostGroupByBuilder {
    pub fn where_clause<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
        F: FnOnce(&mut PostWhereBuilder),
    {
        let mut builder = PostWhereBuilder::default();
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
        F: FnOnce(&mut PostOrderByBuilder),
    {
        let mut builder = PostOrderByBuilder::default();
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
        F: FnOnce(&mut PostGroupBySelectBuilder),
    {
        let mut builder = PostGroupBySelectBuilder::default();
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
        F: FnOnce(&mut PostCountAggregateSelectBuilder),
    {
        let mut builder = PostCountAggregateSelectBuilder::default();
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
        F: FnOnce(&mut PostSumAggregateSelectBuilder),
    {
        let mut builder = PostSumAggregateSelectBuilder::default();
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
        F: FnOnce(&mut PostAvgAggregateSelectBuilder),
    {
        let mut builder = PostAvgAggregateSelectBuilder::default();
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
        F: FnOnce(&mut PostMinAggregateSelectBuilder),
    {
        let mut builder = PostMinAggregateSelectBuilder::default();
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
        F: FnOnce(&mut PostMaxAggregateSelectBuilder),
    {
        let mut builder = PostMaxAggregateSelectBuilder::default();
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
    ) -> ::saola_core::Result<Vec<PostGroupByResult>> {
        self.inner.exec_with(provider).await
    }
    pub async fn exec(self) -> ::saola_core::Result<Vec<PostGroupByResult>> {
        self.inner.exec().await
    }
}
