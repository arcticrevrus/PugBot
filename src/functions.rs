use std::sync::Arc;
use serenity::{prelude::*, all::{*}};

pub struct Data {
    pub first_launch: bool,
    pub tank_queue: Arc<Mutex<Vec<Player>>>,
    pub healer_queue: Arc<Mutex<Vec<Player>>>,
    pub dps_queue: Arc<Mutex<Vec<Player>>>,
    pub listen_channel: String
}

#[derive(PartialEq)]
pub struct Player {
    pub name: User,
    pub role: Roles,
}

#[derive(PartialEq, Clone)]
pub enum Roles {
    Tank,
    Healer,
    DPS,
}

impl TypeMapKey for DataKey {
    type Value = Arc<RwLock<Data>>;
}

pub struct DataKey;

pub struct Handler;

pub fn check_first_launch(mut data: tokio::sync::RwLockWriteGuard<'_, Data, >) -> bool {
    let first_launch = data.first_launch;

    if first_launch {
        data.first_launch = false;
    }
    first_launch
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

pub async fn get_listen_channel(ctx: &Context) -> ChannelId {
    let data = initialize_data(&ctx).await;
    let data = data.write().await;
    for channel in get_channel_listing(&ctx).await {
        if channel.1.name == data.listen_channel {
            return channel.1.id;
        }
    }
    panic!("get_listen_channel failed to return channel")
}

pub async fn clean_messages(ctx: &Context, channel: &Channel, user: &UserId) {
    for message in channel.id().messages(&ctx, GetMessages::new()).await.expect("Error getting channel messages") {
        if &message.author.id == user {
            if message.embeds.is_empty() != true {
                message.delete(&ctx).await.expect("Error deleting messages");
            }
        }
    }
}

pub async fn create_message_contents(ctx: &Context) -> CreateMessage {
    let data = initialize_data(&ctx).await;
    let data = data.write().await;
    let tank_queue_len = data.tank_queue.lock().await.len().to_string();
    let healer_queue_len = data.healer_queue.lock().await.len().to_string();
    let dps_queue_len = data.dps_queue.lock().await.len().to_string();
    let embed = CreateEmbed::new()
        .title("The current queue is:")
        .field("<:tank:444634700523241512>", tank_queue_len, true)
        .field("<:heal:444634700363857921>", healer_queue_len, true)
        .field("<:dps:444634700531630094>", dps_queue_len, true)
        .color(Colour::FOOYOO);
    let buttons = make_buttons();
    let mut contents = CreateMessage::new().add_embed(embed);

    for button in &buttons {
        contents = contents.button(button.clone())
    }
    return contents
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

pub async fn add_user_to_queue(ctx: &Context, user: &User, channel: &Channel, role: Roles) {
    let data = initialize_data(&ctx).await;
    let data = data.write().await;
    let mut tank_queue = data.tank_queue.lock().await;
    let mut healer_queue = data.healer_queue.lock().await;
    let mut dps_queue = data.dps_queue.lock().await;
    let player = create_player(&user, &role);
    match role {
        Roles::Tank => {
            if tank_queue.contains(&player) != true && healer_queue.contains(&player) != true && dps_queue.contains(&player) != true {
                tank_queue.push(player);
                channel.id().say(&ctx.http, format!("{} has added to tank queue.", user.global_name.as_ref().expect("user does not have global name"))).await.expect("Error sending message");
            } else {
                channel.id().say(&ctx.http, format!("Error: {} already in queue.", user.global_name.as_ref().unwrap())).await.unwrap();            }
        },
        Roles::Healer => {
            if tank_queue.contains(&player) != true && healer_queue.contains(&player) != true && dps_queue.contains(&player) != true {
                healer_queue.push(player);
                channel.id().say(&ctx.http, format!("{} has added to healer queue.", user.global_name.as_ref().unwrap())).await.expect("Error sending message");
            } else {
                channel.id().say(&ctx.http, format!("Error: {} already in queue.", user.global_name.as_ref().unwrap())).await.unwrap();
            }
        },
        Roles::DPS => {
            if tank_queue.contains(&player) != true && healer_queue.contains(&player) != true && dps_queue.contains(&player) != true {
                dps_queue.push(player);
                channel.id().say(&ctx.http, format!("{} has added to tank queue.", user.global_name.as_ref().expect("user does not have global name"))).await.expect("Error sending message");
            } else {
                channel.id().say(&ctx.http, format!("Error: {} already in queue.", user.global_name.as_ref().unwrap())).await.unwrap();            }
        }
    }
    if tank_queue.len() >= 1 && healer_queue.len() >= 1 && dps_queue.len() >= 3 {
        let mut game_found: String = "Game found! The players are: ".to_owned();
        game_found.push_str(&add_players_to_game_found(tank_queue, healer_queue, dps_queue));
        channel.id().say(&ctx, game_found.trim_end_matches(", ")).await.expect("Failed to send message");
    }
}

pub async fn remove_from_queue(ctx: &Context, user: &User) {
    let data = initialize_data(&ctx).await;
    let data = data.write().await;
    let mut tank_queue = data.tank_queue.lock().await;
    let mut healer_queue = data.healer_queue.lock().await;
    let mut dps_queue = data.dps_queue.lock().await;
    tank_queue.retain(|p| p.name.id != user.id);
    healer_queue.retain(|p| p.name.id != user.id);
    dps_queue.retain(|p| p.name.id != user.id)
}

fn create_player(user: &User, role: &Roles) -> Player {
    let player = Player {
        name: user.clone(),
        role: role.clone()
    };

    player
}

fn add_players_to_game_found(
    tank_queue: tokio::sync::MutexGuard<'_, Vec<Player>>,
    healer_queue: tokio::sync::MutexGuard<'_, Vec<Player>>,
    dps_queue: tokio::sync::MutexGuard<'_, Vec<Player>>
 ) -> String {
    let mut final_queue = String::new();
    final_queue.push_str(&add_tank_to_game_found(tank_queue));
    final_queue.push_str(&add_healer_to_game_found(healer_queue));
    final_queue.push_str(&add_dps_to_game_found(dps_queue));
    return final_queue
}

fn add_tank_to_game_found(mut tank_queue: tokio::sync::MutexGuard<'_, Vec<Player>>) -> String {
    let Some(tank) = &tank_queue.pop() else { return "Error adding tank to queue".to_owned() };
    let mut tank_string = String::new();
    tank_string.push_str(&format_game_found_output(tank));
    return tank_string
}

fn add_healer_to_game_found(mut healer_queue: tokio::sync::MutexGuard<'_, Vec<Player>>) -> String {
    let Some(healer) = &healer_queue.pop() else { return "Error adding healer to queue".to_owned() };
    let mut healer_string = String::new();
    healer_string.push_str(&format_game_found_output(healer));
    return healer_string
}

fn add_dps_to_game_found(mut dps_queue: tokio::sync::MutexGuard<'_, Vec<Player>>) -> String {
    let mut dps_string = String::new();
    for _ in 0 .. 3 {
        let Some(dps) = &dps_queue.pop() else { return "Error adding healer to queue".to_owned() };
            dps_string.push_str(&format_game_found_output(dps))
    }
    return dps_string
}

fn format_game_found_output(player: &Player) -> String {
    let mut player_string = String::new();
    player_string.push_str("<@");
    player_string.push_str(&player.name.id.to_string());
    player_string.push_str(">, ");
    return player_string
}