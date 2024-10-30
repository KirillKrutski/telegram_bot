mod connection;

use tokio_postgres::{NoTls, Client, Error};

pub async fn establish_db_connection() -> Result<Client, Error>{
    let (client, connection) = tokio_postgres::connection(
        "host=5432 user=user dbname=krutski_bot_database",
        NoTls,
    )
    .await?;

    tokio::spawn(async move{
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn save_message_to_db(
    db_client: &Client,
    user_id: &str,
    message: &str,
) -> Result<(), tokio_postgres::Error> {
    db_client
        .execute(
            "INSERT INTO messages (user_id, message) VALUES ($1, $2)",
            &[&user_id, &message],
        )
        .await?;
    Ok(())
}