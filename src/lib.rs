mod api;
pub mod app;
mod components;
pub mod config;
mod constants;
pub mod error;
#[cfg(feature = "ssr")]
mod generated;
mod models;
mod pages;
mod server;
mod utils;
mod wallet;

#[cfg(feature = "ssr")]
pub use generated::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
