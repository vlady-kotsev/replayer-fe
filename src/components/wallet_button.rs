use crate::error::AppError;
use leptos::prelude::*;

#[component]
pub fn WalletButton() -> Result<impl IntoView, AppError> {
    #[allow(unused_variables)]
    let (key, set_key) = signal::<Option<String>>(None);
    let connect_action = Action::new_unsync(move |_| async move {
        #[cfg(feature = "hydrate")]
        {
            crate::wallet::connect_phantom()
                .await
                .and_then(|key| Ok(set_key.set(Some(key))))
                .map_err(AppError::from)
        }
        #[cfg(not(feature = "hydrate"))]
        {
            Ok::<String, AppError>(String::new())
        }
    });

    let disconnect_action = Action::new_unsync(move |_| async move {
        #[cfg(feature = "hydrate")]
        {
            crate::wallet::disconnect_phantom()
                .await
                .and_then(|_| Ok(set_key.set(None)))
                .map_err(AppError::from)
        }
        #[cfg(not(feature = "hydrate"))]
        {
            Ok::<(), AppError>(())
        }
    });

    Ok(view! {
        <div class="wallet">
            {move || {
                let pending = connect_action.pending().get() || disconnect_action.pending().get();
                if pending {

                    view! { <span class="btn-primary">"Connecting..."</span> }
                        .into_any()
                } else if let Some(k) = key.read().as_ref() {
                    let short = format!("{}...{}", &k[..4], &k[k.len() - 4..]);
                    view! {
                        <div class="wallet__info">
                            <button
                                class="btn-primary"
                                on:click=move |_| { _ = disconnect_action.dispatch(()) }
                            >
                                {short}
                            </button>
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <button
                            class="btn-primary"
                            on:click=move |_| { _ = connect_action.dispatch(()) }
                        >
                            "Connect Phantom"
                        </button>
                    }
                        .into_any()
                }
            }}
        </div>
    })
}
