use leptos::{
    prelude::*,
    server::codee::string::{FromToStringCodec, OptionCodec},
};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};
use leptos_use::storage::{use_local_storage_with_options, UseStorageOptions};
use thaw::{ConfigProvider, Theme};

use crate::{
    app::WalletPublicKeyContext,
    components::{AdminRoute, Footer, Nav},
    pages::{AdminDashboard, BuyGamePage, HomePage, PublishGamePage},
    utils::LS_PUBLIC_KEY,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (public_key, set_public_key, _) =
        use_local_storage_with_options::<Option<String>, OptionCodec<FromToStringCodec>>(
            LS_PUBLIC_KEY,
            UseStorageOptions::default().delay_during_hydration(true),
        );

    let wallet_ctx = WalletPublicKeyContext {
        public_key,
        set_public_key,
    };

    provide_context(wallet_ctx);

    view! {
        <Stylesheet id="leptos" href="/pkg/replayer-fe.css" />

        <Title text="Replayer" />

        <Router>
            <ConfigProvider theme=RwSignal::new(Theme::dark())>
                <Nav />
                <main class="main-content">
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=StaticSegment("/buy") view=BuyGamePage />
                        <Route path=StaticSegment("/publish") view=PublishGamePage />
                        <ParentRoute path=StaticSegment("/admin") view=AdminRoute>
                            <Route path=StaticSegment("") view=AdminDashboard />
                        </ParentRoute>
                    </Routes>
                </main>
                <Footer />
            </ConfigProvider>
        </Router>
    }
}
