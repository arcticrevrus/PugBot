use serenity::{all::*, prelude::*};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

pub struct Data {
    pub first_launch: bool,
    pub queue: Arc<Mutex<VecDeque<Player>>>,
    pub listen_channel: String,
}

#[derive(PartialEq, Clone)]
pub struct Player {
    pub id: UserId,
    pub role: Roles,
    pub timeout: Duration,
    pub timestamp: SystemTime,
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
    Ok(())
}

pub fn check_first_launch(
    mut data: tokio::sync::RwLockWriteGuard<'_, Data>,
) -> Result<bool, Error> {
    let first_launch = data.first_launch;

    if first_launch {
        data.first_launch = false;
    }
    Ok(first_launch)
}

pub async fn initialize_data(ctx: &Context) -> Result<Arc<RwLock<Data>>, Error> {
    let data_read = ctx.data.read().await;

    Ok(data_read
        .get::<DataKey>()
        .expect("Expected Data in TypeMap.")
        .clone())
}

pub async fn get_listen_channel(ctx: &Context) -> Result<ChannelId> {
    let data = initialize_data(ctx).await.unwrap();
    let data = data.write().await;
    let channel_name = get_channel_listing(ctx).await.unwrap();
    for channel in channel_name {
        if channel.name == data.listen_channel {
            return Ok(channel.id);
        }
    }
    panic!("get_listen_channel failed to return channel")
}

pub async fn get_channel_listing(ctx: &Context) -> Result<Vec<GuildChannel>, Error> {
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
            message.delete(&ctx).await.expect("Error deleting messages");
        }
    }
}

pub async fn create_message_contents(ctx: &Context) -> CreateMessage {
    let data = initialize_data(ctx).await.unwrap();
    let data = data.write().await;
    let queue = data.queue.lock().await;
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

pub fn make_buttons() -> Vec<CreateButton> {
    let tank_button = CreateButton::new("add_tank")
        .label("Tank")
        .style(ButtonStyle::Primary);
    let healer_button = CreateButton::new("add_healer")
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

pub async fn check_user_in_queue(
    ctx: &Context,
    button: &ComponentInteraction,
    role: Roles,
) -> bool {
    let data = initialize_data(ctx).await.unwrap();
    let data = data.write().await;
    let queue = data.queue.lock().await;
    let user = &button.user;

    if !queue
        .iter()
        .any(|p| p.id.to_string() == user.id.to_string() && p.role == role)
    {
        true
    } else {
        let message = serenity::all::CreateInteractionResponseMessage::new()
            .content("You are already in the queue.")
            .flags(InteractionResponseFlags::EPHEMERAL);
        let response = CreateInteractionResponse::Message(message);
        button.create_response(&ctx.http, response).await.unwrap();
        false
    }
}

pub async fn add_user_to_queue(ctx: &Context, button: &ComponentInteraction, role: Roles) -> bool {
    let data = initialize_data(ctx).await.unwrap();
    let data = data.write().await;
    let mut queue = data.queue.lock().await;
    let user = &button.user;
    let channel = &button.channel_id.to_channel(&ctx.http).await.unwrap();
    let player = create_player(user.id, &role);
    let guild = &button.guild_id.unwrap();
    let player_display_name = get_display_name(ctx, user, guild).await;
    queue.push_back(player);
    channel
        .id()
        .say(
            &ctx.http,
            format!("{} has added to {:?} queue.", player_display_name, role),
        )
        .await
        .expect("Error sending message");
    button
        .defer(&ctx.http)
        .await
        .expect("Error deferring interaction");
    queue_check(ctx, channel, queue).await;
    true
}

pub async fn queue_check(
    ctx: &Context,
    channel: &Channel,
    mut queue: tokio::sync::MutexGuard<'_, VecDeque<Player>>,
) {
    let mut final_queue = Vec::new();
    let mut tank_check = VecDeque::new();
    let mut healer_check = VecDeque::new();
    let mut dps_check = VecDeque::new();
    let mut game_found: String = "Game found! The players are: ".to_owned();

    if queue.len() >= 5 {
        for player in queue.iter() {
            match player.role {
                Roles::Tank => tank_check.push_back(player.clone()),
                Roles::Healer => healer_check.push_back(player.clone()),
                Roles::Dps => dps_check.push_back(player.clone()),
            }
        }
        if !tank_check.is_empty() && !healer_check.is_empty() && dps_check.len() >= 3 {
            final_queue.push(tank_check.pop_front().unwrap());
            final_queue.push(healer_check.pop_front().unwrap());
            for _ in 1..=3 {
                final_queue.push(dps_check.pop_front().unwrap())
            }
            *queue = queue
                .iter()
                .filter(|p| !final_queue.contains(p))
                .cloned()
                .collect();
            game_found.push_str(&add_players_to_game_found(final_queue));
            channel
                .id()
                .say(&ctx, game_found.trim_end_matches(", "))
                .await
                .expect("Failed to send message");
        }
    }
}

pub async fn remove_from_queue(ctx: &Context, button: &ComponentInteraction) {
    let data = initialize_data(ctx).await.unwrap();
    let data = data.write().await;
    let mut queue = data.queue.lock().await;
    let user = &button.user;
    let channel = &button.channel_id;
    let guild = &button.guild_id.unwrap();
    let player_display_name = get_display_name(ctx, user, guild).await;

    queue.retain(|p| p.id != user.id);
    button.defer(&ctx.http).await.unwrap();
    channel
        .say(
            &ctx.http,
            format!("{} has left the queue.", player_display_name),
        )
        .await
        .expect("Error sending message");
}

async fn get_display_name(ctx: &Context, user: &User, guild: &GuildId) -> String {
    if user.nick_in(&ctx.http, guild).await.is_some() {
        user.nick_in(&ctx.http, guild).await.unwrap()
    } else if user.global_name.is_some() {
        user.global_name.as_ref().unwrap().to_owned()
    } else {
        user.name.clone()
    }
}

fn create_player(user: UserId, role: &Roles) -> Player {
    Player {
        id: user,
        role: role.clone(),
        timeout: Duration::new(10_800, 0),
        timestamp: SystemTime::now(),
    }
}

fn add_players_to_game_found(queue: Vec<Player>) -> String {
    let mut final_queue = String::new();
    let mut current_queue = queue.clone();

    for _ in 0..5 {
        final_queue.push_str(&format_game_found_output(current_queue.pop().unwrap()))
    }
    final_queue
}

fn format_game_found_output(player: Player) -> String {
    let mut player_string = String::new();

    player_string.push_str("<@");
    player_string.push_str(&player.id.to_string());
    player_string.push_str(">, ");
    player_string
}
