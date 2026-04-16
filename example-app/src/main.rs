prisma_macros::init!("schema.prisma");
use db::*;
use prisma_core::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Client Initialization
    let client = db::client().await?;
    println!("✓ Client initialized successfully");

    let unique_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let unique_email = format!("user-{}@example.com", unique_ts);

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

    // 10. BULK Operations
    println!("\n[9. Bulk Operations]");
    let created_users_count = user()
        .create_many()
        .data(|d| {
            d.email(format!("bulk-1-{}@example.com", unique_ts));
        })
        .data(|d| {
            d.email(format!("bulk-2-{}@example.com", unique_ts));
        })
        .exec(&client)
        .await?;
    println!("  ✓ Bulk created {} users", created_users_count);

    // Test CreateManyAndReturn
    let created_users_with_data = user()
        .create_many_and_return()
        .data(|d| {
            d.email(format!("bulk-ret1-{}@example.com", unique_ts));
        })
        .data(|d| {
            d.email(format!("bulk-ret2-{}@example.com", unique_ts));
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

    let updated_users = user()
        .update_many_and_return()
        .where_clause(|w| {
            w.email().contains("bulk-ret".to_string());
        })
        .data(|d| {
            d.display_name("Updated Bulk User".to_string());
        })
        .exec(&client)
        .await?;

    println!("  ✓ Bulk updated and returned {} users", updated_users.len());
    for u in &updated_users {
        println!("    - {} (id: {})", u.email, u.id);
    }

    let deleted_count = user()
        .delete_many()
        .where_clause(|w| {
            w.email().contains("bulk".to_string());
        })
        .exec(&client)
        .await?;
    println!("  ✓ Bulk deleted {} users", deleted_count);

    // 11. AGGREGATE Operation
    println!("\n[10. Aggregate Operation]");
    let aggregate_result = user()
        .aggregate()
        .count(|c| {
            c._all().email();
        })
        .exec(&client)
        .await?;
    println!("  ✓ Aggregate count: {:?}", aggregate_result._count);

    // 11. GROUP BY Operation
    println!("\n[11. GroupBy Operation]");
    let group_by_results = user()
        .group_by()
        .by(|b| {
            b.role();
        })
        .count(|c| {
            c._all();
        })
        .order_by(|o| {
            o.role(SortOrder::Asc);
        })
        .take(1)
        .exec(&client)
        .await?;
    println!("  ✓ GroupBy results (ordered by role, take 1): {:?}", group_by_results);

    // TRANSACTION Example
    println!("\n[9. Transaction Example - Create Multiple Records Atomically]");
    let unique_ts_clone = unique_ts.clone();
    let tx_result = client
        .transaction(|tx| async move {
            let tx_user = user()
                .create(format!("tx-user-{}@example.com", unique_ts_clone))
                .exec(&tx)
                .await?;

            let _post = post()
                .create(format!("Transaction Post - {}", unique_ts_clone), |u| {
                    u.connect(|w| {
                        w.id(tx_user.id.clone());
                    });
                })
                .exec(&tx)
                .await?;

            Ok(tx_user)
        })
        .await?;
    println!("  ✓ Transaction committed: user {} created with posts", tx_result.email);

    // TRANSACTION Rollback Example
    println!("\n[10. Transaction Rollback Example - Failed Transaction]");
    let unique_ts_fail = unique_ts.clone();
    let rollback_result: anyhow::Result<()> = client
        .transaction(|tx| async move {
            let fail_user = user()
                .create(format!("fail-user-{}@example.com", unique_ts_fail))
                .exec(&tx)
                .await?;

            println!("    - Created user: {} (transaction in progress)", fail_user.email);

            // Simulate error
            Err(anyhow::anyhow!("Simulated error: invalid data detected"))
        })
        .await;

    match rollback_result {
        Ok(_) => println!("  ✗ Transaction succeeded (unexpected)"),
        Err(e) => {
            println!("  ✓ Transaction rolled back due to: {}", e);

            // Verify the user was NOT created (transaction was rolled back)
            let all_users = user().find_many().exec(&client).await?;
            let fail_user_exists = all_users.iter().any(|u| u.email.contains("fail-user"));
            if fail_user_exists {
                println!("  ✗ ERROR: User exists after rollback (rollback failed!)");
            } else {
                println!("  ✓ Verified: User was NOT persisted (rollback successful!)");
            }
        }
    }

    // MACRO-BASED Transaction Example (simpler syntax!)
    println!("\n[11. Transaction using Macro (cleanest syntax - recommended)]");
    let unique_ts_macro = unique_ts.clone();
    let macro_result = prisma_core::tx!(client, tx, {
        let user_via_macro = user()
            .create(format!("macro-user-{}@example.com", unique_ts_macro))
            .data(|d| {
                d.posts(|p| {
                    p.create(format!("Post for macro user - {}", unique_ts_macro), |_| {});
                });
            })
            .exec(&tx)
            .await?;
        println!("  ✓ User created via macro: {}", user_via_macro.email);
        Ok(user_via_macro)
    })?;

    // CONFIGURATION-BASED Transaction Example with custom timeout
    println!("\n[12. Transaction with Custom Configuration]");
    let config = prisma_core::TransactionConfig::default()
        .timeout_ms(30000) // 30 second timeout instead of 60
        .isolation_level(IsolationLevel::Serializable);

    let unique_ts_config = unique_ts.clone();
    let config_user = client
        .transaction_with_config(config, |tx| {
            Box::pin(async move {
                let user_with_config = user()
                    .create(format!("config-user-{}@example.com", unique_ts_config))
                    .exec(&tx)
                    .await?;
                println!("  ✓ User created with custom config: {}", user_with_config.email);
                Ok(user_with_config)
            })
        })
        .await?;

    // MACRO with Configuration Example
    println!("\n[13. Transaction Macro with Custom Configuration (advanced)]");
    let config_macro = prisma_core::TransactionConfig::default()
        .max_wait(2000)
        .timeout_ms(20000);

    let unique_ts_macro_config = unique_ts.clone();
    prisma_core::tx_config!(client, tx, config_macro, {
        let user_macro_config = user()
            .create(format!("macro-config-{}@example.com", unique_ts_macro_config))
            .exec(&tx)
            .await?;
        println!("  ✓ User created with macro and config: {}", user_macro_config.email);
        Ok(user_macro_config)
    })?;

    // 14. Pagination and Ordering
    println!("\n[14. Pagination and Ordering]");
    let paginated_users = user()
        .find_many()
        .order_by(|o| {
            o.email(SortOrder::Desc);
        })
        .take(2)
        .skip(1)
        .exec(&client)
        .await?;
    println!(
        "  ✓ Paginated users (desc, take 2, skip 1): {:?}",
        paginated_users.iter().map(|u| &u.email).collect::<Vec<_>>()
    );

    // 15. Nested Pagination and Ordering
    println!("\n[15. Nested Pagination and Ordering]");
    let user_with_paginated_posts = user()
        .find_first()
        .where_clause(|w| {
            w.email().eq(format!("macro-user-{}@example.com", unique_ts));
        })
        .include(|i| {
            i.posts_with(|p| {
                p.order_by(|o| {
                    o.title(SortOrder::Asc);
                })
                .take(1);
                p.empty()
            })
        })
        .exec(&client)
        .await?;
    if let Some(u) = user_with_paginated_posts {
        println!("  ✓ User {} has {} paginated posts", u.email, u.posts.len());
        for p in u.posts {
            println!("    - Post: {}", p.title);
        }
    }

    // 16. Cursor-based pagination
    println!("\n[16. Cursor-based Pagination]");
    let first_user = user()
        .find_first()
        .order_by(|o| {
            o.email(SortOrder::Asc);
        })
        .exec(&client)
        .await?
        .unwrap();
    let users_after_cursor = user()
        .find_many()
        .order_by(|o| {
            o.email(SortOrder::Asc);
        })
        .cursor(|c| {
            c.id(first_user.id.clone());
        })
        .take(2)
        .exec(&client)
        .await?;
    println!(
        "  ✓ Users after cursor (id: {}): {:?}",
        first_user.id,
        users_after_cursor.iter().map(|u| &u.email).collect::<Vec<_>>()
    );

    println!("\n[All supported features demonstrated] ✓\n");
    Ok(())
}
