use std::env;
use std::sync::Arc;
use serenity::prelude::*;
use crate::handler::{*};
use crate::functions::{*};
mod functions;
mod handler;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(&token, intents)
            .event_handler(Handler)
            .await
            .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<DataKey>(Arc::new(RwLock::new(Data { 
            first_launch: true,
            tank_queue: Arc::new(Mutex::new(Vec::new())),
            healer_queue: Arc::new(Mutex::new(Vec::new())),
            dps_queue: Arc::new(Mutex::new(Vec::new())),
            listen_channel: "mythic-plus-pickup".to_string()
         })));
    }
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}