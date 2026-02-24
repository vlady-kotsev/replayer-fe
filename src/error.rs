use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub mod server_error {
    use axum::http::StatusCode;

    #[derive(Debug, Clone)]
    pub struct ServerError {
        pub status: StatusCode,
        pub message: String,
    }

    impl ServerError {
        pub fn internal(msg: impl Into<String>) -> Self {
            ServerError {
                message: msg.into(),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            }
        }

        pub fn bad_request(msg: impl Into<String>) -> Self {
            ServerError {
                status: StatusCode::BAD_REQUEST,
                message: msg.into(),
            }
        }

        pub fn forbidden(msg: impl Into<String>) -> Self {
            ServerError {
                status: StatusCode::FORBIDDEN,
                message: msg.into(),
            }
        }

        pub fn not_found(msg: impl Into<String>) -> Self {
            ServerError {
                status: StatusCode::NOT_FOUND,
                message: msg.into(),
            }
        }
    }

    impl std::fmt::Display for ServerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl std::error::Error for ServerError {}

    impl From<String> for ServerError {
        fn from(message: String) -> Self {
            Self {
                message,
                status: StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }

    pub type ServerResult<T> = Result<T, ServerError>;
}

#[derive(Debug, Clone)]
pub struct AppError {
    pub message: String,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

#[component]
pub fn ErrorView(errors: ArcRwSignal<Errors>) -> impl IntoView {
    view! {
        <div class="error-boundary">
            {move || {
                errors
                    .get()
                    .into_iter()
                    .map(|(_, e)| {
                        let app_err = e
                            .downcast_ref::<AppError>()
                            .cloned()
                            .unwrap_or(AppError::from(e.to_string()));

                        view! {
                            <div>
                                <strong>"Error: "</strong>
                                <span>{app_err.message}</span>
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

impl From<String> for AppError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

pub type AppResult<T> = Result<T, AppError>;
