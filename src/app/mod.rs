mod component;
mod context;
mod shell;

use crate::app::shell::shell;
use crate::error::{AppError, AppResult};
pub use component::*;
pub use context::*;
use std::net::SocketAddr;

#[cfg(feature = "ssr")]
pub struct App {
    pub router: axum::Router,
    pub addr: SocketAddr,
}

#[cfg(feature = "ssr")]
impl App {
    pub async fn new() -> AppResult<App> {
        use crate::config::{load_config, DEFAULT_CONFIG_FILE};
        use axum::Router;

        use leptos::prelude::*;
        use leptos_axum::{generate_route_list, LeptosRoutes};
        use solana_client::rpc_client::RpcClient;

        use std::sync::Arc;

        let app_config = load_config().await?;

        let leptos_config = get_configuration(Some(DEFAULT_CONFIG_FILE))
            .map_err(|e| AppError::custom(e.to_string()))?;

        let addr = leptos_config.leptos_options.site_addr;
        let leptos_options = leptos_config.leptos_options;
        let routes = generate_route_list(App);
        let solana_client = Arc::new(RpcClient::new(app_config.solana.rpc_url.clone()));

        let router = Router::new()
            .leptos_routes_with_context(
                &leptos_options,
                routes,
                {
                    let config = app_config.clone();
                    move || {
                        provide_context(config.clone());
                        provide_context(solana_client.clone());
                    }
                },
                {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                },
            )
            .fallback(leptos_axum::file_and_error_handler(shell))
            .with_state(leptos_options);

        Ok(Self { router, addr })
    }

    pub async fn run(self) -> AppResult<()> {
        use leptos::logging::log;
        log!("listening on {}", &self.addr);

        let listener = tokio::net::TcpListener::bind(&self.addr)
            .await
            .map_err(|e| AppError::custom(e.to_string()))?;

        axum::serve(listener, self.router.into_make_service())
            .await
            .map_err(|e| AppError::custom(e.to_string()))?;
        Ok(())
    }
}
