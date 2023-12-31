
use std::sync::Arc;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::all::User;
use crate::functions::{*};

pub struct Data {
    pub first_launch: bool,
    pub tank_queue: Arc<Mutex<Vec<Player>>>,
    pub healer_queue: Arc<Mutex<Vec<Player>>>,
    pub dps_queue: Arc<Mutex<Vec<Player>>>,
}

pub struct Player {
    pub name: User,
    pub role: Roles,
}

pub enum Roles {
    Tank,
    Healer,
    DPS,
}

impl TypeMapKey for DataKey {
    type Value = Arc<RwLock<Data>>;
}

pub struct DataKey;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
    async fn ready(&self, ctx: Context, _: Ready) {
        let data = {
            let data_read = ctx.data.read().await;
            data_read.get::<DataKey>().expect("Expected Data in TypeMap.").clone()
        };

        let mut data = data.write().await;
        if data.first_launch {
            data.first_launch = false;
            let channels = get_channel_listing(&ctx).await;
            for channel in channels {
                if channel.1.name == "mythic-plus-pickup" {
                    channel.1.id.say(&ctx.http, "Bot reloaded").await.expect("Failed to send message.");
                }
            }
        println!("Connected")
        }
    }
}