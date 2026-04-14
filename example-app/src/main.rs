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

    if let Some(u) = found_user {
        println!("  ✓ Found user: {} (id: {})", u.email, u.id);
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

    println!("\n[All tests passed] ✓\n");
    Ok(())
}
