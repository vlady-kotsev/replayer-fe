use leptos::prelude::*;

#[cfg(feature = "ssr")]
fn bundlr_upload(
    bundlr: std::sync::Arc<bundlr_sdk::Bundlr<bundlr_sdk::currency::solana::Solana>>,
    data: Vec<u8>,
    tags: Vec<bundlr_sdk::tags::Tag>,
    gateway_url: String,
) -> Result<String, String> {
    let handle = tokio::runtime::Handle::current();
    handle.block_on(async move {
        let mut tx = bundlr
            .create_transaction(data, tags)
            .map_err(|e| format!("Create tx: {e}"))?;
        bundlr
            .sign_transaction(&mut tx)
            .await
            .map_err(|e| format!("Sign tx: {e}"))?;
        let response = bundlr
            .send_transaction(tx)
            .await
            .map_err(|e| format!("Upload: {e}"))?;

        response["id"]
            .as_str()
            .map(|id| format!("{}tx/{}/data", gateway_url, id))
            .ok_or_else(|| "No id in upload response".to_string())
    })
}

#[server]
pub async fn upload_game_metadata(
    image_b58: String,
    content_type: String,
) -> Result<String, ServerFnError> {
    use crate::config::Config;
    use bundlr_sdk::{currency::solana::Solana, tags::Tag, Bundlr};
    use std::sync::Arc;

    let image_bytes = bs58::decode(&image_b58)
        .into_vec()
        .map_err(|e| ServerFnError::new(format!("Decode image: {e}")))?;

    let bundlr = use_context::<Arc<Bundlr<Solana>>>()
        .ok_or_else(|| ServerFnError::new("Bundlr client not available"))?;
    let config =
        use_context::<Config>().ok_or_else(|| ServerFnError::new("Config not available"))?;
    let gateway_url = config.solana.bundlr_url.clone();

    // Upload image to Bundlr
    let image_tags = vec![Tag::new("Content-Type", &content_type)];
    let bundlr_clone = bundlr.clone();
    let gw = gateway_url.clone();
    let image_uri = tokio::task::spawn_blocking(move || {
        bundlr_upload(bundlr_clone, image_bytes, image_tags, gw)
    })
    .await
    .map_err(|e| ServerFnError::new(format!("Task error: {e}")))?
    .map_err(ServerFnError::new)?;

    Ok(image_uri)
}
