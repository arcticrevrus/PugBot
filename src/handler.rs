
use std::sync::Arc;
use serenity::all::Interaction;
use serenity::builder::CreateInteractionResponse;
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
    pub listen_channel: String
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
        let bot_user_id = {
            let current_user = ctx.cache.current_user();
            current_user.id
        };
        let data = initialize_data(&ctx).await;
        let data = data.write().await;
        let channel = msg.channel(&ctx).await.expect("Failed to get channel");
        if msg.author.id != bot_user_id {
            clean_messages(&ctx, &channel, &bot_user_id).await;
            if channel.id().name(&ctx).await.unwrap() == get_listen_channel(&ctx, &data).await.name(&ctx).await.unwrap() {
                let contents = create_message_contents();
                channel.id().send_message(&ctx, contents).await.unwrap();
            }
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction.id() {
            add_tank => {
                println!("tank");
            }
            add_healer => println!("healer"),
            add_dps => println!("dps"),
            leave => println!("leave"),
            _ => println!("not implemented")
        }
        
    }
    async fn ready(&self, ctx: Context, _: Ready) {
        let data = initialize_data(&ctx).await;
        let data = data.write().await;
        let (first_launch, data) = check_first_launch(data);

        if first_launch {
            let channel = get_listen_channel(&ctx, &data).await;
            channel.say(&ctx.http, "Bot reloaded").await.expect("Failed to send message.");
        }
        println!("Connected");
    }
}