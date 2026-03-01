#![recursion_limit = "256"]

pub mod app;
mod components;
pub mod config;
pub mod error;
mod generated;
mod models;
mod pages;
mod server;
mod utils;
mod vm;
mod wallet;
pub use generated::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
