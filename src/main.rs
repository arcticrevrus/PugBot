use crate::functions::*;
use serenity::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::sync::Arc;
mod commands;
mod functions;
mod handler;
mod tests;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<DataKey>(Arc::new(RwLock::new(Data {
            first_launch: true,
            queue: Arc::new(Mutex::new(VecDeque::new())),
            listen_channel: "mythic-plus-pickup".to_string(),
            user_settings: Arc::new(Mutex::new(HashMap::new())),
        })));
    }
    let client_data = client.data.clone();
    let client_http = client.http.clone();

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            match check_timeouts(&client_data, &client_http).await {
                Ok(()) => (),
                Err(error) => println!("{error}"),
            };
        }
    });

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
