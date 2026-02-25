use crate::wallet::phantom_public_key_string;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct PhantomParams {
    pub message: String,
}

#[derive(Serialize)]
pub struct PhantomRequest {
    pub method: &'static str,
    pub params: PhantomParams,
}

#[derive(Deserialize)]
pub struct PhantomSendTransactionResponse {
    pub signature: String,
}

#[cfg(feature = "hydrate")]
pub fn is_phantom_installed() -> bool {
    let window = web_sys::window().unwrap();
    let solana = js_sys::Reflect::get(&window, &JsValue::from_str("solana"));
    solana
        .map(|v| !v.is_undefined() && !v.is_null())
        .unwrap_or(false)
}

#[cfg(feature = "hydrate")]
pub async fn get_public_key() -> String {
    phantom_public_key_string()
}
