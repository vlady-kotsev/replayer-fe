#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
extern "C" {
    pub type PhantomWallet;

    #[wasm_bindgen(js_namespace = ["window", "solana"], js_name = connect)]
    async fn phantom_connect() -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "solana"], js_name = disconnect)]
    async fn phantom_disconnect();

    #[wasm_bindgen(js_namespace = ["window", "solana", "publicKey"], js_name = toString)]
    fn phantom_public_key_string() -> String;

    #[wasm_bindgen(js_namespace = ["window", "solana"], js_name = isPhantom)]
    static IS_PHANTOM: JsValue;
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
pub async fn connect_phantom() -> Result<String, String> {
    if !is_phantom_installed() {
        return Err("Phantom not installed".into());
    }

    phantom_connect().await;
    Ok(phantom_public_key_string())
}

#[cfg(feature = "hydrate")]
pub async fn disconnect_phantom() -> Result<(), String> {
    if !is_phantom_installed() {
        return Err("Phantom not installed".into());
    }

    phantom_disconnect().await;
    Ok(())
}
