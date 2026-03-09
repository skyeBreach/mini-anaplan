use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyBindings {
    #[serde(default = "default_quit")]
    pub quit: char,

    #[serde(default = "default_refresh")]
    pub refresh: char,
}

fn default_quit() -> char {
    'q'
}

fn default_refresh() -> char {
    'r'
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: default_quit(),
            refresh: default_refresh(),
        }
    }
}
