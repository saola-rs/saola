use super::super::enums;
use ::saola_core::serde;
#[derive(Debug, Clone, :: saola_core :: serde :: Serialize, :: saola_core :: serde :: Deserialize, Default)]
#[serde(crate = "::saola_core::serde")]
pub struct PostData<User = ::saola_core::Unloaded> {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "content")]
    pub content: Option<String>,
    #[serde(rename = "published")]
    pub published: bool,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "createdAt")]
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
            title: map
                .shift_remove("title")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "title")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
            content: map
                .shift_remove("content")
                .map(::saola_core::builder::FromResponseIr::from_ir)
                .transpose()?
                .flatten(),
            published: map
                .shift_remove("published")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "published")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
            user_id: map
                .shift_remove("userId")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "userId")))
                .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
            created_at: map
                .shift_remove("createdAt")
                .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing required field: {}", "createdAt")))
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
