use crate::usersettings::{get_user_settings, set_user_settings};
use serenity::{all::*, prelude::*};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{MutexGuard, RwLockWriteGuard};

pub struct Data {
    pub first_launch: bool,
    pub queue: Arc<Mutex<VecDeque<Player>>>,
    pub listen_channel: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Player {
    pub id: UserId,
    pub role: Roles,
    pub timeout: Duration,
    pub timestamp: SystemTime,
}
#[derive(PartialEq, Clone, Debug)]
pub struct UserSettings {
    timeout: Duration,
    notify: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Roles {
    Tank,
    Healer,
    Dps,
}

impl TypeMapKey for DataKey {
    type Value = Arc<RwLock<Data>>;
}

pub struct DataKey;

pub struct Handler;

pub async fn check_timeouts(
    data: &Arc<serenity::prelude::RwLock<TypeMap>>,
    http: &Arc<Http>,
    cache: &Arc<Cache>,
) -> Result<(), Error> {
    let data = data.read().await;
    let data = data
        .get::<DataKey>()
        .expect("Expected Data in TypeMap.")
        .clone();
    let data = data.write().await;
    let mut queue = data.queue.lock().await;
    let elapsed_players: Vec<_> = queue
        .iter()
        .filter(|player| player.timestamp.elapsed().unwrap() >= player.timeout)
        .map(|player| player.id)
        .collect();
    queue.retain(|player| !elapsed_players.contains(&player.id));
    for player in elapsed_players {
        let channel = player.create_dm_channel(http).await.unwrap();
        channel
            .say(http, "You have timed out of the queue.")
            .await
            .unwrap();
    }
    let content = create_update_contents(&queue);
    update_message(http, cache, &data, content).await;
    Ok(())
}

pub fn check_first_launch(data: &mut RwLockWriteGuard<'_, Data>) -> Result<bool, Error> {
    if data.first_launch {
        data.first_launch = false;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn initialize_data(ctx: &Context) -> Result<Arc<RwLock<Data>>, Error> {
    let data_read = ctx.data.read().await;

    Ok(data_read
        .get::<DataKey>()
        .expect("Expected Data in TypeMap.")
        .clone())
}

pub async fn get_listen_channel(
    ctx: &Context,
    data: &RwLockWriteGuard<'_, Data>,
) -> Result<ChannelId> {
    let channel_name = get_channel_listing(ctx).await.unwrap();

    for channel in channel_name {
        if channel.name == data.listen_channel {
            return Ok(channel.id);
        }
    }
    panic!("get_listen_channel failed to return channel")
}

async fn get_channel_listing(ctx: &Context) -> Result<Vec<GuildChannel>, Error> {
    let mut channels: Vec<GuildChannel> = Vec::new();

    for guild in ctx.cache.guilds() {
        for channel in guild.channels(&ctx).await? {
            channels.push(channel.1)
        }
    }
    Ok(channels)
}

pub async fn clean_messages(ctx: &Context, channel: &Channel, user: &UserId) {
    let messages = channel
        .id()
        .messages(&ctx, GetMessages::new())
        .await
        .unwrap();

    for message in messages {
        if message.author.id == *user && !message.embeds.is_empty() {
            for embed in &message.embeds {
                if embed.title == Some("The current queue is:".to_owned()) {
                    message.delete(&ctx).await.expect("Error deleting messages");
                }
            }
        }
    }
}

pub async fn update_message(
    http: &Arc<Http>,
    cache: &Arc<Cache>,
    data: &tokio::sync::RwLockWriteGuard<'_, Data>,
    content: EditMessage,
) {
    let user = http.get_current_user().await.unwrap().id;
    let listen_channel = data.listen_channel.clone();

    for guild in cache.guilds() {
        for channel in guild.channels(http).await.unwrap().into_values() {
            if channel.name == listen_channel {
                let messages = channel.messages(http, GetMessages::new()).await.unwrap();
                for mut message in messages {
                    if message.author.id == user && !message.embeds.is_empty() {
                        for embed in message.embeds.clone() {
                            if embed.title == Some("The current queue is:".to_owned()) {
                                message.edit(&http, content.clone()).await.unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn create_message_contents(queue: MutexGuard<'_, VecDeque<Player>>) -> CreateMessage {
    let mut tank_queue_len = 0;
    let mut healer_queue_len = 0;
    let mut dps_queue_len = 0;

    for player in queue.iter() {
        match player.role {
            Roles::Tank => tank_queue_len += 1,
            Roles::Healer => healer_queue_len += 1,
            Roles::Dps => dps_queue_len += 1,
        }
    }
    let embed = CreateEmbed::new()
        .title("The current queue is:")
        .field(
            "<:tank:444634700523241512>",
            tank_queue_len.to_string(),
            true,
        )
        .field(
            "<:heal:444634700363857921>",
            healer_queue_len.to_string(),
            true,
        )
        .field("<:dps:444634700531630094>", dps_queue_len.to_string(), true)
        .color(Colour::FOOYOO);
    let buttons = make_buttons();
    let mut contents = CreateMessage::new().add_embed(embed);

    for button in &buttons {
        contents = contents.button(button.clone())
    }
    contents
}
pub fn create_update_contents(queue: &MutexGuard<'_, VecDeque<Player>>) -> EditMessage {
    let mut tank_queue_len = 0;
    let mut healer_queue_len = 0;
    let mut dps_queue_len = 0;
    for player in queue.iter() {
        match player.role {
            Roles::Tank => tank_queue_len += 1,
            Roles::Healer => healer_queue_len += 1,
            Roles::Dps => dps_queue_len += 1,
        }
    }
    let embed = CreateEmbed::new()
        .title("The current queue is:")
        .field(
            "<:tank:444634700523241512>",
            tank_queue_len.to_string(),
            true,
        )
        .field(
            "<:heal:444634700363857921>",
            healer_queue_len.to_string(),
            true,
        )
        .field("<:dps:444634700531630094>", dps_queue_len.to_string(), true)
        .color(Colour::FOOYOO);
    let buttons = make_buttons();
    let mut contents = EditMessage::new().add_embed(embed);

    for button in &buttons {
        contents = contents.button(button.clone())
    }
    contents
}

pub fn make_buttons() -> Vec<CreateButton> {
    let tank_button = CreateButton::new("add_tank")
        .label("Tank")
        .style(ButtonStyle::Primary)
        .emoji(EmojiId::new(444634700523241512));
    let healer_button = CreateButton::new("add_healer")
        .label("Healer")
        .style(ButtonStyle::Success)
        .emoji(EmojiId::new(444634700363857921));
    let dps_button = CreateButton::new("add_dps")
        .label("DPS")
        .style(ButtonStyle::Danger)
        .emoji(EmojiId::new(444634700531630094));
    let leave_button = CreateButton::new("leave")
        .label("Leave")
        .style(ButtonStyle::Secondary);

    vec![tank_button, healer_button, dps_button, leave_button]
}

pub fn check_user_in_queue(
    queue: &MutexGuard<'_, VecDeque<Player>>,
    user: &User,
    role: Roles,
) -> bool {
    !queue
        .iter()
        .any(|p| p.id.to_string() == user.id.to_string() && p.role == role)
}

pub fn check_group_found(queue: &mut MutexGuard<'_, VecDeque<Player>>) -> Option<String> {
    use itertools::Itertools;
    let queue_possibilities = queue.iter().combinations(5);
    for possibility in queue_possibilities {
        let mut tanks = Vec::new();
        let mut healers = Vec::new();
        let mut dps = Vec::new();

        for player in possibility {
            if !dps.contains(&player.id)
                && !healers.contains(&player.id)
                && !tanks.contains(&player.id)
            {
                match player.role {
                    Roles::Tank => {
                        if !tanks.contains(&player.id) {
                            tanks.push(player.id)
                        }
                    }
                    Roles::Healer => {
                        if !healers.contains(&player.id) {
                            healers.push(player.id)
                        }
                    }
                    Roles::Dps => {
                        if !dps.contains(&player.id) && dps.len() < 3 {
                            dps.push(player.id);
                        }
                    }
                }
            }
            if tanks.len() == 1 && healers.len() == 1 && dps.len() == 3 {
                let group_ids = vec![tanks[0], healers[0], dps[0], dps[1], dps[2]];
                let final_group = add_players_to_game_found(group_ids.clone());
                println!("Found group: {final_group:?}");
                queue.retain(|player| !group_ids.contains(&player.id));
                return Some(final_group);
            }
        }
    }
    None
}

pub async fn get_display_name(ctx: &Context, user: &User, guild: &GuildId) -> String {
    if user.nick_in(&ctx.http, guild).await.is_some() {
        user.nick_in(&ctx.http, guild).await.unwrap()
    } else if user.global_name.is_some() {
        user.global_name.as_ref().unwrap().to_owned()
    } else {
        user.name.clone()
    }
}

pub fn create_player(user: UserId, role: Roles) -> Player {
    use crate::usersettings;
    let settings = get_user_settings(user);
    let duration = match &settings {
        Ok(settings) => settings.timeout,
        Err(_) => {
            set_user_settings(usersettings::Settings {
                id: user,
                timeout: Duration::from_secs(10_800),
                notify: true,
            })
            .unwrap();
            Duration::from_secs(10_800)
        }
    };

    Player {
        id: user,
        role,
        timeout: duration,
        timestamp: SystemTime::now(),
    }
}

fn add_players_to_game_found(queue: Vec<UserId>) -> String {
    let mut current_queue = queue.clone();
    let mut final_queue: String = "Game found! The players are: ".to_owned();
    for _ in 0..5 {
        final_queue.push_str(&format_game_found_output(current_queue.pop().unwrap()))
    }
    final_queue
}

fn format_game_found_output(player: UserId) -> String {
    let mut player_string = String::new();

    player_string.push_str("<@");
    player_string.push_str(&player.to_string());
    player_string.push_str(">, ");
    player_string
}
