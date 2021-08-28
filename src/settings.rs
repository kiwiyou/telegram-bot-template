use config::{Config, ConfigError, File};
use log::LevelFilter;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case", remote = "LevelFilter")]
enum LevelFilterDef {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    #[serde(with = "LevelFilterDef")]
    pub level: LevelFilter,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub bind_address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Bot {
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
    pub bot: Bot,
    pub env: Env,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Env {
    Development,
    Testing,
    Production,
}

use std::fmt;
impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Development => write!(f, "development"),
            Self::Testing => write!(f, "testing"),
            Self::Production => write!(f, "production"),
        }
    }
}

const CONFIG_FILE_PATH: &str = "./config/default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        let mut s = Config::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        s.try_into()
    }
}
