mod api;
pub mod app;
mod components;
pub mod config;
pub mod error;
mod models;
mod pages;
mod server;
mod solana;
mod utils;
mod wallet;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
