use std::sync::Arc;
use serenity::{prelude::*, all::{*}};
use crate::handler::*;
use tokio::sync::RwLockWriteGuard;

pub fn check_first_launch(mut data: RwLockWriteGuard<'_, Data>) -> bool {
    if data.first_launch {
        data.first_launch = false;
        return true
    }
    return false
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