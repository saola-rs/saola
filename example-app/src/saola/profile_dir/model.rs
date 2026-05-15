use super::super::enums;
use ::saola_core::serde;
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct ProfileData<User = ::saola_core::Unloaded> {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "bio")]
    pub bio: Option<String>,
    #[serde(rename = "userId")]
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
    User: ::saola_core::builder::FromResponseIr + Default,
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
            bio: map
                .shift_remove("bio")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            user_id: map
                .shift_remove("userId")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "userId")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
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
