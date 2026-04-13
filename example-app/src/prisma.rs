pub mod client {
    use super::*;
    use ::prisma_core as _prisma_core;
    pub fn client() -> _prisma_core::PrismaClient {
        todo!("Initialize client")
    }
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        Hash,
        ::prisma_core::serde::Serialize,
        ::prisma_core::serde::Deserialize,
        Default
    )]
    #[serde(crate = "::prisma_core::serde", rename_all = "UPPERCASE")]
    pub enum Role {
        #[default]
        ADMIN,
        USER,
        MODERATOR,
    }
    impl From<Role> for ::prisma_core::query_structure::PrismaValue {
        fn from(val: Role) -> Self {
            ::prisma_core::query_structure::PrismaValue::Enum(
                format!("{:?}", val).to_uppercase(),
            )
        }
    }
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        Hash,
        ::prisma_core::serde::Serialize,
        ::prisma_core::serde::Deserialize,
        Default
    )]
    #[serde(crate = "::prisma_core::serde", rename_all = "UPPERCASE")]
    pub enum PostStatus {
        #[default]
        DRAFT,
        PUBLISHED,
        ARCHIVED,
    }
    impl From<PostStatus> for ::prisma_core::query_structure::PrismaValue {
        fn from(val: PostStatus) -> Self {
            ::prisma_core::query_structure::PrismaValue::Enum(
                format!("{:?}", val).to_uppercase(),
            )
        }
    }
    #[derive(Debug, ::prisma_core::serde::Deserialize, Default)]
    #[serde(crate = "::prisma_core::serde", default)]
    #[prisma_macros::prisma_model]
    pub struct User {
        #[prisma(name = "id", id, unique)]
        #[serde(rename = "id")]
        pub id: String,
        #[prisma(name = "email", unique)]
        #[serde(rename = "email")]
        pub email: String,
        #[prisma(name = "displayName")]
        #[serde(rename = "displayName")]
        pub display_name: Option<String>,
        #[prisma(name = "role")]
        #[serde(rename = "role")]
        pub role: Role,
        #[serde(rename = "posts")]
        #[prisma(name = "posts", relation)]
        pub posts: Vec<Post>,
    }
    #[derive(Debug, ::prisma_core::serde::Deserialize, Default)]
    #[serde(crate = "::prisma_core::serde", default)]
    #[prisma_macros::prisma_model]
    pub struct Post {
        #[prisma(name = "id", id, unique)]
        #[serde(rename = "id")]
        pub id: String,
        #[prisma(name = "title")]
        #[serde(rename = "title")]
        pub title: String,
        #[prisma(name = "status")]
        #[serde(rename = "status")]
        pub status: PostStatus,
        #[prisma(name = "published")]
        #[serde(rename = "published")]
        pub published: bool,
        #[prisma(name = "userId")]
        #[serde(rename = "userId")]
        pub user_id: String,
        #[serde(rename = "user")]
        #[prisma(name = "user", relation)]
        pub user: User,
    }
}
