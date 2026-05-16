use super::super::enums;
use ::saola_core::serde;
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct PostData<User = ::saola_core::Unloaded> {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "content", default)]
    pub content: Option<String>,
    #[serde(rename = "published", default)]
    pub published: bool,
    #[serde(rename = "userId", default)]
    pub user_id: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: ::saola_core::chrono::DateTime<::saola_core::chrono::Utc>,
    #[serde(rename = "user", default)]
    pub user: User,
}
pub type Post = PostData;
impl ::saola_core::builder::SelectStruct for PostData {
    fn selections() -> Vec<::saola_core::query_core::Selection> {
        vec![
            ::saola_core::query_core::Selection::with_name("id".to_string()),
            ::saola_core::query_core::Selection::with_name("title".to_string()),
            ::saola_core::query_core::Selection::with_name("content".to_string()),
            ::saola_core::query_core::Selection::with_name("published".to_string()),
            ::saola_core::query_core::Selection::with_name("userId".to_string()),
            ::saola_core::query_core::Selection::with_name("createdAt".to_string()),
        ]
    }
}
impl ::saola_core::builder::GetSelections for PostData {
    fn get_selections() -> Vec<::saola_core::query_core::Selection> {
        <Self as ::saola_core::builder::SelectStruct>::selections()
    }
}
impl<User> ::saola_core::builder::FromResponseIr for PostData<User>
where
    User: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
{
    fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
        let mut map = match item {
            ::saola_core::query_core::response_ir::Item::Map(m) => m,
            ::saola_core::query_core::response_ir::Item::Ref(r) => match r.as_ref() {
                ::saola_core::query_core::response_ir::Item::Map(m) => m.clone(),
                _ => {
                    return Err(::saola_core::Error::RuntimeError(
                        "Expected map in response ref".to_string(),
                    ));
                }
            },
            _ => {
                return Err(::saola_core::Error::RuntimeError(format!(
                    "Expected map in response, got {:?}",
                    item
                )));
            }
        };
        Ok(Self {
            id: map
                .shift_remove("id")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            title: map
                .shift_remove("title")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            content: map
                .shift_remove("content")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            published: map
                .shift_remove("published")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            user_id: map
                .shift_remove("userId")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            created_at: map
                .shift_remove("createdAt")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            user: map
                .shift_remove("user")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
        })
    }
}
#[allow(dead_code)]
pub mod _post {
    #[allow(unused_imports)]
    pub use super::PostData;
    pub mod fields {
        use super::super::enums;
        pub const ID: ::saola_core::Field<String> = ::saola_core::Field::new("id");
        pub type id = String;
        pub const TITLE: ::saola_core::Field<String> = ::saola_core::Field::new("title");
        pub type title = String;
        pub const CONTENT: ::saola_core::Field<String> = ::saola_core::Field::new("content");
        pub type content = Option<String>;
        pub const PUBLISHED: ::saola_core::Field<bool> = ::saola_core::Field::new("published");
        pub type published = bool;
        pub const USER_ID: ::saola_core::Field<String> = ::saola_core::Field::new("userId");
        pub type user_id = String;
        pub const CREATED_AT: ::saola_core::Field<::saola_core::chrono::DateTime<::saola_core::chrono::Utc>> =
            ::saola_core::Field::new("createdAt");
        pub type created_at = ::saola_core::chrono::DateTime<::saola_core::chrono::Utc>;
        pub type user = ();
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct PostAggregateCount {
    #[serde(rename = "_all", default)]
    pub _all: i64,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub title: i64,
    #[serde(default)]
    pub content: i64,
    #[serde(default)]
    pub published: i64,
    #[serde(default)]
    pub user_id: i64,
    #[serde(default)]
    pub created_at: i64,
}
impl ::saola_core::builder::FromResponseIr for PostAggregateCount {
    fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
        let mut map = match item {
            ::saola_core::query_core::response_ir::Item::Map(m) => m,
            _ => return Err(::saola_core::Error::RuntimeError("Expected map".to_string())),
        };
        Ok(Self {
            _all: map
                .shift_remove("_all")
                .and_then(|i| i64::from_ir(i).ok())
                .unwrap_or_default(),
            id: map
                .shift_remove("id")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            title: map
                .shift_remove("title")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            content: map
                .shift_remove("content")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            published: map
                .shift_remove("published")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            user_id: map
                .shift_remove("userId")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            created_at: map
                .shift_remove("createdAt")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
        })
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct PostAggregateSum {}
impl ::saola_core::builder::FromResponseIr for PostAggregateSum {
    fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
        let mut map = match item {
            ::saola_core::query_core::response_ir::Item::Map(m) => m,
            _ => return Err(::saola_core::Error::RuntimeError("Expected map".to_string())),
        };
        Ok(Self {})
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct PostAggregateAvg {}
impl ::saola_core::builder::FromResponseIr for PostAggregateAvg {
    fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
        let mut map = match item {
            ::saola_core::query_core::response_ir::Item::Map(m) => m,
            _ => return Err(::saola_core::Error::RuntimeError("Expected map".to_string())),
        };
        Ok(Self {})
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct PostAggregateMin {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub published: Option<bool>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<::saola_core::chrono::DateTime<::saola_core::chrono::Utc>>,
}
impl ::saola_core::builder::FromResponseIr for PostAggregateMin {
    fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
        let mut map = match item {
            ::saola_core::query_core::response_ir::Item::Map(m) => m,
            _ => return Err(::saola_core::Error::RuntimeError("Expected map".to_string())),
        };
        Ok(Self {
            id: map
                .shift_remove("id")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            title: map
                .shift_remove("title")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            content: map
                .shift_remove("content")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            published: map
                .shift_remove("published")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            user_id: map
                .shift_remove("userId")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            created_at: map
                .shift_remove("createdAt")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
        })
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct PostAggregateMax {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub published: Option<bool>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<::saola_core::chrono::DateTime<::saola_core::chrono::Utc>>,
}
impl ::saola_core::builder::FromResponseIr for PostAggregateMax {
    fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
        let mut map = match item {
            ::saola_core::query_core::response_ir::Item::Map(m) => m,
            _ => return Err(::saola_core::Error::RuntimeError("Expected map".to_string())),
        };
        Ok(Self {
            id: map
                .shift_remove("id")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            title: map
                .shift_remove("title")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            content: map
                .shift_remove("content")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            published: map
                .shift_remove("published")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            user_id: map
                .shift_remove("userId")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            created_at: map
                .shift_remove("createdAt")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
        })
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct PostAggregateResult {
    #[serde(rename = "_count", default)]
    pub _count: PostAggregateCount,
    #[serde(rename = "_sum", default)]
    pub _sum: PostAggregateSum,
    #[serde(rename = "_avg", default)]
    pub _avg: PostAggregateAvg,
    #[serde(rename = "_min", default)]
    pub _min: PostAggregateMin,
    #[serde(rename = "_max", default)]
    pub _max: PostAggregateMax,
}
impl ::saola_core::builder::FromResponseIr for PostAggregateResult {
    fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
        let mut map = match item {
            ::saola_core::query_core::response_ir::Item::Map(m) => m,
            _ => return Err(::saola_core::Error::RuntimeError("Expected map".to_string())),
        };
        Ok(Self {
            _count: map
                .shift_remove("_count")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            _sum: map
                .shift_remove("_sum")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            _avg: map
                .shift_remove("_avg")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            _min: map
                .shift_remove("_min")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            _max: map
                .shift_remove("_max")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
        })
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct PostGroupByResult {
    #[serde(flatten)]
    pub fields: PostData,
    #[serde(rename = "_count", default)]
    pub _count: PostAggregateCount,
    #[serde(rename = "_sum", default)]
    pub _sum: PostAggregateSum,
    #[serde(rename = "_avg", default)]
    pub _avg: PostAggregateAvg,
    #[serde(rename = "_min", default)]
    pub _min: PostAggregateMin,
    #[serde(rename = "_max", default)]
    pub _max: PostAggregateMax,
}
impl ::saola_core::builder::FromResponseIr for PostGroupByResult {
    fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
        let mut map = match item {
            ::saola_core::query_core::response_ir::Item::Map(m) => m,
            _ => return Err(::saola_core::Error::RuntimeError("Expected map".to_string())),
        };
        Ok(Self {
            _count: map
                .shift_remove("_count")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            _sum: map
                .shift_remove("_sum")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            _avg: map
                .shift_remove("_avg")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            _min: map
                .shift_remove("_min")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            _max: map
                .shift_remove("_max")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            fields: ::saola_core::builder::FromResponseIr::from_ir(::saola_core::query_core::response_ir::Item::Map(
                map,
            ))?,
        })
    }
}
