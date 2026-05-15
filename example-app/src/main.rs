mod saola;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = saola::client().await?;

    // 1. Create User with all scalar types
    let email = format!(
        "user_{}@example.com",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let user = client
        .user()
        .create(email.clone(), "Complex User".to_string())
        .data(|d| {
            d.level(10);
            d.is_active(true);
        })
        .exec()
        .await?;

    println!("Created User with score/level: {:?}", user);

    // 2. Create Post relation
    let post = client
        .post()
        .create("My first post".to_string(), |u| {
            u.connect(|w| {
                w.email(email.clone());
            });
        })
        .data(|d| {
            d.content("This is the content".to_string());
            d.published(true);
        })
        .include(|i| i.user())
        .exec()
        .await?;

    println!("Created Post: {:?}", post);

    use saola::{post, user};
    saola_core::select!(PostSelect, post, {
        id,
        user: saola::User,
    });
    saola_core::select!(UserSelect, user, {
        id,
        email,
        posts: Vec<PostSelect>,
        profile: Option<saola::Profile>,
    });

    let users = client.user().find_many().select_as::<UserSelect>().exec().await?;

    println!("Total users (select_as): {}, \n {:?}", users.len(), users);

    let users_partial = client
        .user()
        .find_many()
        .select::<::saola_core::serde_json::Value, _>(|s| {
            s.id();
            s.email();
        })
        .exec()
        .await?;

    println!("Total users (select): {}, \n {:?}", users_partial.len(), users_partial);

    saola_core::select!(UserSubset, user, {
        id,
        email,
    });

    let users_subset = client
        .user()
        .find_many()
        .select::<UserSubset, _>(|s| {
            s.id();
            s.email();
        })
        .exec()
        .await?;

    println!(
        "Total users (select into subset): {}, \n {:?}",
        users_subset.len(),
        users_subset
    );

    // Example of model_as in include
    let posts_with_user = client
        .post()
        .find_many()
        .include(|i| i.user_as::<UserSubset>())
        .exec()
        .await?;

    println!(
        "Posts with included user (model_as): {:?}, \n {:?}",
        posts_with_user.len(),
        posts_with_user
    );

    // 4. Delete the post
    let deleted_post = client
        .post()
        .delete()
        .where_unique(|w| {
            w.id(post.id.clone());
        })
        .exec()
        .await?;

    println!("Deleted Post: {:?}", deleted_post);

    // 5. Delete the user
    let deleted_user = client
        .user()
        .delete()
        .where_unique(|w| {
            w.email(email.clone());
        })
        .exec()
        .await?;

    println!("Deleted User: {:?}", deleted_user);

    // 6. Transaction example
    client
        .transaction(|tx| async move {
            let new_user = tx
                .user()
                .create("tx_user@example.com".to_string(), "TX User".to_string())
                .exec()
                .await?;
            println!("Created user in transaction: {:?}", new_user.email);

            tx.user()
                .delete()
                .where_unique(|w| {
                    w.email("tx_user@example.com".to_string());
                })
                .exec()
                .await?;
            println!("Deleted user in transaction");

            Ok(())
        })
        .await?;

    Ok(())
}
