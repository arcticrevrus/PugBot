use crate::usersettings::*;
use serenity::builder::CreateCommand;
use std::time::Duration;

pub fn run(user: &serenity::all::User) -> Option<String> {
    let settings = get_user_settings(user.id);
    let settings = match settings {
        Ok(_) => settings.unwrap(),
        Err(_) => {
            let settings = Settings {
                id: user.id,
                timeout: Duration::from_secs(10_800),
                notify: true,
            };
            set_user_settings(settings.clone()).unwrap();
            settings
        }
    };
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
    Some(response)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("notify").description("Toggle notifications when timed out of queue.")
}
