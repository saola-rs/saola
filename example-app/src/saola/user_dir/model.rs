use super::super::enums;
use ::saola_core::serde;
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct UserData<Posts = ::saola_core::Unloaded, Profile = ::saola_core::Unloaded> {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "isActive", default)]
    pub is_active: bool,
    #[serde(rename = "score", default)]
    pub score: f64,
    #[serde(rename = "level", default)]
    pub level: i32,
    #[serde(rename = "createdAt", default)]
    pub created_at: ::saola_core::chrono::DateTime<::saola_core::chrono::Utc>,
    #[serde(rename = "posts", default)]
    pub posts: Posts,
    #[serde(rename = "profile", default)]
    pub profile: Profile,
}
pub type User = UserData;
impl ::saola_core::builder::SelectStruct for UserData {
    fn selections() -> Vec<::saola_core::query_core::Selection> {
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
}
impl ::saola_core::builder::GetSelections for UserData {
    fn get_selections() -> Vec<::saola_core::query_core::Selection> {
        <Self as ::saola_core::builder::SelectStruct>::selections()
    }
}
impl<Posts, Profile> ::saola_core::builder::FromResponseIr for UserData<Posts, Profile>
where
    Posts: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
    Profile: ::saola_core::builder::FromResponseIr + Default + Send + Sync,
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
            email: map
                .shift_remove("email")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            name: map
                .shift_remove("name")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            is_active: map
                .shift_remove("isActive")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            score: map
                .shift_remove("score")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            level: map
                .shift_remove("level")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            created_at: map
                .shift_remove("createdAt")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            posts: map
                .shift_remove("posts")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            profile: map
                .shift_remove("profile")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
        })
    }
}
#[allow(dead_code)]
pub mod _user {
    #[allow(unused_imports)]
    pub use super::UserData;
    pub mod fields {
        use super::super::enums;
        pub const ID: ::saola_core::Field<String> = ::saola_core::Field::new("id");
        pub type id = String;
        pub const EMAIL: ::saola_core::Field<String> = ::saola_core::Field::new("email");
        pub type email = String;
        pub const NAME: ::saola_core::Field<String> = ::saola_core::Field::new("name");
        pub type name = String;
        pub const IS_ACTIVE: ::saola_core::Field<bool> = ::saola_core::Field::new("isActive");
        pub type is_active = bool;
        pub const SCORE: ::saola_core::Field<f64> = ::saola_core::Field::new("score");
        pub type score = f64;
        pub const LEVEL: ::saola_core::Field<i32> = ::saola_core::Field::new("level");
        pub type level = i32;
        pub const CREATED_AT: ::saola_core::Field<::saola_core::chrono::DateTime<::saola_core::chrono::Utc>> =
            ::saola_core::Field::new("createdAt");
        pub type created_at = ::saola_core::chrono::DateTime<::saola_core::chrono::Utc>;
        pub type posts = ();
        pub type profile = ();
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct UserAggregateCount {
    #[serde(rename = "_all", default)]
    pub _all: i64,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub email: i64,
    #[serde(default)]
    pub name: i64,
    #[serde(default)]
    pub is_active: i64,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub level: i64,
    #[serde(default)]
    pub created_at: i64,
}
impl ::saola_core::builder::FromResponseIr for UserAggregateCount {
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
            email: map
                .shift_remove("email")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            name: map
                .shift_remove("name")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            is_active: map
                .shift_remove("isActive")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            score: map
                .shift_remove("score")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            level: map
                .shift_remove("level")
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
pub struct UserAggregateSum {
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub level: Option<i32>,
}
impl ::saola_core::builder::FromResponseIr for UserAggregateSum {
    fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
        let mut map = match item {
            ::saola_core::query_core::response_ir::Item::Map(m) => m,
            _ => return Err(::saola_core::Error::RuntimeError("Expected map".to_string())),
        };
        Ok(Self {
            score: map
                .shift_remove("score")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            level: map
                .shift_remove("level")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
        })
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct UserAggregateAvg {
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub level: Option<f64>,
}
impl ::saola_core::builder::FromResponseIr for UserAggregateAvg {
    fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
        let mut map = match item {
            ::saola_core::query_core::response_ir::Item::Map(m) => m,
            _ => return Err(::saola_core::Error::RuntimeError("Expected map".to_string())),
        };
        Ok(Self {
            score: map
                .shift_remove("score")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            level: map
                .shift_remove("level")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
        })
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct UserAggregateMin {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub level: Option<i32>,
    #[serde(default)]
    pub created_at: Option<::saola_core::chrono::DateTime<::saola_core::chrono::Utc>>,
}
impl ::saola_core::builder::FromResponseIr for UserAggregateMin {
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
            email: map
                .shift_remove("email")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            name: map
                .shift_remove("name")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            is_active: map
                .shift_remove("isActive")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            score: map
                .shift_remove("score")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            level: map
                .shift_remove("level")
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
pub struct UserAggregateMax {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub level: Option<i32>,
    #[serde(default)]
    pub created_at: Option<::saola_core::chrono::DateTime<::saola_core::chrono::Utc>>,
}
impl ::saola_core::builder::FromResponseIr for UserAggregateMax {
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
            email: map
                .shift_remove("email")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            name: map
                .shift_remove("name")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            is_active: map
                .shift_remove("isActive")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            score: map
                .shift_remove("score")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            level: map
                .shift_remove("level")
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
pub struct UserAggregateResult {
    #[serde(rename = "_count", default)]
    pub _count: UserAggregateCount,
    #[serde(rename = "_sum", default)]
    pub _sum: UserAggregateSum,
    #[serde(rename = "_avg", default)]
    pub _avg: UserAggregateAvg,
    #[serde(rename = "_min", default)]
    pub _min: UserAggregateMin,
    #[serde(rename = "_max", default)]
    pub _max: UserAggregateMax,
}
impl ::saola_core::builder::FromResponseIr for UserAggregateResult {
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
pub struct UserGroupByResult {
    #[serde(flatten)]
    pub fields: UserData,
    #[serde(rename = "_count", default)]
    pub _count: UserAggregateCount,
    #[serde(rename = "_sum", default)]
    pub _sum: UserAggregateSum,
    #[serde(rename = "_avg", default)]
    pub _avg: UserAggregateAvg,
    #[serde(rename = "_min", default)]
    pub _min: UserAggregateMin,
    #[serde(rename = "_max", default)]
    pub _max: UserAggregateMax,
}
impl ::saola_core::builder::FromResponseIr for UserGroupByResult {
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
