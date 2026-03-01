use leptos::prelude::*;
use leptos_router::components::*;

use crate::{app::WalletPublicKeyContext, components::WalletButton, server::is_admin};

#[component]
pub fn Nav() -> impl IntoView {
    let wallet_ctx = use_context::<WalletPublicKeyContext>().expect("Can't get wallet context");

    let check_admin = Resource::new(
        move || wallet_ctx.public_key.get(),
        move |key| async move {
            match key {
                Some(key) => is_admin(key).await.unwrap_or(false),
                None => false,
            }
        },
    );

    view! {
        <nav class="site-nav">
            <A href="/" attr:class="nav-logo">
                <img src="/logo.webp" alt="Logo" class="nav-logo" />
            </A>
            <ul class="nav-links">
                <li>
                    <A href="/play">"Play"</A>
                </li>
                <li>
                    <A href="/buy">"Buy Games"</A>
                </li>
                <li>
                    <A href="/how-it-works">"How it works"</A>
                </li>
                <li>
                    <A href="/publish">"Publish Games"</A>
                </li>
                <Transition>
                    <Show when=move || check_admin.get().unwrap_or(false)>
                        <li>
                            <A href="/admin">"Admin"</A>
                        </li>
                    </Show>
                </Transition>
            </ul>
            <div class="nav-spacer"></div>
            <WalletButton />
        </nav>
    }
}
