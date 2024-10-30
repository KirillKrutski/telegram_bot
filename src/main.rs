pub mod database;
use teloxide::prelude::*;
use teloxide::types::Message;
use crate::database::save_message_to_db;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let bot = Bot::new("7817612148:AAHbGzniv7ZhCE7WilhjhXme8WeeyWq5HLw");
    let db_client = database::establish_db_connection().await.expect("Failed to connect to the database");

    teloxide::repl(bot, |message| async move{
        match message.text() {
            Some(text) => {
                if text == "/start"{
                    message.answer("Welcome to your BOT!").await?;
                } else {
                    message.answer(format!("You said: {}", text)).await?;

                    // save message to database
                    save_message_to_db(&db_client, &message.from().unwrap().id.to_string(), &text)
                    .await
                    .expect("Failed to save message to the database");
                }
            }
            None => {
                message.answer("I can only handle text message.").await?;
            }
        }
        respond(())
    })
    .await;
    Ok(())
}
