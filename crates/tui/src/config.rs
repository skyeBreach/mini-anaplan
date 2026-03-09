pub mod key_bindings;
pub mod theme;

use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{key_bindings::KeyBindings, theme::ThemeName};

const APP_NAME: &str = "mini-polaris";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub debug: bool,

    #[serde(default)]
    pub keybindings: KeyBindings,

    #[serde(default)]
    pub theme: ThemeName,
}

impl Config {
    pub fn new() -> Self {
        Self {
            debug: true,
            keybindings: KeyBindings::default(),
            theme: ThemeName::default(),
        }
    }
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content =
            std::fs::read_to_string(&config_path).context("Failed to read config file")?;

        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn get_config_path() -> Result<PathBuf> {
        // Try home dir first
        if let Some(home) = dirs::home_dir() {
            let bbcli_path = home.join(format!(".{}", APP_NAME));
            if bbcli_path.exists() {
                return Ok(bbcli_path);
            }
        }

        // Try config dir first
        if let Some(config_dir) = dirs::config_dir() {
            let bbcli_config = config_dir.join(APP_NAME);
            return Ok(bbcli_config);
        }

        // Fallback to home directory
        dirs::home_dir()
            .map(|h| h.join(format!(".{}", APP_NAME)))
            .context("Could not determine config path")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
