use crate::error::{AppError, AppResult};

pub struct Message {
    pub valid_period: i64,
    pub game_name: String,
}

impl Message {
    pub fn to_string_message(&self) -> String {
        format!(
            "Replayer: game '{}' (valid until: {})",
            self.game_name, self.valid_period
        )
    }
}

#[cfg(feature = "hydrate")]
pub async fn sign_message(message: Message) -> AppResult<(String, String)> {
    use crate::wallet::{is_phantom_installed, phantom_sign_message};
    use js_sys::Uint8Array;
    use wasm_bindgen::JsValue;

    if !is_phantom_installed() {
        return Err(AppError::custom("Phantom not installed"));
    }

    let msg_string = message.to_string_message();
    let msg_bytes = msg_string.as_bytes();
    let uint8_array = Uint8Array::new_with_length(msg_bytes.len() as u32);
    uint8_array.copy_from(msg_bytes);

    let result = phantom_sign_message(uint8_array.into())
        .await
        .map_err(|e| AppError::custom(format!("{:?}", e)))?;

    let signature = js_sys::Reflect::get(&result, &JsValue::from_str("signature"))
        .map_err(|e| AppError::custom(format!("{:?}", e)))?;

    let sig_bytes = Uint8Array::new(&signature).to_vec();
    Ok((bs58::encode(sig_bytes).into_string(), msg_string))
}
