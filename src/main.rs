#[cfg(feature = "ssr")]
use replayer_fe::{app::App, error::AppResult};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> AppResult<()> {
    let app = App::new().await?;
    app.run().await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
