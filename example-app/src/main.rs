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

    // 2. CREATE Operation with Nested Relations
    println!("\n[1. Creating User with Nested Posts and Profile]");
    let created_user = user()
        .create(unique_email.clone()) // role is optional (default: USER)
        .data(|d| {
            d.posts(|p| {
                // Post required: title (status, published are implicit defaults; user is implicit)
                p.create("Nested Post 1".to_string(), |_| {});
                p.create("Nested Post 2".to_string(), |_| {});
            });
            d.profile(|p| {
                // Profile required: bio (user is implicit)
                p.create("I am a nested profile".to_string(), |_| {});
            });
        })
        .include(|u| u.posts())
        .include(|u| u.profile())
        .exec(&client)
        .await?;
    println!(
        "  ✓ Created: {} (id: {}) with nested posts and profile \n {:?}",
        created_user.email, created_user.id, created_user
    );

    // 3. CREATE Nested Relationships (Manual)
    println!("\n[2. Creating Manual Post for User]");
    let post1 = post()
        .create(
            "Rust ORM is awesome".to_string(),
            |u| { u.connect(|w| { w.id(created_user.id.clone()); }); }
        )
        .exec(&client)
        .await?;

    let post2 = post()
        .create(
            "Native Engines in Rust".to_string(),
            |u| { u.connect(|w| { w.id(created_user.id.clone()); }); }
        )
        .exec(&client)
        .await?;
    println!("  ✓ Created 2 posts: '{}' and '{}'", post1.title, post2.title);

    println!("\n[2.1. Creating Comments on Posts]");
    let _comment1 = comment()
        .create(
            "This is great!".to_string(),
            |p| { p.connect(|w| { w.id(post1.id.clone()); }); }
        )
        .exec(&client)
        .await?;
    let _comment2 = comment()
        .create(
            "I agree".to_string(),
            |p| { p.connect(|w| { w.id(post1.id.clone()); }); }
        )
        .exec(&client)
        .await?;
    println!("  ✓ Created 2 comments on post1");

    // 4. FIND MANY with Complex Filters (Scalar + Relation Filters)
    println!("\n[3. Find Many with Logical Operators + Relation Filters]");
    let users = user()
        .find_many()
        .where_clause(|w| {
            // Nested filter: Users who have some posts with 'awesome' in the title
            w.posts().some(|p: &mut PostWhereBuilder| {
                p.title().contains("awesome".to_string());
            });
            
            w.or(|w| {
                w.email().contains("example.com".to_string());
                w.role().eq(Role::USER);
            });
        })
        .exec(&client)
        .await?;
    println!("  ✓ Found {} users matching criteria", users.len());

    // 5. FIND UNIQUE with Automatic Relationship Inclusion (Nested Include + Nested Filter + Chained Include)
    println!("\n[4. Find Unique + Automatic Include Inference + Nested Filter + Multiple Includes]");
    let user_with_data = user()
        .find_unique()
        .where_clause(|w| {
            w.email(unique_email.clone());
        })
        .include(|i| i.posts_with(|p| {
            p.where_clause(|w| {
                w.title().contains("awesome".to_string());
            });
            p.comments_with(|c| {
                c.where_clause(|w| {
                    w.text().contains("agree".to_string());
                });
                c.empty()
            })
        }))
        .include(|i| i.profile()) // Chained include: Fetch profile as well
        .exec(&client)
        .await?
        .expect("User should exist");

    println!("  ✓ Retrieved UserWithPostsAndProfile: {}", user_with_data.email);
    println!("    Bio: {}", user_with_data.profile.as_ref().map(|p| p.bio.as_str()).unwrap_or("No bio"));
    println!("    Total Filtered Posts: {}", user_with_data.posts.len());
    for p in user_with_data.posts {
        println!("      - {} ({:?})", p.title, p.status);
        for c in p.comments {
            println!("        * {}", c.text);
        }
    }

    // 6. Ad-hoc selection with select_as! macro (TypeScript-like syntax)
    println!("\n[5. Ad-hoc selection with select_as! macro]");
    let partial_data = user()
        .find_unique()
        .where_clause(|w| {
            w.email(unique_email.clone());
        })
        .include(|i| i.posts_as(db::select_as!({
            id: String,
            title: String,
            status: PostStatus
        })))
    .exec(&client)
    .await?
    .expect("User should exist");

    println!("  ✓ Zero-boilerplate selection successful!");
    println!("    Selected Email: {}", partial_data.email);
    println!("    Nested Posts (using _as): {:?}", partial_data.posts);

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

    // 8. UPDATE Operation with Nested Relation Writes
    println!("\n[7. Update User with Nested Relation Writes]");
    let updated_user = user()
        .update()
        .where_clause(|w| {
            w.email(unique_email.clone());
        })
        .data(|d| {
            d.display_name("Prisma Power User".to_string());
            // Nested update: Update a specific post
            d.posts(|p| {
                p.update(|w| { w.title("Nested Post 1".to_string()); }, |d| {
                    d.title("Updated Nested Post 1".to_string());
                });
                // Nested upsert: Update or create a post
                p.upsert(|w| { w.title("New Upserted Post".to_string()); }, 
                    "New Upserted Post".to_string(), // Required title
                    |_| {}, // Create closure
                    |d| { d.status(PostStatus::PUBLISHED); } // Update closure
                );
            });
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

    // 10. BULK Operations
    println!("\n[9. Bulk Operations]");
    let created_users = user()
        .create_many()
        .data(|d| { d.email(format!("bulk-ret1-{}@example.com", unique_email)); })
        .data(|d| { d.email(format!("bulk-ret2-{}@example.com", unique_email)); })
        .exec(&client)
        .await?;
    println!("  ✓ Bulk created {} users", created_users);

    // Test CreateManyAndReturn
    let created_users_with_data = user()
        .create_many_and_return()
        .data(|d| { d.email(format!("bulk-and-ret1-{}@example.com", unique_email)); })
        .data(|d| { d.email(format!("bulk-and-ret2-{}@example.com", unique_email)); })
        .exec(&client)
        .await?;
    println!("  ✓ Bulk created and returned {} users: {:?}", created_users_with_data.len(),
        created_users_with_data.iter().map(|u| u.email.as_str()).collect::<Vec<_>>());

    let updated_count = user()
        .update_many()
        .where_clause(|w| { w.email().contains("bulk".to_string()); })
        .data(|d| { d.display_name("Bulk User".to_string()); })
        .exec(&client)
        .await?;
    println!("  ✓ Bulk updated {} users", updated_count);

    let deleted_count = user()
        .delete_many()
        .where_clause(|w| { w.email().contains("bulk".to_string()); })
        .exec(&client)
        .await?;
    println!("  ✓ Bulk deleted {} users", deleted_count);

    println!("\n[All supported features demonstrated] ✓\n");
    Ok(())
}
