#[cfg(feature = "ssr")]
use replayer_fe::error::server_error::{ServerError, ServerResult};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> ServerResult<()> {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use replayer_fe::app::*;
    use replayer_fe::config::{load_config, DEFAULT_CONFIG_FILE};

    let config = load_config().await?;

    let conf = get_configuration(Some(DEFAULT_CONFIG_FILE))
        .map_err(|e| ServerError::internal(e.to_string()))?;

    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let config = config.clone();
                move || {
                    provide_context(config.clone());
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
        .map_err(|e| ServerError::internal(e.to_string()))?;
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| ServerError::internal(e.to_string()))?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
