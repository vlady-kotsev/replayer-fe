use crate::{app::WalletPublicKeyContext, components::GameScreen, server::get_owned_games};
use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Select, Spinner, SpinnerSize};

#[component]
pub fn PlayPage() -> impl IntoView {
    let public_key = use_context::<WalletPublicKeyContext>()
        .expect("Can't get wallet context")
        .public_key;

    let selected_game = RwSignal::new(String::new());
    let game_to_play = RwSignal::new(String::new());

    let owned_games = LocalResource::new(move || {
        let key = public_key.get();
        async move {
            match key {
                Some(k) => get_owned_games(k).await,
                None => Ok(vec![]),
            }
        }
    });

    let on_play = move |_| {
        game_to_play.set(selected_game.get_untracked());
    };

    view! {
        <div class="play-page">
            <h1>"Games"</h1>
            <Show
                when=move || public_key.get().is_some()
                fallback=|| view! { <p>"Connect your wallet to see your games."</p> }
            >
            <GameScreen game_to_play=game_to_play/>
                <Suspense fallback=move || {
                    view! { <Spinner size=SpinnerSize::ExtraLarge /> }
                }>
                    {move || {
                        owned_games
                            .get()
                            .map(|result| {
                                match result {
                                    Ok(games) if games.is_empty() => {
                                        view! { <p>"You don't own any games yet."</p> }.into_any()
                                    }
                                    Ok(games) => {
                                        view! {
                                            <div class="play-controls">
                                                <Select value=selected_game>
                                                    <option value="" disabled selected>"Select a game"</option>
                                                    {games
                                                        .into_iter()
                                                        .map(|game| {
                                                            let name = game.data.game_name.clone();
                                                            let val = format!("{}|{}", game.data.developer, name);
                                                            let display_name = name;
                                                            view! {
                                                                <option value=val>{display_name}</option>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </Select>
                                                <Button
                                                    appearance=ButtonAppearance::Primary
                                                    on_click=on_play
                                                    disabled=Signal::derive(move || selected_game.get().is_empty())
                                                >
                                                    "Play"
                                                </Button>
                                            </div>
                                        }
                                            .into_any()
                                    }
                                    Err(e) => {
                                        leptos::logging::log!("Error loading games: {e}");
                                        view! {
                                            <p class="error">
                                                "Something went wrong. Please try again."
                                            </p>
                                        }
                                            .into_any()
                                    }
                                }
                            })
                    }}
                </Suspense>

            </Show>
        </div>
    }
}
