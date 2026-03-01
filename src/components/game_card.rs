use leptos::prelude::*;
use thaw::{
    Button, Card, CardFooter, CardHeader, CardHeaderDescription, Image, ImageShape, Spinner,
    SpinnerSize, Toast, ToastBody, ToastIntent, ToastOptions, ToastTitle, ToasterInjection,
};

use crate::{
    app::WalletPublicKeyContext,
    server::{check_game_is_owned, FetchedGameMetadata},
};

#[component]
pub fn GameCard(game: FetchedGameMetadata, refetch_trigger: RwSignal<usize>) -> impl IntoView {
    let public_key = use_context::<WalletPublicKeyContext>()
        .expect("Can't get wallet context")
        .public_key;

    let game_name = game.data.game_name;
    let developer = game.data.developer.to_string();
    let price_text = format!("{} lamports", game.data.price);
    let supply_text = format!("{} / {}", game.data.current_supply, game.data.max_supply);

    let dev_for_check = developer.clone();
    let name_for_check = game_name.clone();
    let is_owned = LocalResource::new(move || {
        refetch_trigger.track();
        let dev = dev_for_check.clone();
        let name = name_for_check.clone();
        async move {
            match public_key.get() {
                Some(k) => check_game_is_owned(k, dev, name).await.unwrap_or(false),
                None => false,
            }
        }
    });

    let toaster = ToasterInjection::expect_context();

    let dev_for_buy = developer.clone();
    let name_for_buy = game_name.clone();
    let buy_action: Action<(), Result<String, crate::error::AppError>> =
        Action::new_unsync(move |_| {
            let dev = dev_for_buy.clone();
            let name = name_for_buy.clone();
            async move {
                #[cfg(feature = "hydrate")]
                {
                    use crate::server::build_buy_game_tx;
                    let key = public_key
                        .get_untracked()
                        .ok_or(crate::error::AppError::custom("No wallet connected"))?;
                    let tx = build_buy_game_tx(key, dev, name)
                        .await
                        .map_err(|e| crate::error::AppError::custom(e.to_string()))?;
                    let sig = crate::wallet::send_transaction(tx).await?;
                    refetch_trigger.update(|v| *v += 1);
                    Ok::<String, crate::error::AppError>(sig)
                }
                #[cfg(not(feature = "hydrate"))]
                Ok::<String, crate::error::AppError>(String::new())
            }
        });

    Effect::new(move || {
        if let Some(result) = buy_action.value().get() {
            match result {
                Ok(_) => {
                    toaster.dispatch_toast(
                        move || {
                            view! {
                                <Toast>
                                    <ToastTitle>"Game Purchased"</ToastTitle>
                                    <ToastBody>"Your game will be available to play shortly."</ToastBody>
                                </Toast>
                            }
                        },
                        ToastOptions::default().with_intent(ToastIntent::Success),
                    );
                    #[cfg(feature = "hydrate")]
                    {
                        glitterbomb::cannon();
                    }
                }
                Err(ref e) => {
                    leptos::logging::log!("Buy error: {e}");
                    toaster.dispatch_toast(
                        move || {
                            view! {
                                <Toast>
                                    <ToastTitle>"Error"</ToastTitle>
                                    <ToastBody>"Something went wrong. Please try again."</ToastBody>
                                </Toast>
                            }
                        },
                        ToastOptions::default().with_intent(ToastIntent::Error),
                    );
                }
            }
        }
    });

    view! {
        <div class="game-card">
        <Card>
            <CardHeader>
                <h3>{game_name}</h3>
                <CardHeaderDescription slot>
                    <span class="game-price">{price_text}</span>
                </CardHeaderDescription>
            </CardHeader>
            <p class="game-supply">"Supply: " {supply_text}</p>
             <Image src={game.data.game_uri} width="200px" height="200px" shape=ImageShape::Rounded/>
            <Show when=move || public_key.get().is_some()>
                <CardFooter>
                    <Suspense fallback=|| {
                        view! { <Spinner size=SpinnerSize::Small /> }
                    }>
                        {move || {
                            is_owned
                                .get()
                                .map(|owned| {
                                    if owned {
                                        view! { <Button disabled=true>"Purchased"</Button> }.into_any()
                                    } else {
                                        view! {
                                            <Button
                                                on_click=move |_| {
                                                    buy_action.dispatch(());
                                                }
                                                loading=buy_action.pending()
                                            >
                                                "Buy"
                                            </Button>
                                        }
                                            .into_any()
                                    }
                                })
                        }}
                    </Suspense>
                </CardFooter>
            </Show>
        </Card>
        </div>
    }
}
