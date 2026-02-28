use leptos::prelude::*;

use crate::components::{DeveloperGate, GameUpload};

#[component]
pub fn PublishGamePage() -> impl IntoView {
    view! {
        <h1>Publish Game</h1>

        <DeveloperGate>
            <GameUpload />
        </DeveloperGate>
    }
}
