use leptos::prelude::*;

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
