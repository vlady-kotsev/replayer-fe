use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_blacklist_account_tx(
    admin: String,
    address: String,
    is_blacklisted: bool,
) -> AppResult<Transaction> {
    use crate::generated::instructions::BlacklistAccountBuilder;
    use crate::{
        constants::{ADMIN_SEED, BLACKLISTED_SEED},
        server::get_latest_blockhash,
    };
    use solana_pubkey::Pubkey;

    let admin_pubkey = Pubkey::from_str_const(&admin);
    let address_pubkey = Pubkey::from_str_const(&address);

    let (admin_account_pda, _) = Pubkey::find_program_address(
        &[ADMIN_SEED, admin_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );
    let (blacklist_account_pda, _) = Pubkey::find_program_address(
        &[BLACKLISTED_SEED, address_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );

    let ix = BlacklistAccountBuilder::new()
        .admin(admin_pubkey)
        .admin_account(admin_account_pda)
        .blacklist_account(blacklist_account_pda)
        .address(address_pubkey)
        .is_blacklisted(is_blacklisted)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&admin_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
