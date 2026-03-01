use crate::app::App;
use leptos::prelude::*;
use leptos_meta::MetaTags;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
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
    }
}
