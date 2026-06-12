use std::env;

pub struct Config {
    pub telegram_bot_token: String,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Self {
        Self {
            telegram_bot_token:
            env::var("TELEGRAM_BOT_TOKEN")
                .expect("TELEGRAM_BOT_TOKEN is missing"),
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL is missing"),

        }
    }
}