use crate::usersettings::*;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(user: &serenity::all::User, _options: &[ResolvedOption]) -> String {
    let settings = get_user_settings(user.id).unwrap();
    let response;
    let settings = Settings {
        id: user.id,
        timeout: settings.timeout,
        notify: {
            match &settings.notify {
                true => {
                    response = "You will no longer be notified when you expire from the queue."
                        .to_string();
                    false
                }
                false => {
                    response =
                        "You will now be notified when you expire from the queue.".to_string();
                    true
                }
            }
        },
    };
    set_user_settings(settings).unwrap();
    response
}

pub fn register() -> CreateCommand {
    CreateCommand::new("notify").description("Toggle notifications when timed out of queue.")
}
