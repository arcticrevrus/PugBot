
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
        let data = initialize_data(&ctx).await;
        let mut data = data.write().await;
        let channel = msg.channel(&ctx).await.expect("Failed to get channel");
    }
    async fn ready(&self, ctx: Context, _: Ready) {
        let data = initialize_data(&ctx).await;
        let mut data = data.write().await;
        if check_first_launch(data) {
            for channel in get_channel_listing(&ctx).await {
                if channel.1.name == "mythic-plus-pickup" {
                    channel.1.id.say(&ctx.http, "Bot reloaded").await.expect("Failed to send message.");
                }
            }
        println!("Connected")
        }
    }
}