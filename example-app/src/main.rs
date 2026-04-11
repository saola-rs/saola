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

    // Test SelectBuilder chaining
    let mut select_builder = client::UserSelectBuilder::new();
    select_builder.id().email().name();
    println!("✓ SelectBuilder chaining works");

    // Test nested SelectBuilder
    let mut select_with_posts = client::UserSelectBuilder::new();
    select_with_posts.id().posts(|p| {
        p.id().title();
    });
    println!("✓ Nested SelectBuilder works");

    // Test WhereBuilder chaining
    let mut where_builder = client::UserWhereBuilder::new();
    where_builder.email().contains("@gmail.com");
    println!("✓ WhereBuilder chaining works");

    // Test nested WhereBuilder
    let mut where_with_posts = client::UserWhereBuilder::new();
    where_with_posts.posts(|p| {
        p.published().eq("true");
    });
    println!("✓ Nested WhereBuilder works");

    println!("\nAll builder chains compile and work!");
}

