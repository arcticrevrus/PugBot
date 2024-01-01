use std::sync::Arc;
use serenity::{prelude::*, all::{*}};
use crate::handler::*;
use tokio::sync::RwLockWriteGuard;

pub fn check_first_launch(mut data: RwLockWriteGuard<'_, Data>) -> (bool, RwLockWriteGuard<'_, Data>) {
    let first_launch = data.first_launch;
    if first_launch {
        data.first_launch = false;
    }
    (first_launch, data)
}

pub async fn initialize_data(ctx: &Context) -> Arc<RwLock<Data>> {
    let data_read = ctx.data.read().await;
    data_read.get::<DataKey>()
        .expect("Expected Data in TypeMap.")
        .clone()
}

pub async fn get_channel_listing(ctx: &Context) -> Vec<(ChannelId, GuildChannel)> {
    let mut channels: Vec<(ChannelId, GuildChannel)> = Vec::new();
    for guild in ctx.cache.guilds() {
        for channel in guild.channels(&ctx).await.expect("Failed to get channel listing.") {
            channels.push(channel)
        }
    }
    return channels
}

pub async fn get_listen_channel(ctx: &Context, data: &Data) -> ChannelId {
    for channel in get_channel_listing(&ctx).await {
        if channel.1.name == data.listen_channel {
            return channel.1.id;
        }
    }
    panic!("get_listen_channel failed to return channel")
}

pub async fn clean_messages(ctx: &Context, channel: &Channel, user: &UserId) {
    for message in channel.id().messages(&ctx, GetMessages::new()).await.unwrap() {
        if &message.author.id == user {
            if message.content.contains("The current queue") {
                message.delete(&ctx).await.unwrap();
            }
        }
    }
}

pub fn create_message_contents() -> CreateMessage {
    let message = 
">>> The current queue is: 
<:tank:444634700523241512> : blah
<:heal:444634700363857921> : blah
<:dps:444634700531630094> :  blah";
    let buttons = make_buttons();
    let mut contents = CreateMessage::new().content(message);

    for button in &buttons {
        contents = contents.button(button.clone())
    }
    return contents
}

pub fn make_buttons() -> Vec<CreateButton> {
    let tank_button = CreateButton::new("add_tank")
    .label("Tank")
    .style(ButtonStyle::Primary);
let healer_button = CreateButton::new("add healer")
    .label("Healer")
    .style(ButtonStyle::Success);
let dps_button = CreateButton::new("add_dps")
    .label("DPS")
    .style(ButtonStyle::Danger);
let leave_button = CreateButton::new("leave")
    .label("Leave")
    .style(ButtonStyle::Secondary);
    vec![tank_button, healer_button, dps_button, leave_button]
}