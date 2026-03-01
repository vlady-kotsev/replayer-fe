use leptos::prelude::*;

#[derive(Clone)]
pub struct WalletPublicKeyContext {
    pub public_key: Signal<Option<String>>,
    pub set_public_key: WriteSignal<Option<String>>,
}
