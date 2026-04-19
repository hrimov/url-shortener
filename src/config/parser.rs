use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use super::models::Config;

type StringSetter = fn(&mut Config, String);
type U16Setter = fn(&mut Config, u16);

const STRING_OVERRIDES: &[(&str, StringSetter)] = &[
    ("DB_USER", |c, v| c.database.user = v),
    ("DB_PASSWORD", |c, v| c.database.password = v),
    ("DB_NAME", |c, v| c.database.database_name = v),
    ("DB_HOST", |c, v| c.database.host = v),
    ("REDIS_HOST", |c, v| c.cache.host = v),
    ("APP_HOST", |c, v| c.application.host = v),
];

const U16_OVERRIDES: &[(&str, U16Setter)] = &[
    ("DB_PORT", |c, v| c.database.port = v),
    ("REDIS_PORT", |c, v| c.cache.port = v),
    ("APP_PORT", |c, v| c.application.port = v),
];

fn apply_env_overrides(config: &mut Config) {
    for (var, set) in STRING_OVERRIDES {
        if let Ok(v) = env::var(var) {
            set(config, v);
        }
    }
    for (var, set) in U16_OVERRIDES {
        if let Ok(v) = env::var(var)
            && let Ok(parsed) = v.parse()
        {
            set(config, parsed);
        }
    }
}

pub fn load_config<P: AsRef<Path>>(file_path: P) -> Result<Config, ConfigError> {
    let contents = fs::read_to_string(file_path).map_err(ConfigError::ReadError)?;
    let mut config: Config = toml::from_str(&contents).map_err(ConfigError::ParseError)?;
    apply_env_overrides(&mut config);
    Ok(config)
}

#[derive(Debug)]
pub enum ConfigError {
    ReadError(io::Error),
    ParseError(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ReadError(e) => write!(f, "Failed to read config file: {}", e),
            ConfigError::ParseError(e) => write!(f, "Failed to parse config file: {}", e),
        }
    }
}

impl StdError for ConfigError {}
