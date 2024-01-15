use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "This is a test".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("notify").description("Toggle notifications when timed out of queue.")
}
