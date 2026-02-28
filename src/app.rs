use leptos::{
    prelude::*,
    server::codee::string::{FromToStringCodec, OptionCodec},
};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_use::storage::{use_local_storage_with_options, UseStorageOptions};
use thaw::{ssr::SSRMountStyleProvider, ConfigProvider, Theme};

use crate::{
    components::{AdminRoute, Footer, Nav},
    constants::LS_PUBLIC_KEY,
    pages::PublishGamePage,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <link rel="preconnect" href="https://fonts.googleapis.com" />
                    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                    <link
                        href="https://fonts.googleapis.com/css2?family=Bitcount+Grid+Double:wght@100..900&display=swap"
                        rel="stylesheet"
                    />
                    <link rel="icon" type="image/png" href="/logo.webp" />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options />
                    <MetaTags />
                </head>
                <body>
                    <App />
                </body>
            </html>
        </SSRMountStyleProvider>
    }
}

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
                        <Route path=StaticSegment("/publish") view=PublishGamePage />
                        <Route path=StaticSegment("/admin") view=AdminRoute />
                    </Routes>
                </main>
                <Footer />
            </ConfigProvider>
        </Router>
    }
}

#[derive(Clone)]
pub struct WalletPublicKeyContext {
    pub public_key: Signal<Option<String>>,
    pub set_public_key: WriteSignal<Option<String>>,
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
