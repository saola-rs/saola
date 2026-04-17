/// Comprehensive test suite demonstrating all Saola ORM features
/// Run with: cargo test --lib

saola_macros::init!("schema.prisma");
use saola::*;
use saola_core::prelude::*;

#[tokio::test]
async fn test_create_with_nested_relations() -> anyhow::Result<()> {
    let client = saola::client().await?;
    let unique_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis();

    let user_rec = user()
        .create(format!("test-{}@example.com", unique_ts))
        .data(|d| {
            d.name(format!("Test User {}", unique_ts));
            d.posts(|p| {
                p.create("Nested Post 1".to_string(), |post| {
                    post.content("Content for post 1".to_string());
                });
                p.create("Nested Post 2".to_string(), |post| {
                    post.content("Content for post 2".to_string());
                });
            });
            d.profile(|prof| {
                prof.create(|p| {
                    p.bio(Some("Test bio".to_string()));
                });
            });
        })
        .include(|u| u.posts())
        .include(|u| u.profile())
        .exec(&client)
        .await?;

    assert_eq!(user_rec.email, format!("test-{}@example.com", unique_ts));
    assert_eq!(user_rec.posts.len(), 2);
    Ok(())
}

#[tokio::test]
async fn test_complex_filtering_with_relations() -> anyhow::Result<()> {
    let client = saola::client().await?;
    let unique_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis();

    // Create test data
    user()
        .create(format!("filter-test-{}@example.com", unique_ts))
        .data(|d| {
            d.name("Filter Test User".to_string());
            d.posts(|p| {
                p.create("Post with Rust keyword".to_string(), |post| {
                    post.content("Rust is awesome".to_string());
                });
            });
        })
        .exec(&client)
        .await?;

    // Find users with posts containing "Rust"
    let users = user()
        .find_many()
        .where_clause(|w| {
            w.posts().some(|p| {
                p.title().contains("Rust".to_string());
            });
        })
        .include(|i| i.posts())
        .exec(&client)
        .await?;

    assert!(!users.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_bulk_operations() -> anyhow::Result<()> {
    let client = saola::client().await?;
    let unique_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis();

    // Bulk create
    let count = user()
        .create_many()
        .data(|d| {
            d.email(format!("bulk-1-{}@example.com", unique_ts));
            d.name("Bulk User 1".to_string());
        })
        .data(|d| {
            d.email(format!("bulk-2-{}@example.com", unique_ts));
            d.name("Bulk User 2".to_string());
        })
        .data(|d| {
            d.email(format!("bulk-3-{}@example.com", unique_ts));
            d.name("Bulk User 3".to_string());
        })
        .exec(&client)
        .await?;

    assert_eq!(count, 3);
    Ok(())
}

#[tokio::test]
async fn test_aggregations() -> anyhow::Result<()> {
    let client = saola::client().await?;
    let unique_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis();

    // Create test data
    user()
        .create(format!("agg-test-{}@example.com", unique_ts))
        .data(|d| d.name("Aggregation Test".to_string()))
        .exec(&client)
        .await?;

    // Count users
    let count = user().count().exec(&client).await?;
    assert!(count > 0);

    Ok(())
}

#[tokio::test]
async fn test_pagination_and_ordering() -> anyhow::Result<()> {
    let client = saola::client().await?;
    let unique_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis();

    // Create multiple users
    for i in 0..5 {
        user()
            .create(format!("page-{}-{}@example.com", i, unique_ts))
            .data(|d| d.name(format!("Paginated User {}", i)))
            .exec(&client)
            .await?;
    }

    // Paginated query
    let paginated = user()
        .find_many()
        .order_by(|o| o.email(SortOrder::Asc))
        .take(2)
        .skip(1)
        .exec(&client)
        .await?;

    assert_eq!(paginated.len(), 2);
    Ok(())
}

#[tokio::test]
async fn test_CRUD_operations() -> anyhow::Result<()> {
    let client = saola::client().await?;
    let unique_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis();
    let email = format!("crud-{}@example.com", unique_ts);

    // CREATE
    let created = user()
        .create(email.clone())
        .data(|d| d.name("CRUD User".to_string()))
        .exec(&client)
        .await?;
    let user_id = created.id.clone();

    // READ - find unique
    let found = user()
        .find_unique()
        .where_clause(|w| w.id(user_id.clone()))
        .exec(&client)
        .await?;
    assert!(found.is_some());

    // UPDATE
    let updated = user()
        .update()
        .where_clause(|w| w.id(user_id.clone()))
        .data(|d| d.name("Updated User".to_string()))
        .exec(&client)
        .await?;
    assert_eq!(updated.name, "Updated User");

    // DELETE
    let _deleted = user()
        .delete()
        .where_clause(|w| w.id(user_id))
        .exec(&client)
        .await?;

    Ok(())
}
