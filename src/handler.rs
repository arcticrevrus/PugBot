use crate::commands;
use crate::functions::*;
use serenity::all::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let data = initialize_data(&ctx).await.unwrap();
        let data = data.write().await;
        let channel = msg.channel(&ctx).await.expect("Failed to get channel");
        let bot_user_id = ctx.cache.current_user().id;
        let queue = data.queue.lock().await;

        if msg.author.id != bot_user_id {
            clean_messages(&ctx, &channel, &bot_user_id).await;
            let channel_name = channel
                .id()
                .name(&ctx)
                .await
                .expect("Error getting channel name");
            let contents_channel_name = get_listen_channel(&ctx, &data)
                .await
                .unwrap()
                .name(&ctx)
                .await
                .expect("Error getting listen channel");
            if channel_name == contents_channel_name {
                let contents = create_message_contents(queue).await;
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
            let mut left_queue = false;
            let mut added_to_queue = false;
            let data = initialize_data(&ctx).await.unwrap();
            let data = data.write().await;
            let mut queue = data.queue.lock().await;
            let user = &button.user;
            let guild = &button.guild_id.unwrap();
            let player_display_name = get_display_name(&ctx, user, guild).await;
            let mut role_string = String::new();
            match button_id.as_str() {
                "add_tank" => {
                    if check_user_in_queue(&queue, user, Roles::Tank).await {
                        queue.push_back(create_player(user.id, Roles::Tank));
                        added_to_queue = true;
                        role_string = "Tank".to_string();
                    }
                }
                "add_healer" => {
                    if check_user_in_queue(&queue, user, Roles::Healer).await {
                        queue.push_back(create_player(user.id, Roles::Healer));
                        added_to_queue = true;
                        role_string = "Healer".to_string()
                    }
                }
                "add_dps" => {
                    if check_user_in_queue(&queue, user, Roles::Dps).await {
                        queue.push_back(create_player(user.id, Roles::Dps));
                        added_to_queue = true;
                        role_string = "DPS".to_string()
                    }
                }
                "leave" => {
                    queue.retain(|p| p.id != user.id);
                    channel
                        .id()
                        .say(
                            &ctx.http,
                            format!("{} has left the queue.", player_display_name),
                        )
                        .await
                        .expect("Error sending message");
                    left_queue = true;
                }
                _ => println!("Button not implemented"),
            }
            if added_to_queue {
                let message =
                    serenity::all::CreateInteractionResponseMessage::new().content(format!(
                        "{} has been added to the queue as {}",
                        player_display_name, role_string
                    ));
                let response = CreateInteractionResponse::Message(message);
                button.create_response(&ctx.http, response).await.unwrap();
                let added_to_queue = check_group_found(&mut queue);
                if added_to_queue.is_some() {
                    let message = added_to_queue.unwrap();
                    channel.id().say(&ctx.http, message).await.unwrap();
                }
            }
            clean_messages(
                &ctx,
                channel,
                &ctx.http.get_current_user().await.unwrap().id,
            )
            .await;
            let contents = create_message_contents(queue).await;
            button
                .channel_id
                .send_message(&ctx, contents)
                .await
                .expect("Error sending message");
            if !added_to_queue && !left_queue {
                let message = serenity::all::CreateInteractionResponseMessage::new()
                    .content("You are already in the queue.")
                    .flags(InteractionResponseFlags::EPHEMERAL);
                let response = CreateInteractionResponse::Message(message);
                button.create_response(&ctx.http, response).await.unwrap();
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
        let mut data = data.write().await;
        let first_launch = check_first_launch(&mut data);
        println!("{:?}", first_launch);

        for guild_id in ctx.cache.guilds() {
            let commands = guild_id
                .set_commands(&ctx.http, vec![commands::notify::register()])
                .await;
            println!("Created commands: {commands:?}");
        }

        if first_launch.unwrap() {
            println!("First Launch Detected");
            let channel = get_listen_channel(&ctx, &data).await.unwrap();
            channel
                .say(&ctx.http, "Bot reloaded")
                .await
                .expect("Failed to send message.");
        }
        println!("Connected");
    }
}
