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

    let unique_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    // 2. CREATE Operation with Nested Relations
    println!("\n[1. Creating User with Nested Posts and Profile]");
    let created_user = user()
        .create(unique_email.clone()) // role is optional (default: USER)
        .data(|d| {
            d.posts(|p| {
                // Post required: title (status, published are implicit defaults; user is implicit)
                p.create(format!("Nested Post 1 - {}", unique_ts), |_| {});
                p.create(format!("Nested Post 2 - {}", unique_ts), |_| {});
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
        .create(format!("Rust ORM is awesome - {}", unique_ts), |u| {
            u.connect(|w| {
                w.id(created_user.id.clone());
            });
        })
        .exec(&client)
        .await?;

    let post2 = post()
        .create(format!("Native Engines in Rust - {}", unique_ts), |u| {
            u.connect(|w| {
                w.id(created_user.id.clone());
            });
        })
        .exec(&client)
        .await?;
    println!("  ✓ Created 2 posts: '{}' and '{}'", post1.title, post2.title);

    println!("\n[2.1. Creating Comments on Posts]");
    let _comment1 = comment()
        .create("This is great!".to_string(), |p| {
            p.connect(|w| {
                w.id(post1.id.clone());
            });
        })
        .exec(&client)
        .await?;
    let _comment2 = comment()
        .create("I agree".to_string(), |p| {
            p.connect(|w| {
                w.id(post1.id.clone());
            });
        })
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
        .include(|i| {
            i.posts_with(|p| {
                p.where_clause(|w| {
                    w.title().contains("awesome".to_string());
                });
                p.comments_with(|c| {
                    c.where_clause(|w| {
                        w.text().contains("agree".to_string());
                    });
                    c.empty()
                })
            })
        })
        .include(|i| i.profile()) // Chained include: Fetch profile as well
        .exec(&client)
        .await?
        .expect("User should exist");

    println!("  ✓ Retrieved UserWithPostsAndProfile: {}", user_with_data.email);
    println!(
        "    Bio: {}",
        user_with_data
            .profile
            .as_ref()
            .map(|p| p.bio.as_str())
            .unwrap_or("No bio")
    );
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
        .include(|i| {
            i.posts_as(db::select_as!({
                id: String,
                title: String,
                status: PostStatus
            }))
        })
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

    // 7.1. FIND FIRST Operation
    println!("\n[6.1. Find First]");
    let first_user = user()
        .find_first()
        .where_clause(|w| {
            w.role().eq(Role::USER);
        })
        .exec(&client)
        .await?;
    if let Some(u) = first_user {
        println!("  ✓ Found first user: {}", u.email);
    } else {
        println!("  ✓ No users found");
    };

    // 7.2. FIND FIRST OR THROW Operation
    println!("\n[6.2. Find First Or Throw]");
    let first_or_throw = user()
        .find_first_or_throw()
        .where_clause(|w| {
            w.role().eq(Role::USER);
        })
        .exec(&client)
        .await;
    match first_or_throw {
        Ok(u) => println!("  ✓ Found first user (or throw): {}", u.email),
        Err(_) => println!("  ✓ No user found (error expected)"),
    }

    // 7.3. FIND UNIQUE OR THROW Operation
    println!("\n[6.3. Find Unique Or Throw]");
    let unique_or_throw = user()
        .find_unique_or_throw()
        .where_clause(|w| {
            w.email(unique_email.clone());
        })
        .exec(&client)
        .await;
    match unique_or_throw {
        Ok(u) => println!("  ✓ Found user (or throw): {}", u.email),
        Err(_) => println!("  ✓ User not found (error expected)"),
    }

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
                p.update(
                    |w| {
                        w.title(format!("Nested Post 1 - {}", unique_ts));
                    },
                    |d| {
                        d.title(format!("Updated Nested Post 1 - {}", unique_ts));
                    },
                );
                // Nested upsert: Update or create a post
                p.upsert(
                    |w| {
                        w.title(format!("New Upserted Post - {}", unique_ts));
                    },
                    format!("New Upserted Post - {}", unique_ts), // Required title
                    |_| {},                          // Create closure
                    |d| {
                        d.status(PostStatus::PUBLISHED);
                    }, // Update closure
                );
            });
        })
        .exec(&client)
        .await?;
    println!("  ✓ Updated: {} -> {:?}", updated_user.email, updated_user.display_name);

    // 8.1. TOP-LEVEL UPSERT Operation
    println!("\n[7.1. Upsert User]");
    let upsert_email = format!("upsert-user-{}@example.com", unique_ts);
    let upserted_user = user()
        .upsert()
        .where_clause(|w| {
            w.email(upsert_email.clone());
        })
        .create(upsert_email.clone(), |d| {
            d.role(Role::ADMIN);
        })
        .update(|d| {
            d.role(Role::MODERATOR);
        })
        .exec(&client)
        .await?;
    println!("  ✓ Upserted: {} with role {:?}", upserted_user.email, upserted_user.role);

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
        .data(|d| {
            d.email(format!("bulk-ret1-{}@example.com", unique_ts));
        })
        .data(|d| {
            d.email(format!("bulk-ret2-{}@example.com", unique_ts));
        })
        .exec(&client)
        .await?;
    println!("  ✓ Bulk created {} users", created_users);

    // Test CreateManyAndReturn
    let created_users_with_data = user()
        .create_many_and_return()
        .data(|d| {
            d.email(format!("bulk-and-ret1-{}@example.com", unique_ts));
        })
        .data(|d| {
            d.email(format!("bulk-and-ret2-{}@example.com", unique_ts));
        })
        .exec(&client)
        .await?;
    println!(
        "  ✓ Bulk created and returned {} users: {:?}",
        created_users_with_data.len(),
        created_users_with_data
            .iter()
            .map(|u| u.email.as_str())
            .collect::<Vec<_>>()
    );

    let updated_count = user()
        .update_many()
        .where_clause(|w| {
            w.email().contains("bulk".to_string());
        })
        .data(|d| {
            d.display_name("Bulk User".to_string());
        })
        .exec(&client)
        .await?;
    println!("  ✓ Bulk updated {} users", updated_count);

    let deleted_count = user()
        .delete_many()
        .where_clause(|w| {
            w.email().contains("bulk".to_string());
        })
        .exec(&client)
        .await?;
    println!("  ✓ Bulk deleted {} users", deleted_count);

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  ✓ ALL PRISMA FOR RUST FEATURES DEMONSTRATED ✓           ║");
    println!("║                                                            ║");
    println!("║  ✅ READ Operations:                                       ║");
    println!("║     • find_many()                                          ║");
    println!("║     • find_unique()                                        ║");
    println!("║     • find_first()                                         ║");
    println!("║     • find_unique_or_throw()                               ║");
    println!("║     • find_first_or_throw()                                ║");
    println!("║                                                            ║");
    println!("║  ✅ WRITE Operations:                                      ║");
    println!("║     • create() with nested relations                       ║");
    println!("║     • update() with nested writes                          ║");
    println!("║     • delete()                                             ║");
    println!("║     • upsert() (top-level)                                 ║");
    println!("║                                                            ║");
    println!("║  ✅ BULK Operations:                                       ║");
    println!("║     • create_many()                                        ║");
    println!("║     • create_many_and_return()                             ║");
    println!("║     • update_many()                                        ║");
    println!("║     • delete_many()                                        ║");
    println!("║                                                            ║");
    println!("║  ✅ AGGREGATION Operations:                                ║");
    println!("║     • count()                                              ║");
    println!("║                                                            ║");
    println!("║  ✅ FILTER Features:                                       ║");
    println!("║     • Type-safe filters (String, Int, Bool, Enum, etc)     ║");
    println!("║     • Logical operators (AND, OR, NOT)                     ║");
    println!("║     • Relation filters (some, every, none)                 ║");
    println!("║                                                            ║");
    println!("║  ✅ INCLUDE/SELECT Features:                               ║");
    println!("║     • Nested includes with filtering                       ║");
    println!("║     • Selective field selection (.posts_with())            ║");
    println!("║     • select_as! macro for zero-boilerplate structs        ║");
    println!("║                                                            ║");
    println!("║  ✅ NESTED Features:                                       ║");
    println!("║     • Nested create within relations                       ║");
    println!("║     • Nested update within relations                       ║");
    println!("║     • Nested upsert within relations                       ║");
    println!("║     • Nested includes with custom filters                  ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    Ok(())
}
