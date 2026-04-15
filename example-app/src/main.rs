use prisma_core::prelude::*;
prisma_macros::init!("schema.prisma");
use db::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize client from schema
    let client = db::client().await?;
    println!("✓ Client initialized successfully");

    let unique_email = format!(
        "user-{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // Create a new user with type-safe builders
    println!("\n[Creating user]");
    let create_result: User = user()
        .create(unique_email.clone(), Role::USER)
        .exec(&client)
        .await?;
    println!("  ✓ Created user: {} ({})", create_result.email, create_result.role);

    // Test type-safe filters
    println!("\n[Testing type-safe filters]");
    let found_users: Vec<User> = user()
        .find_many()
        .where_clause(|w| {
            // Type-safe: .contains() is only available on StringFilter
            w.email().contains("@example.com".to_string());
        })
        .exec(&client)
        .await?;
    println!("  ✓ Found {} users with email containing '@example.com'", found_users.len());

    // Find unique user by email
    println!("\n[Finding unique user]");
    let found_user: Option<User> = user()
        .find_unique()
        .where_clause(|w| {
            w.email(unique_email.clone());
        })
        .exec(&client)
        .await?;

    let found_user_email = if let Some(ref u) = found_user {
        println!("  ✓ Found user: {} (id: {})", u.email, u.id);
        u.id.clone()
    } else {
        "".to_string()
    };

    // Create some posts if we have a user
    if !found_user_email.is_empty() {
        println!("\n[Creating posts for user]");
        for i in 1..=3 {
            let _post: db::Post = db::post()
                .create(
                    format!("Post {} Title", i),
                    db::PostStatus::PUBLISHED,
                    false,
                    found_user_email.clone(),
                )
                .exec(&client)
                .await?;
        }
        println!("  ✓ Created 3 posts");
    }

    // Count users with specific role
    println!("\n[Counting users]");
    let count: i64 = user()
        .count()
        .where_clause(|w| {
            w.role().eq(Role::USER);
        })
        .exec(&client)
        .await?;
    println!("  ✓ Total users with role USER: {}", count);

    // Test relation type generation
    println!("\n[Testing relation types]");
    // This should compile - relation types should be available
    let _: std::marker::PhantomData<db::UserWithPosts> = std::marker::PhantomData;
    println!("  ✓ UserWithPosts type is available");

    // Test include with typed return
    println!("\n[Testing include with typed return]");
    if !found_user_email.is_empty() {
        let user_with_posts: db::UserWithPosts = user()
            .find_unique()
            .where_clause(|w| {
                w.id(found_user_email.clone());
            })
            .include(|i| {
                i.posts();
            })
            .exec(&client)
            .await?;
        println!("  ✓ Retrieved UserWithPosts: {}", user_with_posts.email);
        println!("    Posts count: {}", user_with_posts.posts.len());
        for (idx, post) in user_with_posts.posts.iter().enumerate() {
            println!("      Post {}: {} ({})", idx + 1, post.title, post.status);
            println!("        User in post: {} (id: {})", post.user.email, post.user.id);
        }
    }

    println!("\n[All tests passed] ✓\n");
    Ok(())
}
