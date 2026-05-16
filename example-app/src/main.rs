mod saola;

use saola::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = saola::client().await?;

    println!("🚀 Starting Saola Complex Example...");

    // Clean up before starting
    client.post().delete_many().exec().await?;
    client.profile().delete_many().exec().await?;
    client.user().delete_many().exec().await?;

    // 1. Create multiple users using create_many
    println!("\n--- 1. Create Many ---");
    let users_count = client
        .user()
        .create_many()
        .data(|d| {
            d.email("user1@example.com".to_string());
            d.name("User One".to_string());
            d.level(10);
        })
        .data(|d| {
            d.email("user2@example.com".to_string());
            d.name("User Two".to_string());
            d.level(20);
        })
        .data(|d| {
            d.email("user3@example.com".to_string());
            d.name("User Three".to_string());
            d.level(30);
        })
        .exec()
        .await?;
    println!("Created {} users", users_count);

    // 2. Advanced Filtering
    println!("\n--- 2. Advanced Filtering ---");
    let high_level_users = client
        .user()
        .find_many()
        .where_clause(|w| {
            w.level().gt(15);
            w.email().contains("user".to_string());
        })
        .exec()
        .await?;
    println!(
        "Users with level > 15: {:?}",
        high_level_users.iter().map(|u| &u.name).collect::<Vec<_>>()
    );

    // 3. Logical Operators (AND/OR)
    println!("\n--- 3. Logical Operators ---");
    let mixed_users = client
        .user()
        .find_many()
        .where_clause(|w| {
            w.or(|w| {
                w.level().lt(15);
                w.name().starts_with("User Three".to_string());
            });
        })
        .exec()
        .await?;
    println!(
        "Users matching OR condition: {:?}",
        mixed_users.iter().map(|u| &u.name).collect::<Vec<_>>()
    );

    // 4. Ordering and Pagination
    println!("\n--- 4. Ordering and Pagination ---");
    let ordered_users = client
        .user()
        .find_many()
        .order_by(|o| {
            o.level(SortOrder::Desc);
        })
        .take(2)
        .skip(1)
        .exec()
        .await?;
    println!(
        "Ordered users (skip 1, take 2): {:?}",
        ordered_users.iter().map(|u| &u.name).collect::<Vec<_>>()
    );

    // 5. Create Post with nested relation
    println!("\n--- 5. Nested Relation Operations ---");
    let user1 = client
        .user()
        .find_unique()
        .where_unique(|w| {
            w.email("user1@example.com".to_string());
        })
        .exec()
        .await?
        .unwrap();

    client
        .post()
        .create("User 1's Post".to_string(), |u| {
            u.connect(|w| {
                w.id(user1.id.clone());
            });
        })
        .exec()
        .await?;

    // 6. Aggregate
    println!("\n--- 6. Aggregations ---");
    let aggregation = client
        .user()
        .aggregate()
        .count(|c| {
            c._all();
        })
        .sum(|s| {
            s.level();
        })
        .avg(|a| {
            a.level();
        })
        .min(|m| {
            m.level();
        })
        .max(|m| {
            m.level();
        })
        .exec()
        .await?;
    println!("User Aggregations: {:?}", aggregation);

    // 7. Group By
    println!("\n--- 7. Group By ---");
    let groups = client
        .user()
        .group_by()
        .by(|b| {
            b.is_active();
        })
        .count(|c| {
            c._all();
        })
        .avg(|a| {
            a.level();
        })
        .exec()
        .await?;
    println!("User Groups by isActive: {:?}", groups);

    // 8. Atomic Updates
    println!("\n--- 8. Atomic Updates ---");
    let updated_user = client
        .user()
        .update()
        .where_unique(|w| {
            w.email("user1@example.com".to_string());
        })
        .data(|d| {
            d.level_increment(5);
        })
        .exec()
        .await?;
    println!(
        "User 1 level updated (increment 5): {} -> {}",
        user1.level, updated_user.level
    );

    // 9. Upsert
    println!("\n--- 9. Upsert ---");
    let upserted_user = client
        .user()
        .upsert("upsert@example.com".to_string(), "Upsert User".to_string())
        .where_unique(|w| {
            w.email("upsert@example.com".to_string());
        })
        .update(|d| {
            d.name("Updated Upsert User".to_string());
        })
        .exec()
        .await?;
    println!("Upserted user: {:?}", upserted_user.name);
    // 10. Complex Select/Include with Custom Structs
    println!("\n--- 10. Complex Select/Include ---");
    use saola::user;
    saola_core::select!(UserSummary, user, {
        id,
        name,
        email,
    });

    let users_summaries = client.user().find_many().select_as::<UserSummary>().exec().await?;
    println!("Users summaries: {:?}", users_summaries);

    // 11. Transaction with complex logic
    println!("\n--- 11. Complex Transaction ---");
    client
        .transaction(|tx| async move {
            let u = tx
                .user()
                .create("tx_complex@example.com".to_string(), "TX Complex".to_string())
                .exec()
                .await?;

            tx.post()
                .create("TX Post".to_string(), |u_rel| {
                    u_rel.connect(|w| {
                        w.id(u.id.clone());
                    });
                })
                .exec()
                .await?;

            let count = tx
                .post()
                .count()
                .where_clause(|w| {
                    w.user_id().equals(u.id.clone());
                })
                .exec()
                .await?;
            println!("Posts created in transaction for new user: {}", count);

            if count > 0 {
                tx.user()
                    .update()
                    .where_unique(|w| {
                        w.id(u.id.clone());
                    })
                    .data(|d| {
                        d.is_active(true);
                    })
                    .exec()
                    .await?;
            }

            Ok(())
        })
        .await?;

    println!("\n✅ All examples completed successfully!");

    Ok(())
}
