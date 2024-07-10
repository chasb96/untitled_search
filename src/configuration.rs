use std::{env, sync::OnceLock};
use log_unwrap::LogUnwrap;
use serde::Deserialize;

static CONFIGURATION: OnceLock<Configuration> = OnceLock::new();

#[derive(Deserialize)]
pub struct Configuration {
    pub database_url: String,
}

impl Configuration {
    pub fn from_env() -> Self {
        Configuration {
            database_url: env::var("SEARCH_DATABASE_URL").log_unwrap(),
        }
    }
}

impl Default for &Configuration {
    fn default() -> Self {
        CONFIGURATION.get_or_init(Configuration::from_env)
    }
}