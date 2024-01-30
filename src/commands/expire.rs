use crate::usersettings::*;
use serenity::all::{CommandOptionType, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::{builder::CreateCommandOption, model::application::ResolvedOption};
use std::time::Duration;

pub fn run(user: &serenity::all::User, options: &[ResolvedOption]) -> String {
    let initial_settings = get_user_settings(user.id);
    let settings = match initial_settings {
        Ok(_) => initial_settings.unwrap(),
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
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(minutes),
        ..
    }) = options.first()
    {
        let settings = Settings {
            id: user.id,
            timeout: Duration::from_secs(minutes.to_owned().unsigned_abs() * 60),
            notify: settings.notify,
        };
        let response = format!(
            "You will now remain in the queue for {:?} minutes",
            settings.timeout.as_secs() / 60
        );
        set_user_settings(settings).unwrap();
        response
    } else {
        let settings = get_user_settings(user.id).unwrap();
        let response = format!(
            "You're expiration duration is currently set to {} minutes.",
            settings.timeout.as_secs() / 60
        );
        response
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("expire")
        .description("Set time in minutes")
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "minutes", "Number of minutes")
                .min_int_value(30)
                .max_int_value(180)
                .required(false),
        )
}
