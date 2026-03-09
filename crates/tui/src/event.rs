use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{App, Config};

pub enum AppAction {
    None,
    Refresh,
}

pub fn handle_events(app: &mut App, config: &Config) -> Result<AppAction> {
    if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                return handle_key_event(app, config, key);
            }
            _ => {}
        }
    }
    Ok(AppAction::None)
}

fn handle_key_event(app: &mut App, config: &Config, key: KeyEvent) -> Result<AppAction> {
    let kb = &config.keybindings;

    // Normal mode key handling
    match key.code {
        KeyCode::Char(c) if c == kb.quit => app.quit(),
        KeyCode::Char(c) if c == kb.refresh => return Ok(AppAction::Refresh),

        _ => {}
    }

    Ok(AppAction::None)
}
