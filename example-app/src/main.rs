prisma_macros::init!("schema.prisma");
use db::*;
use prisma_core::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Client Initialization
    let client = db::client().await?;
    println!("✓ Client initialized successfully");

    let unique_email = format!(
        "user-{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // 2. CREATE Operation (Auto-inferred return type: db::User)
    println!("\n[1. Creating User]");
    // Rename local variable to avoid shadowing the user() factory function
    let created_user = user().create(unique_email.clone(), Role::USER).exec(&client).await?;
    println!("  ✓ Created: {} (id: {})", created_user.email, created_user.id);

    // 3. CREATE Nested Relationships
    println!("\n[2. Creating Posts for User]");
    let post1 = post()
        .create(
            "Rust ORM is awesome".to_string(),
            PostStatus::PUBLISHED,
            true,
            created_user.id.clone(),
        )
        .exec(&client)
        .await?;

    let post2 = post()
        .create(
            "Native Engines in Rust".to_string(),
            PostStatus::DRAFT,
            false,
            created_user.id.clone(),
        )
        .exec(&client)
        .await?;
    println!("  ✓ Created 2 posts: '{}' and '{}'", post1.title, post2.title);

    // 4. FIND MANY with Complex Filters
    println!("\n[3. Find Many with Logical Operators]");
    let users = user()
        .find_many()
        .where_clause(|w| {
            w.or(|w| {
                w.email().contains("example.com".to_string());
                w.role().eq(Role::USER);
            });
            w.not(|w| {
                w.id().eq("non-existent-id".to_string());
            });
        })
        .exec(&client)
        .await?;
    println!("  ✓ Found {} users matching criteria", users.len());

    // 5. FIND UNIQUE with Automatic Relationship Inclusion
    println!("\n[4. Find Unique + Automatic Include Inference]");
    // .posts() transitions return type from Option<User> to Option<UserWithPosts>
    let user_with_posts = user()
        .find_unique()
        .where_clause(|w| {
            w.email(unique_email.clone());
        })
        .posts()
        .exec(&client)
        .await?
        .expect("User should exist");

    println!("  ✓ Retrieved UserWithPosts: {}", user_with_posts.email);
    println!("    Total Posts: {}", user_with_posts.posts.len());
    for p in user_with_posts.posts {
        println!("      - {} ({:?})", p.title, p.status);
    }

    // 6. Ad-hoc selection with select_as! macro (TypeScript-like syntax)
    println!("\n[5. Ad-hoc selection with select_as! macro]");
    let partial_data = db::select_as!(
        user()
            .find_unique()
            .where_clause(|w| {w.email(unique_email.clone());}),
        {
            id: String,
            email: String,
            posts: {
                title: String,
                status: PostStatus
            }[]
        }
    )
    .exec(&client)
    .await?
    .expect("User should exist");

    println!("  ✓ Zero-boilerplate selection successful!");
    println!("    Selected ID: {}", partial_data.id);
    println!("    Selected Email: {}", partial_data.email);
    println!("    Nested Posts: {:?}", partial_data.posts);

    // 7. COUNT Operation
    println!("\n[6. Count Users]");
    let count = user()
        .count()
        .where_clause(|w| {
            w.role().eq(Role::USER);
        })
        .exec(&client)
        .await?;
    println!("  ✓ Total USERs in database: {}", count);

    // 8. UPDATE Operation (using Unique Filter)
    println!("\n[7. Update User Display Name]");
    let updated_user = user()
        .update()
        .where_clause(|w| {
            w.email(unique_email.clone());
        })
        .data(|d| {
            d.display_name("Prisma User".to_string());
        })
        .exec(&client)
        .await?;
    println!("  ✓ Updated: {} -> {:?}", updated_user.email, updated_user.display_name);

    // 9. DELETE Operation
    println!("\n[8. Delete User]");
    let deleted_user = user()
        .delete()
        .where_clause(|w| {
            w.email(unique_email.clone());
        })
        .exec(&client)
        .await?;
    println!("  ✓ Deleted User: {}", deleted_user.email);

    println!("\n[All supported features demonstrated] ✓\n");
    Ok(())
}
