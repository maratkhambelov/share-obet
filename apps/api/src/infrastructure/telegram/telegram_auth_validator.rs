use std::collections::HashMap;

use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

pub struct TelegramAuthValidator {
    bot_token: String,
}

#[derive(Deserialize)]
struct TelegramUser {
    id: i64,

    first_name: String,
}

pub struct TelegramAuthData {
    pub telegram_id: String,

    pub display_name: String,
}

fn calculate_hash(
    bot_token: &str,
    data_check_string: &str,
) -> Result<String, String> {
    let mut secret_mac =
        HmacSha256::new_from_slice(
            b"WebAppData",
        )
            .map_err(|e| e.to_string())?;

    secret_mac.update(
        bot_token.as_bytes(),
    );

    let secret_key =
        secret_mac.finalize()
            .into_bytes();

    let mut mac =
        HmacSha256::new_from_slice(
            &secret_key,
        )
            .map_err(|e| e.to_string())?;

    mac.update(
        data_check_string.as_bytes(),
    );

    Ok(
        hex::encode(
            mac.finalize()
                .into_bytes(),
        ),
    )
}

impl TelegramAuthValidator {
    pub fn new(
        bot_token: String,
    ) -> Self {
        Self { bot_token }
    }

    pub fn validate(
        &self,
        init_data: &str,
    ) -> Result<TelegramAuthData, String> {
        let params: HashMap<String, String> =
            url::form_urlencoded::parse(
                init_data.as_bytes(),
            )
                .into_owned()
                .collect();

        let received_hash = params
            .get("hash")
            .ok_or("hash is missing")?;

        let mut data_check_items =
            params
                .iter()
                .filter(|(key, _)| *key != "hash")
                .map(|(key, value)| {
                    format!("{key}={value}")
                })
                .collect::<Vec<_>>();

        data_check_items.sort();

        let data_check_string =
            data_check_items.join("\n");

        let calculated_hash =
            calculate_hash(
                &self.bot_token,
                &data_check_string,
            )?;

        println!(
            "calculated_hash = {}",
            calculate_hash(
                &self.bot_token,
                &data_check_string,
            )?,
        );
        if calculated_hash != *received_hash {
            return Err(
                "invalid telegram signature"
                    .to_string(),
            );
        }

        let user_json =
            params
                .get("user")
                .ok_or("user is missing")?;

        let user: TelegramUser =
            serde_json::from_str(
                user_json,
            )
                .map_err(|e| e.to_string())?;

        Ok(
            TelegramAuthData {
                telegram_id:
                user.id.to_string(),

                display_name:
                user.first_name,
            },
        )
    }
}