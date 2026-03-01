use crate::error::{AppError, AppResult};
use crate::utils::SIGN_AND_SEND_TRANSACTION_METHOD;
use solana_transaction::Transaction;

#[cfg(feature = "hydrate")]
pub async fn send_transaction(transaction: Transaction) -> AppResult<String> {
    use crate::wallet::{
        is_phantom_installed, phantom_request, PhantomParams, PhantomRequest,
        PhantomSendTransactionResponse,
    };

    if !is_phantom_installed() {
        return Err(AppError::custom("Phantom not installed"));
    }

    let tx_bytes = bincode::serialize(&transaction).map_err(|e| AppError::custom(e.to_string()))?;
    let encoded_tx_bytes = bs58::encode(tx_bytes).into_string();

    let req = PhantomRequest {
        method: SIGN_AND_SEND_TRANSACTION_METHOD,
        params: PhantomParams {
            message: encoded_tx_bytes,
        },
    };
    let js_req = serde_wasm_bindgen::to_value(&req).map_err(|e| e.to_string())?;

    let result = phantom_request(js_req)
        .await
        .map_err(|e| AppError::custom(format!("{:?}", e)))?;

    let resp: PhantomSendTransactionResponse =
        serde_wasm_bindgen::from_value(result).map_err(|e| AppError::custom(e.to_string()))?;

    Ok(resp.signature)
}
