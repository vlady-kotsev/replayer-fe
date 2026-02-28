use leptos::prelude::*;
use leptos_router::components::{Outlet, Redirect};

use crate::{app::WalletPublicKeyContext, server::is_admin};

#[component]
pub fn AdminRoute() -> impl IntoView {
    let wallet_ctx = use_context::<WalletPublicKeyContext>().expect("Can't get wallet context");

    let is_admin = Resource::new(
        || false,
        move |_| async move {
            match wallet_ctx.public_key.get() {
                Some(key) => is_admin(key).await.unwrap_or(false),
                None => false,
            }
        },
    );

    provide_context(is_admin);

    view! {
        <Suspense fallback=|| {
            view! { <p>"Checking auth..."</p> }
        }>
            {move || {
                match is_admin.get() {
                    Some(true) => view! { <Outlet /> }.into_any(),
                    _ => view! { <Redirect path="/login" /> }.into_any(),
                }
            }}
        </Suspense>
    }
}
