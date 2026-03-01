use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Input, Spinner, SpinnerSize};

use crate::{
    app::WalletPublicKeyContext,
    server::{build_create_developer_tx, developer_exists},
};

#[component]
pub fn DeveloperGate(children: ChildrenFn) -> impl IntoView {
    let public_key = use_context::<WalletPublicKeyContext>()
        .expect("Can't get wallet context")
        .public_key;

    let company_name = RwSignal::new(String::new());
    let collection_uri = RwSignal::new(String::new());
    let created = RwSignal::new(false);

    let dev_check = Resource::new(
        move || public_key.get(),
        move |key| async move {
            #[cfg(feature = "hydrate")]
            {
                match key {
                    Some(k) => developer_exists(k).await.unwrap_or(false),
                    None => false,
                }
            }
            #[cfg(not(feature = "hydrate"))]
            false
        },
    );

    let create_action = Action::new_unsync(move |_| async move {
        #[cfg(feature = "hydrate")]
        {
            let result = async {
                let key = public_key
                    .get_untracked()
                    .ok_or(crate::error::AppError::custom("No wallet connected"))?;
                let tx = build_create_developer_tx(
                    key,
                    company_name.get_untracked(),
                    collection_uri.get_untracked(),
                )
                .await
                .map_err(|e| crate::error::AppError::custom(e.to_string()))?;
                crate::wallet::send_transaction(tx).await
            }
            .await;
            if result.is_ok() {
                created.set(true);
            }
            result
        }
        #[cfg(not(feature = "hydrate"))]
        Ok::<String, crate::error::AppError>(String::new())
    });

    let wallet_connected = move || public_key.get().is_some();
    let is_loading = move || wallet_connected() && dev_check.get().is_none() && !created.get();
    let is_developer =
        move || wallet_connected() && (created.get() || dev_check.get().unwrap_or(false));
    let needs_registration = move || wallet_connected() && !is_loading() && !is_developer();

    view! {
        <div class="create-developer">
            <Show when=move || !wallet_connected()>
                <p>"Connect your wallet to continue"</p>
            </Show>
            <Show when=is_loading>
                <Spinner size=SpinnerSize::ExtraLarge />
            </Show>
            <Show when=is_developer>{children()}</Show>
            <Show when=needs_registration>
                <div>
                    <h3>"Register as Developer"</h3>
                    <Input value=company_name placeholder="Company Name" />
                    <Input value=collection_uri placeholder="Collection URI" />
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=move |_| {
                            create_action.dispatch(());
                        }
                        loading=create_action.pending()
                    >
                        "Create Developer"
                    </Button>
                </div>
            </Show>
        </div>
    }
}
