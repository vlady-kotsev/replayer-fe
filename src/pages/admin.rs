use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Input};

use crate::{
    app::WalletPublicKeyContext,
    server::{
        build_add_admin_tx, build_blacklist_account_tx, build_remove_admin_tx,
        build_withdraw_platform_fee_tx,
    },
};

#[component]
pub fn AdminDashboard() -> impl IntoView {
    view! {
        <div class="admin-dashboard">
            <h1>"Admin Dashboard"</h1>
            <div class="admin-actions">
                <AddAdminForm />
                <RemoveAdminForm />
                <BlacklistAccountForm />
                <UnblacklistAccountForm />
                <WithdrawPlatformFeeForm />
            </div>
        </div>
    }
}

#[component]
fn AddAdminForm() -> impl IntoView {
    let public_key = use_context::<WalletPublicKeyContext>()
        .expect("Can't get wallet context")
        .public_key;
    let new_admin = RwSignal::new(String::new());

    let action = Action::new_unsync(move |_| async move {
        #[cfg(feature = "hydrate")]
        {
            let key = public_key
                .get_untracked()
                .ok_or(crate::error::AppError::custom("No wallet connected"))?;
            let tx = build_add_admin_tx(key, new_admin.get_untracked())
                .await
                .map_err(|e| crate::error::AppError::custom(e.to_string()))?;
            let sig = crate::wallet::send_transaction(tx).await?;
            new_admin.set(String::new());
            Ok::<String, crate::error::AppError>(sig)
        }
        #[cfg(not(feature = "hydrate"))]
        Ok::<String, crate::error::AppError>(String::new())
    });

    view! {
        <div class="admin-action-card">
            <h2>"Add Admin"</h2>
            <Input value=new_admin placeholder="New admin public key" />
            <Button
                appearance=ButtonAppearance::Primary
                on_click=move |_| {
                    action.dispatch(());
                }
                loading=action.pending()
                disabled=Signal::derive(move || {
                    new_admin.get().is_empty() || action.pending().get()
                })
            >
                "Add Admin"
            </Button>
            <ActionStatus action=action />
        </div>
    }
}

#[component]
fn RemoveAdminForm() -> impl IntoView {
    let public_key = use_context::<WalletPublicKeyContext>()
        .expect("Can't get wallet context")
        .public_key;
    let removed_admin = RwSignal::new(String::new());

    let action = Action::new_unsync(move |_| async move {
        #[cfg(feature = "hydrate")]
        {
            let key = public_key
                .get_untracked()
                .ok_or(crate::error::AppError::custom("No wallet connected"))?;
            let tx = build_remove_admin_tx(key, removed_admin.get_untracked())
                .await
                .map_err(|e| crate::error::AppError::custom(e.to_string()))?;
            let sig = crate::wallet::send_transaction(tx).await?;
            removed_admin.set(String::new());
            Ok::<String, crate::error::AppError>(sig)
        }
        #[cfg(not(feature = "hydrate"))]
        Ok::<String, crate::error::AppError>(String::new())
    });

    view! {
        <div class="admin-action-card">
            <h2>"Remove Admin"</h2>
            <Input value=removed_admin placeholder="Admin public key to remove" />
            <Button
                appearance=ButtonAppearance::Primary
                on_click=move |_| {
                    action.dispatch(());
                }
                loading=action.pending()
                disabled=Signal::derive(move || {
                    removed_admin.get().is_empty() || action.pending().get()
                })
            >
                "Remove Admin"
            </Button>
            <ActionStatus action=action />
        </div>
    }
}

#[component]
fn BlacklistAccountForm() -> impl IntoView {
    let public_key = use_context::<WalletPublicKeyContext>()
        .expect("Can't get wallet context")
        .public_key;
    let address = RwSignal::new(String::new());

    let action = Action::new_unsync(move |_| async move {
        #[cfg(feature = "hydrate")]
        {
            let key = public_key
                .get_untracked()
                .ok_or(crate::error::AppError::custom("No wallet connected"))?;
            let tx = build_blacklist_account_tx(key, address.get_untracked(), true)
                .await
                .map_err(|e| crate::error::AppError::custom(e.to_string()))?;
            let sig = crate::wallet::send_transaction(tx).await?;
            address.set(String::new());
            Ok::<String, crate::error::AppError>(sig)
        }
        #[cfg(not(feature = "hydrate"))]
        Ok::<String, crate::error::AppError>(String::new())
    });

    view! {
        <div class="admin-action-card">
            <h2>"Blacklist Account"</h2>
            <Input value=address placeholder="Account public key to blacklist" />
            <Button
                appearance=ButtonAppearance::Primary
                on_click=move |_| {
                    action.dispatch(());
                }
                loading=action.pending()
                disabled=Signal::derive(move || address.get().is_empty() || action.pending().get())
            >
                "Blacklist"
            </Button>
            <ActionStatus action=action />
        </div>
    }
}

#[component]
fn UnblacklistAccountForm() -> impl IntoView {
    let public_key = use_context::<WalletPublicKeyContext>()
        .expect("Can't get wallet context")
        .public_key;
    let address = RwSignal::new(String::new());

    let action = Action::new_unsync(move |_| async move {
        #[cfg(feature = "hydrate")]
        {
            let key = public_key
                .get_untracked()
                .ok_or(crate::error::AppError::custom("No wallet connected"))?;
            let tx = build_blacklist_account_tx(key, address.get_untracked(), false)
                .await
                .map_err(|e| crate::error::AppError::custom(e.to_string()))?;
            let sig = crate::wallet::send_transaction(tx).await?;
            address.set(String::new());
            Ok::<String, crate::error::AppError>(sig)
        }
        #[cfg(not(feature = "hydrate"))]
        Ok::<String, crate::error::AppError>(String::new())
    });

    view! {
        <div class="admin-action-card">
            <h2>"Unblacklist Account"</h2>
            <Input value=address placeholder="Account public key to unblacklist" />
            <Button
                appearance=ButtonAppearance::Primary
                on_click=move |_| {
                    action.dispatch(());
                }
                loading=action.pending()
                disabled=Signal::derive(move || address.get().is_empty() || action.pending().get())
            >
                "Unblacklist"
            </Button>
            <ActionStatus action=action />
        </div>
    }
}

#[component]
fn WithdrawPlatformFeeForm() -> impl IntoView {
    let public_key = use_context::<WalletPublicKeyContext>()
        .expect("Can't get wallet context")
        .public_key;
    let receiver = RwSignal::new(String::new());
    let amount = RwSignal::new(String::new());

    let action = Action::new_unsync(move |_| async move {
        #[cfg(feature = "hydrate")]
        {
            let key = public_key
                .get_untracked()
                .ok_or(crate::error::AppError::custom("No wallet connected"))?;
            let amt: u64 = amount
                .get_untracked()
                .parse()
                .map_err(|_| crate::error::AppError::custom("Invalid amount"))?;
            let tx = build_withdraw_platform_fee_tx(key, receiver.get_untracked(), amt)
                .await
                .map_err(|e| crate::error::AppError::custom(e.to_string()))?;
            let sig = crate::wallet::send_transaction(tx).await?;
            receiver.set(String::new());
            amount.set(String::new());
            Ok::<String, crate::error::AppError>(sig)
        }
        #[cfg(not(feature = "hydrate"))]
        Ok::<String, crate::error::AppError>(String::new())
    });

    view! {
        <div class="admin-action-card">
            <h2>"Withdraw Platform Fee"</h2>
            <Input value=receiver placeholder="Receiver public key" />
            <Input value=amount placeholder="Amount (lamports)" />
            <Button
                appearance=ButtonAppearance::Primary
                on_click=move |_| {
                    action.dispatch(());
                }
                loading=action.pending()
                disabled=Signal::derive(move || {
                    receiver.get().is_empty() || amount.get().is_empty() || action.pending().get()
                })
            >
                "Withdraw"
            </Button>
            <ActionStatus action=action />
        </div>
    }
}

#[component]
fn ActionStatus(action: Action<(), Result<String, crate::error::AppError>>) -> impl IntoView {
    view! {
        <p class="action-status">
            {move || {
                action
                    .value()
                    .get()
                    .map(|result| match result {
                        Ok(sig) => format!("Success: {sig}"),
                        Err(e) => format!("Error: {e}"),
                    })
            }}
        </p>
    }
}
