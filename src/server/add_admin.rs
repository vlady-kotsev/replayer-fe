use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_add_admin_tx(admin: String, new_admin: String) -> AppResult<Transaction> {
    use crate::generated::instructions::AddAdminBuilder;
    use crate::{constants::ADMIN_SEED, server::get_latest_blockhash};

    use solana_pubkey::Pubkey;

    let admin_pubkey = Pubkey::from_str_const(&admin);
    let new_admin_pubkey = Pubkey::from_str_const(&new_admin);

    let (admin_account_pda, _) =
        Pubkey::find_program_address(&[ADMIN_SEED, admin_pubkey.as_ref()], &crate::REPLAYER_ID);
    let (new_admin_pda, _) = Pubkey::find_program_address(
        &[ADMIN_SEED, new_admin_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );

    let ix = AddAdminBuilder::new()
        .admin(admin_pubkey)
        .admin_account(admin_account_pda)
        .new_admin(new_admin_pda)
        .new_admin_arg(new_admin_pubkey)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&admin_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
