use std::sync::Arc;
use serenity::{prelude::*, all::{*}};
use tokio::sync::RwLockWriteGuard;

pub struct Data {
    pub first_launch: bool,
    pub tank_queue: Arc<Mutex<Vec<Player>>>,
    pub healer_queue: Arc<Mutex<Vec<Player>>>,
    pub dps_queue: Arc<Mutex<Vec<Player>>>,
    pub listen_channel: String
}

pub struct Player {
    pub name: User,
    pub role: Roles,
}

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

pub async fn add_user_to_queue(ctx: &Context, user: &User, role: String) {
    let data = initialize_data(&ctx).await;
    let data = data.write().await;

    match role.as_str() {
        "tank" => {
            let queue = &data.tank_queue;
            let mut locked_queue = queue.lock().await;
            let player = create_player(&user, "tank".to_string());
            locked_queue.push(player);
        },
        "healer" => {
            let queue = &data.healer_queue;
            let mut locked_queue = queue.lock().await;
            let player = create_player(&user, "healer".to_string());
            locked_queue.push(player);
        },
        "dps" => {
            let queue = &data.dps_queue;
            let mut locked_queue = queue.lock().await;
            let player = create_player(&user, "dps".to_string());
            locked_queue.push(player);
        }
        _ => ()
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

fn create_player(user: &User, role: String) -> Player {
    let player = Player {
        name: user.clone(),
        role: match role.to_string() {
            tank => Roles::Tank,
            healer => Roles::Healer,
            dps => Roles::DPS
        }
    };

    player
}