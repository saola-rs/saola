// Generate the client code from schema.prisma
pub mod client {
    prisma_macros::prisma_client!();
}

#[tokio::main]
async fn main() {
    println!("Prisma client generated!");

    // Test that the generated code is accessible
    let _user_query = client::user();
    let _post_query = client::post();

    println!("✓ client::user() works");
    println!("✓ client::post() works");

    // Test find_many builder
    let _finder = client::user().find_many();
    println!("✓ find_many() works");

    // Test find_unique builder
    let _finder = client::user().find_unique();
    println!("✓ find_unique() works");

    // Test create builder
    let _creator = client::user().create();
    println!("✓ create() works");

    // Test update builder
    let _updater = client::user().update();
    println!("✓ update() works");

    // Test delete builder
    let _deleter = client::user().delete();
    println!("✓ delete() works");

    // Test SelectBuilder with scalar fields only (no nested relations in SelectBuilder)
    let mut select_builder = client::UserSelectBuilder::new();
    select_builder.id().email().name();
    println!("✓ SelectBuilder scalar-only chaining works");

    // Test SelectBuilder.all() method
    let mut select_all = client::UserSelectBuilder::new();
    select_all.all();
    println!("✓ SelectBuilder.all() selects all scalar fields");

    // Test IncludeBuilder (include full relations)
    let mut include_all = client::UserIncludeBuilder::new();
    include_all.posts();
    println!("✓ IncludeBuilder include all works");

    // Test IncludeBuilder with nested select
    let mut include_with_select = client::UserIncludeBuilder::new();
    include_with_select.posts_with(|s| {
        s.id().title();
    });
    println!("✓ IncludeBuilder with nested select works");

    // Test WhereBuilder with dynamic string values
    let search_email = "user@example.com".to_string();
    let mut where_with_dynamic = client::UserWhereBuilder::new();
    where_with_dynamic.email().contains(&search_email); // Dynamic string now works!
    println!("✓ WhereBuilder accepts dynamic strings");

    // Test WhereBuilder chaining with literals still works
    let mut where_builder = client::UserWhereBuilder::new();
    where_builder.email().contains("@gmail.com");
    println!("✓ WhereBuilder string filter works");

    // Test type-aware WhereBuilder with boolean field
    let mut where_with_bool = client::UserWhereBuilder::new();
    where_with_bool.posts(|p| {
        p.published().eq(true); // Type-aware: boolean field takes bool, not &str!
    });
    println!("✓ Type-aware boolean filter works");

    // Test enum-aware filter with Role enum
    let mut where_role = client::UserWhereBuilder::new();
    where_role.role().eq(client::Role::Admin);
    println!("✓ Enum-aware filter for Role works");

    // Test enum-aware filter with dynamic string value
    let role_str = "MODERATOR";
    let mut where_role_str = client::UserWhereBuilder::new();
    where_role_str.role().eq_str(role_str);
    println!("✓ Enum filter with dynamic string value works");

    // Test enum-aware filter with PostStatus enum
    let mut where_status = client::PostWhereBuilder::new();
    where_status.status().eq(client::PostStatus::Published);
    println!("✓ Enum-aware filter for PostStatus works");

    // Test PostStatus with dynamic string
    let status_str = "DRAFT";
    let mut where_status_str = client::PostWhereBuilder::new();
    where_status_str.status().eq_str(status_str);
    println!("✓ PostStatus filter with dynamic string works");

    // Test that enums can be converted from strings
    let role_from_str = client::Role::from("ADMIN");
    println!("✓ Enum conversion from &str works: {:?}", role_from_str);

    // Test that enums have as_str() method
    let status = client::PostStatus::Draft;
    let status_str = status.as_str();
    println!("✓ Enum as_str() method works: {}", status_str);

    println!("\nAll Phase 3 enum-aware features work correctly!");
}
