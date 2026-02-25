use crate::error::{AppError, AppResult};

#[cfg(feature = "hydrate")]
pub async fn connect_phantom() -> AppResult<String> {
    use crate::wallet::{is_phantom_installed, phantom_connect, phantom_public_key_string};

    if !is_phantom_installed() {
        return Err(AppError::custom("Phantom not installed"));
    }

    phantom_connect().await;
    Ok(phantom_public_key_string())
}

#[cfg(feature = "hydrate")]
pub async fn disconnect_phantom() -> AppResult<()> {
    use crate::wallet::{is_phantom_installed, phantom_disconnect};

    if !is_phantom_installed() {
        return Err(AppError::custom("Phantom not installed"));
    }

    phantom_disconnect().await;
    Ok(())
}
