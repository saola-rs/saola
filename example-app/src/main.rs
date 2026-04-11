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

    // Test WhereBuilder chaining
    let mut where_builder = client::UserWhereBuilder::new();
    where_builder.email().contains("@gmail.com");
    println!("✓ WhereBuilder string filter works");

    // Test type-aware WhereBuilder with boolean field
    let mut where_with_bool = client::UserWhereBuilder::new();
    where_with_bool.posts(|p| {
        p.published().eq(true);  // Type-aware: boolean field takes bool, not &str!
    });
    println!("✓ Type-aware boolean filter works");

    println!("\nAll Phase 2 builder patterns work correctly!");
}
