use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_remove_admin_tx(admin: String, removed_admin: String) -> AppResult<Transaction> {
    use crate::generated::instructions::RemoveAdminBuilder;
    use crate::{server::get_latest_blockhash, utils::ADMIN_SEED};
    use solana_pubkey::Pubkey;

    let admin_pubkey = Pubkey::from_str_const(&admin);
    let removed_admin_pubkey = Pubkey::from_str_const(&removed_admin);

    let (admin_account_pda, _) =
        Pubkey::find_program_address(&[ADMIN_SEED, admin_pubkey.as_ref()], &crate::REPLAYER_ID);
    let (removed_admin_pda, _) = Pubkey::find_program_address(
        &[ADMIN_SEED, removed_admin_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );

    let ix = RemoveAdminBuilder::new()
        .admin(admin_pubkey)
        .admin_account(admin_account_pda)
        .removed_admin(removed_admin_pda)
        .removed_admin_arg(removed_admin_pubkey)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&admin_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
