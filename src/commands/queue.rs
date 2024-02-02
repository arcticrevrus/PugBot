use crate::get_display_name;
use crate::Player;
use crate::Roles;
use crate::VecDeque;
use serenity::all::{CommandInteraction, CreateEmbed};
use serenity::builder::CreateCommand;
use serenity::builder::CreateInteractionResponse;
use serenity::builder::CreateInteractionResponseMessage;
use serenity::model::Color;
use tokio::sync::MutexGuard;

pub async fn run(
    ctx: &serenity::all::Context,
    command: &CommandInteraction,
    queue: &MutexGuard<'_, VecDeque<Player>>,
) -> Option<String> {
    let mut tank_queue = String::new();
    let mut healer_queue = String::new();
    let mut dps_queue = String::new();
    let guild = &command.guild_id.unwrap();
    for player in queue.iter() {
        match player.role {
            Roles::Tank => {
                tank_queue.push_str(
                    &get_display_name(ctx, &player.id.to_user(&ctx.http).await.unwrap(), guild)
                        .await,
                );
                tank_queue.push_str(", ");
            }
            Roles::Healer => {
                healer_queue.push_str(
                    &get_display_name(ctx, &player.id.to_user(&ctx.http).await.unwrap(), guild)
                        .await,
                );
                healer_queue.push_str(", ");
            }
            Roles::Dps => {
                dps_queue.push_str(
                    &get_display_name(ctx, &player.id.to_user(&ctx.http).await.unwrap(), guild)
                        .await,
                );
                dps_queue.push_str(", ");
            }
        }
    }

    let embed = CreateEmbed::new()
        .title("The following players are in the queue")
        .field(
            "<:tank:444634700523241512>",
            tank_queue.trim_end_matches(", "),
            true,
        )
        .field(
            "<:heal:444634700363857921>",
            healer_queue.trim_end_matches(", "),
            true,
        )
        .field(
            "<:dps:444634700531630094>",
            dps_queue.trim_end_matches(", "),
            true,
        )
        .color(Color::GOLD);
    let message = CreateInteractionResponseMessage::new().add_embed(embed);
    let contents = CreateInteractionResponse::Message(message);
    command.create_response(&ctx.http, contents).await.unwrap();

    None
}

pub fn register() -> CreateCommand {
    CreateCommand::new("queue").description("Print out the current queue.")
}
