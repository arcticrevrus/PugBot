
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
            let user = &button.user;
            let button_id = &button.data.custom_id;
    
            match button_id.as_str() {
                "add_tank" => {
                    add_user_to_queue(&ctx, &user, "tank".to_owned()).await;
                    clean_messages(&ctx, &button.channel_id.to_channel(&ctx.http).await.unwrap(), &ctx.http.get_current_user().await.unwrap().id).await;
                    let contents = create_message_contents(&ctx).await;
                    button.channel_id.send_message(&ctx, contents).await.expect("Error sending message");
                    button.channel_id.say(&ctx.http, format!("{} has added to tank queue.", user.global_name.as_ref().expect("user does not have global name"))).await.expect("Error sending message");
                }
                "add_healer" => {
                    add_user_to_queue(&ctx, &user, "healer".to_owned()).await;
                    clean_messages(&ctx, &button.channel_id.to_channel(&ctx.http).await.unwrap(), &ctx.http.get_current_user().await.unwrap().id).await;
                    let contents = create_message_contents(&ctx).await;
                    button.channel_id.send_message(&ctx, contents).await.expect("Error sending message");
                    button.channel_id.say(&ctx.http, format!("{} has added to healer queue.", user.global_name.as_ref().expect("user does not have global name"))).await.expect("Error sending message");
                }
                "add_dps" => {
                    add_user_to_queue(&ctx, &user, "dps".to_owned()).await;
                    clean_messages(&ctx, &button.channel_id.to_channel(&ctx.http).await.unwrap(), &ctx.http.get_current_user().await.unwrap().id).await;
                    let contents = create_message_contents(&ctx).await;
                    button.channel_id.send_message(&ctx, contents).await.expect("Error sending message");
                    button.channel_id.say(&ctx.http, format!("{} has added to dps queue.", user.global_name.as_ref().expect("user does not have global name"))).await.expect("Error sending message");
                }
                "leave" => {
                    remove_from_queue(&ctx, user).await;
                    clean_messages(&ctx, &button.channel_id.to_channel(&ctx.http).await.unwrap(), &ctx.http.get_current_user().await.unwrap().id).await;
                    let contents = create_message_contents(&ctx).await;
                    button.channel_id.send_message(&ctx, contents).await.expect("Error sending message");
                    button.channel_id.say(&ctx.http, format!("{} has left all queues.", user.global_name.as_ref().expect("user does not have global name"))).await.expect("Error sending message");

                }
                _ => println!("Button not implemented"),
            }
            button.defer(&ctx.http).await.expect("Error deferring interaction");
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