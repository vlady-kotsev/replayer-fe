#[cfg(feature = "ssr")]
pub mod config {
    use crate::{
        error::{AppError, AppResult},
        utils::deserialize_address,
    };
    use serde::Deserialize;
    use solana_pubkey::Pubkey;

    pub const DEFAULT_CONFIG_FILE: &'static str = "config/config.toml";

    #[derive(Clone, Deserialize)]
    pub struct Config {
        pub app: AppConfig,
        pub solana: SolanaConfig,
    }

    #[derive(Clone, Deserialize)]
    pub struct SolanaConfig {
        pub rpc_url: String,
        #[serde(deserialize_with = "deserialize_address")]
        pub program_id: Pubkey,
    }

    #[derive(Clone, Deserialize)]
    pub struct AppConfig {
        pub backend_url: String,
    }

    pub async fn load_config() -> AppResult<Config> {
        let config_file = std::env::var("CONFIG_DIR").unwrap_or(String::from(DEFAULT_CONFIG_FILE));

        let content = tokio::fs::read_to_string(config_file)
            .await
            .map_err(|e| AppError::custom(e.to_string()))?;

        let config =
            toml::from_str::<Config>(&content).map_err(|e| AppError::custom(e.to_string()))?;

        Ok(config)
    }
}
