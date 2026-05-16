use super::super::enums;
use ::saola_core::serde;
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct ProfileData<User = ::saola_core::Unloaded> {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "bio", default)]
    pub bio: Option<String>,
    #[serde(rename = "userId", default)]
    pub user_id: String,
    #[serde(rename = "user", default)]
    pub user: User,
}
pub type Profile = ProfileData;
impl ::saola_core::builder::SelectStruct for ProfileData {
    fn selections() -> Vec<::saola_core::query_core::Selection> {
        vec![
            ::saola_core::query_core::Selection::with_name("id".to_string()),
            ::saola_core::query_core::Selection::with_name("bio".to_string()),
            ::saola_core::query_core::Selection::with_name("userId".to_string()),
        ]
    }
}
impl ::saola_core::builder::GetSelections for ProfileData {
    fn get_selections() -> Vec<::saola_core::query_core::Selection> {
        <Self as ::saola_core::builder::SelectStruct>::selections()
    }
}
impl<User> ::saola_core::builder::FromResponseIr for ProfileData<User>
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
            bio: map
                .shift_remove("bio")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            user_id: map
                .shift_remove("userId")
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
pub mod _profile {
    #[allow(unused_imports)]
    pub use super::ProfileData;
    pub mod fields {
        use super::super::enums;
        pub const ID: ::saola_core::Field<String> = ::saola_core::Field::new("id");
        pub type id = String;
        pub const BIO: ::saola_core::Field<String> = ::saola_core::Field::new("bio");
        pub type bio = Option<String>;
        pub const USER_ID: ::saola_core::Field<String> = ::saola_core::Field::new("userId");
        pub type user_id = String;
        pub type user = ();
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct ProfileAggregateCount {
    #[serde(rename = "_all", default)]
    pub _all: i64,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub bio: i64,
    #[serde(default)]
    pub user_id: i64,
}
impl ::saola_core::builder::FromResponseIr for ProfileAggregateCount {
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
            bio: map
                .shift_remove("bio")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
            user_id: map
                .shift_remove("userId")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .unwrap_or_default(),
        })
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct ProfileAggregateSum {}
impl ::saola_core::builder::FromResponseIr for ProfileAggregateSum {
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
pub struct ProfileAggregateAvg {}
impl ::saola_core::builder::FromResponseIr for ProfileAggregateAvg {
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
pub struct ProfileAggregateMin {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
}
impl ::saola_core::builder::FromResponseIr for ProfileAggregateMin {
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
            bio: map
                .shift_remove("bio")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            user_id: map
                .shift_remove("userId")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
        })
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct ProfileAggregateMax {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
}
impl ::saola_core::builder::FromResponseIr for ProfileAggregateMax {
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
            bio: map
                .shift_remove("bio")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            user_id: map
                .shift_remove("userId")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
        })
    }
}
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct ProfileAggregateResult {
    #[serde(rename = "_count", default)]
    pub _count: ProfileAggregateCount,
    #[serde(rename = "_sum", default)]
    pub _sum: ProfileAggregateSum,
    #[serde(rename = "_avg", default)]
    pub _avg: ProfileAggregateAvg,
    #[serde(rename = "_min", default)]
    pub _min: ProfileAggregateMin,
    #[serde(rename = "_max", default)]
    pub _max: ProfileAggregateMax,
}
impl ::saola_core::builder::FromResponseIr for ProfileAggregateResult {
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
pub struct ProfileGroupByResult {
    #[serde(flatten)]
    pub fields: ProfileData,
    #[serde(rename = "_count", default)]
    pub _count: ProfileAggregateCount,
    #[serde(rename = "_sum", default)]
    pub _sum: ProfileAggregateSum,
    #[serde(rename = "_avg", default)]
    pub _avg: ProfileAggregateAvg,
    #[serde(rename = "_min", default)]
    pub _min: ProfileAggregateMin,
    #[serde(rename = "_max", default)]
    pub _max: ProfileAggregateMax,
}
impl ::saola_core::builder::FromResponseIr for ProfileGroupByResult {
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
