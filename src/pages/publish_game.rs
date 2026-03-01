use leptos::prelude::*;

use crate::components::{DeveloperGate, GameUpload};

#[component]
pub fn PublishGamePage() -> impl IntoView {
    view! {
        <h1>"Publish Game"</h1>

        <div class="publish-layout">
            <img class="publish-hero" src="/publish-game.png" alt="Publish Game" />
            <DeveloperGate>
                <GameUpload />
            </DeveloperGate>
        </div>
    }
}
