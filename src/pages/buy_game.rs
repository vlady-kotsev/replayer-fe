use crate::{components::GameCard, server::get_all_games};
use leptos::prelude::*;
use thaw::{Spinner, SpinnerSize};

#[component]
pub fn BuyGamePage() -> impl IntoView {
    let games = LocalResource::new(|| async move { get_all_games().await });
    let refetch_trigger = RwSignal::new(0usize);

    view! {
        <div class="buy-game-page">
            <h1>"Buy Games"</h1>
            <Suspense fallback=move || {
                view! { <Spinner size=SpinnerSize::ExtraLarge /> }
            }>
                {move || {
                    games
                        .get()
                        .map(|result| {
                            match result {
                                Ok(games) if games.is_empty() => {
                                    view! { <p>"No games available yet."</p> }.into_any()
                                }
                                Ok(games) => {
                                    view! {
                                        <div class="games-grid">
                                            {games
                                                .into_iter()
                                                .map(|game| {
                                                    view! {
                                                        <GameCard game=game refetch_trigger=refetch_trigger />
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <p class="error">{format!("Error loading games: {e}")}</p>
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}
