use std::env;

pub struct Config {
    pub telegram_bot_token: String,
}

impl Config {
    pub fn load() -> Self {
        Self {
            telegram_bot_token:
            env::var("TELEGRAM_BOT_TOKEN")
                .expect("TELEGRAM_BOT_TOKEN is missing"),
        }
    }
}