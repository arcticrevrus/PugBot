
use std::ops::Deref;
use serenity::all::Interaction;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use crate::functions::{*};



#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = initialize_data(&ctx).await;
        let data = data.write().await;
        let channel = msg.channel(&ctx).await.expect("Failed to get channel");
        let bot_user_id = ctx.cache.current_user().id;
        if msg.author.id != bot_user_id {
            clean_messages(&ctx, &channel, &bot_user_id).await;
            if channel.id().name(&ctx).await.unwrap() == get_listen_channel(&ctx, data.deref()).await.name(&ctx).await.unwrap() {
                // Acquire locks, get lengths, and release locks
                let tank_length = data.tank_queue.lock().await.len();
                let healer_length = data.healer_queue.lock().await.len();
                let dps_length = data.dps_queue.lock().await.len();
    
                // Call create_message_contents with lengths
                let contents = create_message_contents(tank_length, healer_length, dps_length);
                channel.id().send_message(&ctx, contents).await.unwrap();
            }
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(button) = interaction.message_component() {
            let data = initialize_data(&ctx).await;
            let data = data.write().await;
            let user = &button.user;
            let button_id = &button.data.custom_id;
    
            match button_id.as_str() {
                "add_tank" => {
                    add_user_to_queue(&ctx, &user, "tank".to_owned()).await;
                    button.channel_id.say(&ctx.http, format!("{} has added to tank queue.", user.global_name.as_ref().unwrap())).await.unwrap();
                }
                "add_healer" => {
                    println!("Healer");
                }
                "add_dps" => {
                    println!("DPS");
                }
                "leave" => {
                    println!("Leave");
                }
                _ => println!("Button not implemented"),
            }
            button.defer(&ctx.http).await.unwrap();
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