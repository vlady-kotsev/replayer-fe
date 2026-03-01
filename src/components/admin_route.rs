use leptos::prelude::*;
use leptos_router::components::{Outlet, Redirect};
use thaw::{Spinner, SpinnerSize};

use crate::{app::WalletPublicKeyContext, server::is_admin};

#[component]
pub fn AdminRoute() -> impl IntoView {
    let wallet_ctx = use_context::<WalletPublicKeyContext>().expect("Can't get wallet context");

    let is_admin = Resource::new(
        move || wallet_ctx.public_key.get(),
        move |key| async move {
            match key {
                Some(key) => Some(is_admin(key).await.unwrap_or(false)),
                None => None,
            }
        },
    );

    provide_context(is_admin);

    view! {
        <Suspense fallback=|| {
            view! { <Spinner size=SpinnerSize::ExtraLarge/> }
        }>
            {move || {
                is_admin.get().map(|result| {
                    match result {
                        Some(true) => view! { <Outlet /> }.into_any(),
                        Some(false) => view! { <Redirect path="/" /> }.into_any(),
                        None => view! { <Spinner size=SpinnerSize::Huge/> }.into_any(),
                    }
                })
            }}
        </Suspense>
    }
}
