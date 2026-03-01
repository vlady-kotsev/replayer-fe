use std::sync::Arc;

#[cfg(feature = "ssr")]
use replayer_fe::error::{AppError, AppResult};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> AppResult<()> {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use replayer_fe::app::*;
    use replayer_fe::config::{load_config, DEFAULT_CONFIG_FILE};
    use solana_client::rpc_client::RpcClient;

    let app_config = load_config().await?;

    let leptos_config = get_configuration(Some(DEFAULT_CONFIG_FILE))
        .map_err(|e| AppError::custom(e.to_string()))?;

    let addr = leptos_config.leptos_options.site_addr;
    let leptos_options = leptos_config.leptos_options;
    let routes = generate_route_list(App);
    let solana_client = Arc::new(RpcClient::new(app_config.solana.rpc_url.clone()));

    let app = Router::new()
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

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| AppError::custom(e.to_string()))?;
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| AppError::custom(e.to_string()))?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
