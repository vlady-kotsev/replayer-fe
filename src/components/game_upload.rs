use crate::error::{AppError, AppResult};
use crate::server::{
    build_allocate_game_account_tx, build_finalize_game_upload_tx, build_upload_game_chunk_tx,
};
use leptos::{prelude::*, task::spawn_local};
use thaw::{Button, ButtonAppearance, FileList, Input, Upload, UploadDragger};
use wasm_bindgen_futures::{js_sys::Uint8Array, JsFuture};

const CHUNK_SIZE: usize = 900;

#[component]
pub fn GameUpload() -> impl IntoView {
    let game_name = RwSignal::new(String::new());
    let game_uri = RwSignal::new(String::new());
    let game_price = RwSignal::new(String::new());
    let max_supply = RwSignal::new(String::new());
    let file_bytes: StoredValue<Option<Vec<u8>>> = StoredValue::new(None);
    let file_loaded = RwSignal::new(false);
    let status = RwSignal::new(String::new());
    let uploading = RwSignal::new(false);

    let handle_file = move |file_list: FileList| {
        if let Some(file) = file_list.get(0) {
            let file = file.to_owned();
            spawn_local(async move {
                let array_buffer = JsFuture::from(file.array_buffer()).await.unwrap();
                let bytes = Uint8Array::new(&array_buffer).to_vec();
                status.set(format!("File loaded: {} bytes", bytes.len()));
                file_bytes.set_value(Some(bytes));
                file_loaded.set(true);
            });
        }
    };

    let on_submit = move |_| {
        uploading.set(true);
        status.set("Starting upload...".into());
        #[cfg(feature = "hydrate")]
        spawn_local(async move {
            let result = upload_game_flow(
                game_name.get_untracked(),
                game_uri.get_untracked(),
                game_price.get_untracked(),
                max_supply.get_untracked(),
                file_bytes,
                status,
            )
            .await;

            match result {
                Ok(_) => {
                    status.set("Game published successfully!".into());
                    game_name.set(String::new());
                    game_uri.set(String::new());
                    game_price.set(String::new());
                    max_supply.set(String::new());
                    file_bytes.set_value(None);
                    file_loaded.set(false);
                }
                Err(e) => status.set(format!("Error: {e}")),
            }
            uploading.set(false);
        });
    };

    view! {
        <div class="game-upload">
            <h2>"Publish Game"</h2>
            <Input value=game_name placeholder="Game Name" />
            <Input value=game_uri placeholder="Game URI" />
            <Input value=game_price placeholder="Price (lamports)" />
            <Input value=max_supply placeholder="Max Supply" />

            {move || {
                if file_loaded.get() {
                    view! { <p class="file-status">"File ready for upload"</p> }.into_any()
                } else {
                    view! {
                        <Upload custom_request=handle_file>
                            <UploadDragger>"Click or drag a game file to upload"</UploadDragger>
                        </Upload>
                    }
                        .into_any()
                }
            }}

            <p class="upload-status">{move || status.get()}</p>

            <Button
                appearance=ButtonAppearance::Primary
                on_click=on_submit
                loading=uploading
                disabled=Signal::derive(move || !file_loaded.get() || uploading.get())
            >
                "Publish Game"
            </Button>
        </div>
    }
}

#[cfg(feature = "hydrate")]
async fn sha256(data: &[u8]) -> AppResult<Vec<u8>> {
    let crypto = web_sys::window()
        .ok_or(AppError::custom("No window"))?
        .crypto()
        .map_err(|_| AppError::custom("No crypto API"))?;
    let subtle = crypto.subtle();
    let js_data = Uint8Array::from(data);
    let promise = subtle
        .digest_with_str_and_buffer_source("SHA-256", &js_data)
        .map_err(|_| AppError::custom("SHA-256 digest failed"))?;
    let result = JsFuture::from(promise)
        .await
        .map_err(|_| AppError::custom("SHA-256 await failed"))?;
    Ok(Uint8Array::new(&result).to_vec())
}

#[cfg(feature = "hydrate")]
async fn upload_game_flow(
    game_name: String,
    game_uri: String,
    game_price: String,
    max_supply: String,
    file_bytes: StoredValue<Option<Vec<u8>>>,
    status: RwSignal<String>,
) -> AppResult<()> {
    use crate::wallet::{get_public_key, send_transaction};

    let bytes = file_bytes
        .get_value()
        .ok_or(AppError::custom("No file selected"))?;

    let price: u64 = game_price
        .parse()
        .map_err(|_| AppError::custom("Invalid price"))?;
    let supply: u64 = max_supply
        .parse()
        .map_err(|_| AppError::custom("Invalid max supply"))?;

    let developer = get_public_key().await;
    let game_hash = sha256(&bytes).await?;

    // 1. Allocate game account
    status.set("Allocating game account...".into());
    let tx = build_allocate_game_account_tx(
        developer.clone(),
        game_name.clone(),
        game_uri,
        price,
        game_hash,
        supply,
        bytes.len() as u64,
    )
    .await
    .map_err(|e| AppError::custom(e.to_string()))?;
    send_transaction(tx).await?;

    // 2. Upload chunks
    let total_chunks = bytes.len().div_ceil(CHUNK_SIZE);
    for (i, chunk) in bytes.chunks(CHUNK_SIZE).enumerate() {
        status.set(format!("Uploading chunk {}/{}...", i + 1, total_chunks));
        let tx = build_upload_game_chunk_tx(developer.clone(), game_name.clone(), chunk.to_vec())
            .await
            .map_err(|e| AppError::custom(e.to_string()))?;
        send_transaction(tx).await?;
    }

    // 3. Finalize
    status.set("Finalizing upload...".into());
    let tx = build_finalize_game_upload_tx(developer, game_name)
        .await
        .map_err(|e| AppError::custom(e.to_string()))?;
    send_transaction(tx).await?;

    Ok(())
}
