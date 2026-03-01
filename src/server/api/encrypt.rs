use leptos::prelude::*;

use crate::error::{AppError, AppResult};

#[server]
pub async fn encrypt_game_data(
    encryption_key: String,
    nonce: String,
    game_data: String,
) -> AppResult<String> {
    use crate::models::Encryptor;
    use crate::server::CreateGameResponse;
    use base64::{engine::general_purpose::STANDARD, Engine};

    let encryptor: Encryptor = CreateGameResponse {
        encryption_key,
        nonce,
    }
    .try_into()?;

    let data = STANDARD
        .decode(&game_data)
        .map_err(|e| AppError::custom(format!("Base64 decode error: {e}")))?;

    let encrypted = encryptor.encrypt(&data)?;

    Ok(STANDARD.encode(&encrypted))
}

#[server]
pub async fn decrypt_game_data(
    encryption_key: String,
    nonce: String,
    game_data: String,
) -> AppResult<String> {
    use crate::models::Encryptor;
    use crate::server::CreateGameResponse;
    use base64::{engine::general_purpose::STANDARD, Engine};

    let encryptor: Encryptor = CreateGameResponse {
        encryption_key,
        nonce,
    }
    .try_into()?;

    let data = STANDARD
        .decode(&game_data)
        .map_err(|e| AppError::custom(format!("Base64 decode error: {e}")))?;

    let decrypted = encryptor.decrypt(&data)?;

    Ok(STANDARD.encode(&decrypted))
}
