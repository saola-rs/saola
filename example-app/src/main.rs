extern crate prisma_core;
pub mod prisma;

use prisma::client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize the client
    let schema_str = include_str!("../schema.prisma");
    let client = prisma_core::PrismaClient::new(schema_str, "file:./dev.db").await?;

    println!("Client initialized successfully.");

    let unique_email = format!(
        "bob-{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // 2. CREATE a user!
    println!("Inserting a new user...");
    let create_result: client::User = client::user()
        .create(unique_email.clone(), client::Role::USER)
        .data(|u| {
            u.display_name("Bob the Builder".to_string());
            u.posts(|p| {
                p.title("Hello Post".to_string());
                p.status(client::PostStatus::DRAFT);
                p.published(false);
            });
        })
        .exec(&client)
        .await?;

    println!("Created user: {:?}", create_result);

    // 3. UPDATE the user we just created
    println!("Updating user...");
    let update_result: client::User = client::user()
        .update()
        .where_clause(|w| {
            w.email(unique_email.clone());
        })
        .data(|u| {
            u.display_name("Bob the Master Builder".to_string());
        })
        .select(|u| {
            u.id().display_name();
        })
        .exec(&client)
        .await?;

    println!("Update result: {:?}", update_result);

    // 4. FIND the user we just updated
    println!("Finding user...");
    let query = client::user()
        .find_many()
        .where_clause(|u| {
            u.or(|u2| {
                u2.email().eq(unique_email.clone());
                // u2.email().eq("some_other_email@example.com");
            });
        })
        .select(|u| {
            u.id().display_name();
        })
        .include(|i| {
            i.posts(|p| {
                p.all();
            });
        });

    let find_result: Vec<client::User> = query.exec(&client).await?;
    println!("Find result: {:?}", find_result);

    // 5. DELETE the user
    println!("Deleting user...");
    let delete_result: client::User = client::user()
        .delete()
        .where_clause(|w| {
            w.email(unique_email.clone());
        })
        .select(|u| {
            u.id().email();
        })
        .exec(&client)
        .await?;

    println!("Delete result: {:?}", delete_result);

    Ok(())
}
