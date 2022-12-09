use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use toml::de::Error as TomlError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub matrix: MatrixConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatrixConfig {
    /// The initial display name when first logging in to a homeserver.
    pub initial_display_name: Option<String>,

    /// The root matrix space.
    pub space: MatrixSpace,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatrixSpace {
    /// The room ID of the space.
    pub room_id: String,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    /// An error occurred during TOML deserialization.
    #[error(transparent)]
    SerdeToml(#[from] TomlError),

    #[error("Unable to open config file `{0}'")]
    UnableToReadFile(String),
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let path = path.as_ref();
    let data = std::fs::read_to_string(path)
        .map_err(|_| ConfigError::UnableToReadFile(path.to_str().unwrap().to_owned()))?;
    let config: Config = toml::from_str(&data)?;

    Ok(config)
}
