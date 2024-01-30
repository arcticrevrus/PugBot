use rusqlite::{Connection, Result};
use serenity::all::*;
use std::time::Duration;

#[derive(Clone)]
pub struct Settings {
    pub id: UserId,
    pub timeout: Duration,
    pub notify: bool,
}

fn create_connection() -> Result<Connection> {
    Connection::open("settings.db")
}

pub fn set_user_settings(settings: Settings) -> Result<()> {
    let conn = create_connection().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            timeout INTEGER,
            notify INTEGER
        )",
        (),
    )?;

    conn.execute(
        "INSERT OR REPLACE INTO users (id, timeout, notify) VALUES (?1, ?2, ?3)",
        (
            &settings.id.get(),
            &settings.timeout.as_secs(),
            &settings.notify,
        ),
    )?;
    Ok(())
}

pub fn get_user_settings(user: UserId) -> Result<Settings> {
    let conn = create_connection().unwrap();
    let querystring = format!("SELECT * FROM users where id = {}", user);
    let mut query = conn.prepare(&querystring)?;
    let result = query.query_row([], |row| {
        Ok(Settings {
            id: UserId::new(row.get(0)?),
            timeout: Duration::from_secs(row.get(1)?),
            notify: row.get(2)?,
        })
    })?;
    Ok(result)
}
