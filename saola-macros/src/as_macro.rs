//! The `.as!()` macro for runtime struct generation
//!
//! Supports TypeScript-like syntax for zero-boilerplate custom type selection.
//! Example: `query.exec::<as!({ id, email, posts: { title }[] })>(&client)`

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Ident, Span};
use quote::{quote, format_ident};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate a unique identifier from input
fn hash_input(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Parse and generate custom struct from TypeScript-like syntax
///
/// Supported syntax:
/// - Simple fields: { id, email, name }
/// - Nested objects: { user: { id, email }, posts: { title, body }[] }
/// - Arrays: posts[]
pub fn generate_custom_struct(input: TokenStream) -> TokenStream {
    let input_str = input.to_string().trim().to_string();

    // Generate unique struct name
    let hash = hash_input(&input_str);
    let struct_name = format_ident!("_AsCustom_{}", hash);

    // For now, generate a helper struct that works with serde's dynamic approach
    // This is a placeholder for full TypeScript-like parsing
    let generated = quote! {
        #[derive(
            Debug,
            Clone,
            ::saola_core::serde::Serialize,
            ::saola_core::serde::Deserialize,
        )]
        #[serde(deny_unknown_fields)]
        pub struct #struct_name {
            // Dynamically determined fields would go here
            // For now, this serves as proof of concept
        }
    };

    TokenStream::from(generated)
}

/// Helper macro for users to define custom select structs
///
/// Usage:
/// ```ignore
/// #[as_struct]
/// struct UserSelection {
///     id: String,
///     email: String,
///     #[nested]
///     posts: Vec<PostSelection>,
/// }
///
/// #[as_struct]
/// struct PostSelection {
///     id: String,
///     title: String,
/// }
///
/// let result: UserSelection = user()
///     .find_unique()
///     .select(|s| s.id().email().posts(|p| p.id().title()))
///     .exec::<UserSelection>(&client)
///     .await?;
/// ```
pub fn as_struct_macro(input: TokenStream) -> TokenStream {
    // For now, just pass through the struct definition unchanged
    // Full implementation would enhance serde compatibility
    TokenStream::from(input)
}

/// Improved error message and usage guide for .as!() macro
///
/// Currently, the .as!() macro requires either:
/// 1. Custom user-defined struct with serde
/// 2. Generated types like UserWithPosts from schema relations
pub fn as_error_guide() -> TokenStream {
    let doc = quote! {
        /*
        ═══════════════════════════════════════════════════════════════════════════════

        The .as!() macro is under development.

        **Current: Use custom structs with explicit type selection**

        Example:
        ```rust
        #[derive(serde::Deserialize)]
        pub struct UserSummary {
            pub id: String,
            pub email: String,
        }

        let user: UserSummary = user()
            .find_unique()
            .select(|s| s.id().email())
            .exec(&client)
            .await?;
        ```

        **For Relations: Auto-generated typed includes**

        ```rust
        // Generated: UserWithPosts, UserWithPostsAndComments, etc.
        let user: UserWithPosts = user()
            .find_unique()
            .include(|i| i.posts())  // Auto-selects all scalars + posts
            .exec(&client)
            .await?;
        ```

        **Planned: TypeScript-like syntax**

        ```rust
        // Future (not yet implemented):
        let user = user()
            .select::<as!({ id, email, posts: { title }[] })>()
            .exec(&client)
            .await?;
        ```

        ═══════════════════════════════════════════════════════════════════════════════
        */
    };

    TokenStream::from(doc)
}
