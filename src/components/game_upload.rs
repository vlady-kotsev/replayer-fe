use crate::error::{AppError, AppResult};
use crate::server::{
    build_allocate_game_account_tx, build_finalize_game_upload_tx, build_upload_game_chunk_tx,
    encrypt_game_data, upload_game_metadata,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use leptos::{prelude::*, task::spawn_local};
use thaw::{
    Button, ButtonAppearance, FileList, Input, Toast, ToastBody, ToastIntent, ToastOptions,
    ToastTitle, ToasterInjection, Upload, UploadDragger,
};
use wasm_bindgen_futures::{js_sys::Uint8Array, JsFuture};
const CHUNK_SIZE: usize = 900;

#[component]
pub fn GameUpload() -> impl IntoView {
    let game_name = RwSignal::new(String::new());
    let game_price = RwSignal::new(String::new());
    let max_supply = RwSignal::new(String::new());
    let file_bytes: StoredValue<Option<Vec<u8>>> = StoredValue::new(None);
    let file_loaded = RwSignal::new(false);
    let image_bytes: StoredValue<Option<Vec<u8>>> = StoredValue::new(None);
    let image_content_type: StoredValue<Option<String>> = StoredValue::new(None);
    let image_loaded = RwSignal::new(false);
    let status = RwSignal::new(String::new());
    let uploading = RwSignal::new(false);
    let toaster = ToasterInjection::expect_context();

    let handle_file = move |file_list: FileList| {
        if let Some(file) = file_list.get(0) {
            let file = file.to_owned();
            spawn_local(async move {
                let Ok(array_buffer) = JsFuture::from(file.array_buffer()).await else {
                    leptos::logging::log!("Failed to read game file");
                    status.set("Failed to read game file.".into());
                    return;
                };
                let bytes = Uint8Array::new(&array_buffer).to_vec();
                status.set(format!("Game file loaded: {} bytes", bytes.len()));
                file_bytes.set_value(Some(bytes));
                file_loaded.set(true);
            });
        }
    };

    let handle_image = move |file_list: FileList| {
        if let Some(file) = file_list.get(0) {
            let file = file.to_owned();
            spawn_local(async move {
                let content_type = file.type_();
                let Ok(array_buffer) = JsFuture::from(file.array_buffer()).await else {
                    leptos::logging::log!("Failed to read image file");
                    status.set("Failed to read image file.".into());
                    return;
                };
                let bytes = Uint8Array::new(&array_buffer).to_vec();
                status.set(format!("Image loaded: {} bytes", bytes.len()));
                image_bytes.set_value(Some(bytes));
                image_content_type.set_value(Some(if content_type.is_empty() {
                    "image/png".to_string()
                } else {
                    content_type
                }));
                image_loaded.set(true);
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
                game_price.get_untracked(),
                max_supply.get_untracked(),
                file_bytes,
                image_bytes,
                image_content_type,
                status,
            )
            .await;

            match result {
                Ok(_) => {
                    status.set("Game published successfully!".into());
                    toaster.dispatch_toast(
                        move || {
                            view! {
                                <Toast>
                                    <ToastTitle>"Game Published"</ToastTitle>
                                    <ToastBody>"Your game will be available for purchase shortly."</ToastBody>
                                </Toast>
                            }
                        },
                        ToastOptions::default().with_intent(ToastIntent::Success),
                    );
                    game_name.set(String::new());
                    game_price.set(String::new());
                    max_supply.set(String::new());
                    file_bytes.set_value(None);
                    file_loaded.set(false);
                    image_bytes.set_value(None);
                    image_content_type.set_value(None);
                    image_loaded.set(false);
                }
                Err(e) => {
                    leptos::logging::log!("Upload error: {e}");
                    status.set("Something went wrong. Please try again.".into());
                }
            }
            uploading.set(false);
        });
    };

    view! {
        <div class="game-upload">
            <h2>"Game data"</h2>
            <Input value=game_name placeholder="Game Name" />
            <Input value=game_price placeholder="Price (lamports)" />
            <Input value=max_supply placeholder="Max Supply" />

            {move || {
                if image_loaded.get() {
                    view! { <p class="file-status">"Image ready for upload"</p> }.into_any()
                } else {
                    view! {
                        <Upload custom_request=handle_image>
                            <UploadDragger>"Click or drag a game cover image"</UploadDragger>
                        </Upload>
                    }
                        .into_any()
                }
            }}

            {move || {
                if file_loaded.get() {
                    view! { <p class="file-status">"Game file ready for upload"</p> }.into_any()
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
                disabled=Signal::derive(move || {
                    !file_loaded.get() || !image_loaded.get() || uploading.get()
                })
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
    game_price: String,
    max_supply: String,
    file_bytes: StoredValue<Option<Vec<u8>>>,
    image_bytes: StoredValue<Option<Vec<u8>>>,
    image_content_type: StoredValue<Option<String>>,
    status: RwSignal<String>,
) -> AppResult<()> {
    use crate::server::create_game;
    use crate::wallet::{get_public_key, send_transaction, sign_message, Message};

    let bytes = file_bytes
        .get_value()
        .ok_or(AppError::custom("No game file selected"))?;

    let price: u64 = game_price
        .parse()
        .map_err(|_| AppError::custom("Invalid price"))?;
    let supply: u64 = max_supply
        .parse()
        .map_err(|_| AppError::custom("Invalid max supply"))?;

    let developer = get_public_key().await;

    // 0. Register game with backend
    status.set("Registering game...".into());
    let valid_period = (js_sys::Date::now() / 1000.0) as i64 + 7200; // now + 2 hours
    let (signature, _msg) = sign_message(Message {
        valid_period,
        game_name: game_name.clone(),
    })
    .await?;
    let create_game_reponse = create_game(
        game_name.clone(),
        developer.clone(),
        signature,
        valid_period,
    )
    .await
    .map_err(|e| AppError::custom(e.to_string()))?;

    // 1. Encrypt game bytes on server
    status.set("Encrypting game data...".into());
    let encrypted_b64 = encrypt_game_data(
        create_game_reponse.encryption_key,
        create_game_reponse.nonce,
        STANDARD.encode(&bytes),
    )
    .await
    .map_err(|e| AppError::custom(e.to_string()))?;
    let bytes = STANDARD
        .decode(&encrypted_b64)
        .map_err(|e| AppError::custom(format!("Base64 decode error: {e}")))?;

    // 2. Upload image and create metadata URI
    status.set("Uploading game image...".into());
    let img_bytes = image_bytes
        .get_value()
        .ok_or(AppError::custom("No image selected"))?;
    let content_type = image_content_type
        .get_value()
        .unwrap_or_else(|| "image/png".to_string());
    let image_b58 = bs58::encode(&img_bytes).into_string();
    let game_uri = upload_game_metadata(image_b58, content_type)
        .await
        .map_err(|e| AppError::custom(e.to_string()))?;

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
    let tx = build_finalize_game_upload_tx(developer.clone(), game_name.clone())
        .await
        .map_err(|e| AppError::custom(e.to_string()))?;
    send_transaction(tx).await?;

    Ok(())
}
