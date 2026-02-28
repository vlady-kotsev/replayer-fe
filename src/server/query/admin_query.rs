use crate::error::AppResult;
use leptos::prelude::*;

#[server]
pub async fn is_admin(admin: String) -> AppResult<bool> {
    use std::sync::Arc;

    use crate::accounts::fetch_admin;
    use crate::constants::ADMIN_SEED;
    use crate::error::AppError;
    use solana_client::rpc_client::RpcClient;
    use solana_pubkey::Pubkey;

    let solana_client =
        use_context::<Arc<RpcClient>>().ok_or(AppError::custom("Can't get context"))?;

    let admin_pubkey = Pubkey::from_str_const(&admin);
    let (admin_account_pda, _) =
        Pubkey::find_program_address(&[ADMIN_SEED, admin_pubkey.as_ref()], &crate::REPLAYER_ID);
    let res = match fetch_admin(&solana_client, &admin_account_pda){
        Ok(res) => res,
        Err(e) =>return Err(AppError::custom("Can't get context"))
    };
    leptos::logging::log!("{:?}", res);
    Ok(false)
}
