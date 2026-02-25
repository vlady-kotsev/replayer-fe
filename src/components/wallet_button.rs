use crate::error::AppError;
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance};

#[component]
pub fn WalletButton() -> impl IntoView {
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

    let pending = Signal::derive(move || {
        connect_action.pending().get() || disconnect_action.pending().get()
    });

    view! {
        <div class="wallet">
            {move || {
                if pending.get() {
                    view! {
                        <Button appearance=ButtonAppearance::Primary loading=true>
                            "Connecting..."
                        </Button>
                    }
                        .into_any()
                } else if let Some(k) = key.read().as_ref() {
                    let short = format!("{}...{}", &k[..4], &k[k.len() - 4..]);
                    view! {
                        <div class="wallet__info">
                            <Button
                                appearance=ButtonAppearance::Primary
                                on_click=move |_| { _ = disconnect_action.dispatch(()) }
                            >
                                {short}
                            </Button>
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=move |_| { _ = connect_action.dispatch(()) }
                        >
                            "Connect Phantom"
                        </Button>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
