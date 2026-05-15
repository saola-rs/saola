use super::super::enums;
use ::saola_core::serde;
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct UserData<Posts = ::saola_core::Unloaded, Profile = ::saola_core::Unloaded> {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "score")]
    pub score: f64,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "createdAt")]
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
    Posts: ::saola_core::builder::FromResponseIr + Default,
    Profile: ::saola_core::builder::FromResponseIr + Default,
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
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "id")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
            email: map
                .shift_remove("email")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "email")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
            name: map
                .shift_remove("name")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "name")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
            is_active: map
                .shift_remove("isActive")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "isActive")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
            score: map
                .shift_remove("score")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "score")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
            level: map
                .shift_remove("level")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "level")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
            created_at: map
                .shift_remove("createdAt")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "createdAt")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
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
