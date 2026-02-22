use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="footer__content">
                <div class="footer__brand">
                    <span class="footer__logo">
                        "Re"<span class="logo-accent">"player"</span>
                    </span>
                    <p class="footer__tagline">"Replay your favorite games"</p>
                </div>
                <div class="footer__links">
                    <a href="/how-it-works">"How it works"</a>
                    <a href="/publish">"Publish Games"</a>
                </div>
            </div>
            <div class="footer__bottom">
                <span>"© 2026 Replayer. All rights reserved."</span>
            </div>
        </footer>
    }
}
