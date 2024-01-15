use crate::commands;
use crate::functions::*;
use serenity::all::Interaction;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let channel = msg.channel(&ctx).await.expect("Failed to get channel");
        let bot_user_id = ctx.cache.current_user().id;
        if msg.author.id != bot_user_id {
            clean_messages(&ctx, &channel, &bot_user_id).await;
            let channel_name = channel
                .id()
                .name(&ctx)
                .await
                .expect("Error getting channel name");
            let contents_channel_name = get_listen_channel(&ctx)
                .await
                .unwrap()
                .name(&ctx)
                .await
                .expect("Error getting listen channel");
            if channel_name == contents_channel_name {
                let contents = create_message_contents(&ctx).await;
                channel
                    .id()
                    .send_message(&ctx, contents)
                    .await
                    .expect("Error sending message");
            }
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(button) = interaction.clone().message_component() {
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
                    if check_user_in_queue(&ctx, &button, Roles::Dps).await {
                        added_to_queue = add_user_to_queue(&ctx, &button, Roles::Dps).await;
                    }
                }
                "leave" => {
                    remove_from_queue(&ctx, &button).await;
                }
                _ => println!("Button not implemented"),
            }
            if added_to_queue {
                clean_messages(
                    &ctx,
                    channel,
                    &ctx.http.get_current_user().await.unwrap().id,
                )
                .await;
                let contents = create_message_contents(&ctx).await;
                button
                    .channel_id
                    .send_message(&ctx, contents)
                    .await
                    .expect("Error sending message");
            }
        } else if let Interaction::Command(command) = &interaction {
            println!("Received command: {command:#?}");

            let content = match command.data.name.as_str() {
                "notify" => Some(commands::notify::run(&command.data.options())),
                _ => Some("not implemented".to_string()),
            };

            if let Some(content) = content {
                let data =
                    serenity::builder::CreateInteractionResponseMessage::new().content(content);
                let builder = serenity::builder::CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        };
    }
    async fn ready(&self, ctx: Context, _: Ready) {
        let data = initialize_data(&ctx).await.unwrap();
        let data = data.write().await;
        let first_launch = check_first_launch(data);

        for guild_id in ctx.cache.guilds() {
            let commands = guild_id
                .set_commands(&ctx.http, vec![commands::notify::register()])
                .await;
            println!("I created the following commands: {commands:#?}");
        }

        if first_launch.unwrap() {
            let channel = get_listen_channel(&ctx).await.unwrap();
            channel
                .say(&ctx.http, "Bot reloaded")
                .await
                .expect("Failed to send message.");
        }
        println!("Connected");
    }
}
