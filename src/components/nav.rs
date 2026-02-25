use leptos::prelude::*;
use leptos_router::components::*;

use crate::components::WalletButton;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="site-nav">
            <A href="/" attr:class="nav-logo">
                <img src="/logo.webp" alt="Logo" class="nav-logo" />
            </A>
            <ul class="nav-links">
                <li>
                    <A href="/">"Play"</A>
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
            </ul>
            <div class="nav-spacer"></div>
                <WalletButton />
        </nav>
    }
}
