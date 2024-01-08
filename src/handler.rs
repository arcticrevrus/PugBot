
use serenity::all::Interaction;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use crate::functions::{*};

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let channel = msg.channel(&ctx).await.expect("Failed to get channel");
        let bot_user_id = ctx.cache.current_user().id;
        if msg.author.id != bot_user_id {
            clean_messages(&ctx, &channel, &bot_user_id).await;
            if channel.id().name(&ctx).await.expect("Error getting channel name") == get_listen_channel(&ctx).await.name(&ctx).await.expect("Error getting listen channel") {
                let contents = create_message_contents(&ctx).await;
                channel.id().send_message(&ctx, contents).await.expect("Error sending message");
            }
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(button) = interaction.message_component() {
            let button_id = &button.data.custom_id;
            let channel = &button.channel_id.to_channel(&ctx.http).await.unwrap();
            let mut added_to_queue = false;
            
            match button_id.as_str() {
                "add_tank" => {
                    if check_user_in_queue(&ctx, &button, Roles::Tank).await {
                        added_to_queue = add_user_to_queue(&ctx, &button, Roles::Tank).await;
                    }
                }
                "add_healer" => {
                    if check_user_in_queue(&ctx, &button, Roles::Healer).await {
                        added_to_queue = add_user_to_queue(&ctx, &button, Roles::Healer).await;
                    }
                }
                "add_dps" => {
                    if check_user_in_queue(&ctx, &button, Roles::DPS).await {
                        added_to_queue = add_user_to_queue(&ctx, &button, Roles::DPS).await;
                    }
                }
                "leave" => {
                    remove_from_queue(&ctx, &button).await;
                }
                _ => println!("Button not implemented"),
            }
            if added_to_queue {
                clean_messages(&ctx, &channel, &ctx.http.get_current_user().await.unwrap().id).await;
                let contents = create_message_contents(&ctx).await;
                button.channel_id.send_message(&ctx, contents).await.expect("Error sending message");
            }
        } 
    }
    async fn ready(&self, ctx: Context, _: Ready) {
        let data = initialize_data(&ctx).await;
        let data = data.write().await;
        let first_launch = check_first_launch(data);

        if first_launch {
            let channel = get_listen_channel(&ctx).await;
            channel.say(&ctx.http, "Bot reloaded").await.expect("Failed to send message.");
        }
        println!("Connected");
    }
}