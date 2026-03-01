use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <h1 class="home-title">"Replayer"</h1>
            <p class="home-subtitle">"Retro games, on-chain. Play, publish, own."</p>
            <a class="home-hero-link" href="/play">
                <img class="home-hero" src="/home.png" alt="Replayer" />
            </a>
        </div>
    }
}
