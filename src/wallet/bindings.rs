#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "solana"], js_name = connect)]
    pub async fn phantom_connect() -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "solana"], js_name = disconnect)]
    pub async fn phantom_disconnect();

    #[wasm_bindgen(js_namespace = ["window", "solana", "publicKey"], js_name = toString)]
    pub fn phantom_public_key_string() -> String;

    #[wasm_bindgen(js_namespace = ["window", "solana"], js_name = request, catch)]
    pub async fn phantom_request(args: JsValue) -> Result<JsValue, JsValue>;
}
