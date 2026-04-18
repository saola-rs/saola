saola_macros::init!("schema.prisma");
use saola::*;
use saola_core::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 Saola ORM - Real-world Blog Example\n");

    let client = saola::client().await?;

    // ============ SCENARIO 1: Blog Writer Creates a Post ============
    println!("Scenario 1: Creating a blog post with profile & metadata\n");

    let unique_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let unique_email = format!("alice-{}@blog.com", unique_ts);

    // email and name are required. password has no default but is optional (?), so it's not a positional arg.
    let writer = user()
        .create(unique_email.clone(), "Alice Writer".to_string())
        .data(|d| {
            d.role(Role::USER);
            d.password("hello".to_string());
            // Create profile and posts in one call!
            d.profile(|prof| {
                prof.create(|p| {
                    p.bio("Tech writer & developer".to_string());
                    p.website("alice.dev".to_string());
                });
            });
            d.posts(|p| {
                p.create(
                    "Getting Started with Rust ORM".to_string(),
                    "Learn how to build type-safe database queries...".to_string(),
                    |post| {
                        post.status(PostStatus::PUBLISHED);
                        post.published(true);
                    },
                );
                p.create(
                    "Advanced Query Patterns".to_string(),
                    "Master complex queries with relations filtering...".to_string(),
                    |_| {},
                );
            });
        })
        .include(|u| u.posts())
        .include(|u| u.profile())
        .exec(&client)
        .await?;

    println!(
        "Created user: {} with {} posts\n {:?}",
        writer.name,
        writer.posts.len(),
        writer
    );

    // ============ SCENARIO 2: Find Popular Blog Posts ============
    println!("Scenario 2: Finding published posts by active authors\n");

    let published_posts = post()
        .find_many()
        .where_clause(|w| {
            // Find published posts from active users
            w.published().eq(true);
            w.user().is(|u| {
                u.is_active().eq(true);
            });
        })
        .include(|i| i.user())
        .order_by(|o| {
            o.created_at(SortOrder::Desc);
        })
        .take(10)
        .exec(&client)
        .await?;

    for post_item in &published_posts {
        println!("  {} by {}", post_item.title, post_item.user.name);
    }
    println!();

    // ============ SCENARIO 3: Blog Analytics ============
    println!("Scenario 3: Analytics on blog activity\n");

    let total_posts = post()
        .count()
        .where_clause(|w| {
            w.published().eq(true);
        })
        .exec(&client)
        .await?;

    let active_writers = user()
        .count()
        .where_clause(|w| {
            w.is_active().eq(true);
            w.posts().some(|p| {
                p.published().eq(true);
            });
        })
        .exec(&client)
        .await?;

    println!("  Total published posts: {}", total_posts);
    println!("  Active writers: {}\n", active_writers);

    // ============ SCENARIO 4: Find Popular Tech Content ============
    println!("Scenario 4: Finding tech-related content\n");

    let tech_posts = post()
        .find_many()
        .where_clause(|w| {
            // Posts containing tech keywords
            w.or(|or| {
                or.title().contains("Rust".to_string());
                or.title().contains("ORM".to_string());
                or.content().contains("programming".to_string());
            });
            // Only published
            w.published().eq(true);
        })
        .include(|i| i.user())
        .order_by(|o| {
            o.created_at(SortOrder::Desc);
        })
        .take(5)
        .exec(&client)
        .await?;

    for item in &tech_posts {
        println!("   {} - by {}", item.title, item.user.name);
    }
    println!();

    // ============ SCENARIO 5: Update Post Status ============
    println!("Scenario 5: Publishing a draft post\n");

    let draft_posts = post()
        .find_many()
        .where_clause(|w| {
            w.status().eq(PostStatus::DRAFT);
        })
        .take(1)
        .exec(&client)
        .await?;

    if let Some(draft) = draft_posts.first() {
        let published = post()
            .update()
            .where_clause(|w| {
                w.id(draft.id.clone());
            })
            .data(|d| {
                d.status(PostStatus::PUBLISHED);
                d.published(true);
                d.views_increment(1);
            })
            .exec(&client)
            .await?;

        println!("  ✅ Published: {} (Views updated atomically: {})", published.title, published.views);

    }

    // ============ SCENARIO 6: Pagination & Sorting ============
    println!("Scenario 6: Listing posts with pagination\n");

    let paginated = post()
        .find_many()
        .where_clause(|w| {
            w.published().eq(true);
        })
        .order_by(|o| {
            o.created_at(SortOrder::Desc);
        })
        .take(3)
        .skip(0)
        .exec(&client)
        .await?;

    println!("  Latest {} posts:", paginated.len());
    for post_item in &paginated {
        println!("    • {}", post_item.title);
    }
    println!();

    // ============ SCENARIO 7: Complex Filtering ============
    println!("Scenario 7: Finding moderators with recent posts\n");

    let moderator_posts = user()
        .find_many()
        .where_clause(|w| {
            w.role().eq(Role::MODERATOR);
            w.is_active().eq(true);
            w.posts().some(|p| {
                p.published().eq(true);
            });
        })
        .include(|i| i.posts())
        .exec(&client)
        .await?;

    for user_item in &moderator_posts {
        println!("  {} (Moderator) - {} posts", user_item.name, user_item.posts.len());
    }
    println!();

    // ============ SCENARIO 8: Select Projection ============
    println!("Scenario 8: Fetching only specific fields (zero boilerplate)\n");

    let minimal_posts = post()
        .find_many()
        .select_as(saola::select_as!({
            id: String,
            title: String,
            views: i32,
            user: {
                email: String,
                password?: String
            }
        }))
        .where_clause(|w| {
            w.published().eq(true);
        })
        .take(1)
        .exec(&client)
        .await?;

    println!("  Selected partial data: {:?}", minimal_posts.first());
    println!();

    Ok(())
}
