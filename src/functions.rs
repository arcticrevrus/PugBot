use std::sync::Arc;
use serenity::{prelude::*, all::{*}};
use crate::handler::*;

pub async fn get_channel_listing(ctx: &Context) -> Vec<(ChannelId, GuildChannel)> {
    let mut channels: Vec<(ChannelId, GuildChannel)> = Vec::new();
    for guild in ctx.cache.guilds() {
        for channel in guild.channels(&ctx).await.expect("Failed to get channel listing.") {
            channels.push(channel)
        }
    }
    return channels
}
