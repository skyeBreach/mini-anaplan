use std::io;
use std::time::{Instant, Duration};

use anyhow::Result;
use mini_polaris_core::modelling::model::Model;
use ratatui::prelude::CrosstermBackend;
use ratatui::{Terminal};


use crate::theme::Theme;
use crate::{Config, event, ui};

pub struct App {
    // Polaris Data
    pub models: Vec<Model>,

    // Config
    pub theme: Theme,

    // Tick
    pub ticker_index: usize,
    pub ticker_counter: u32,

    // Drawing
    pub should_redraw: bool,

    // Loading
    pub is_loading: bool,

    // Refreshing
    pub is_refreshing: bool,
    pub last_refreshed_at: Instant,

    // Error Handling
    pub error_message: Option<String>,

    // Quitting
    pub should_quit: bool,
}

impl App {
    pub fn new(theme: Theme) -> Self {
        Self {
            // Polaris Data
            models: Vec::new(),

            // Config
            theme,

            // Tick
            ticker_index: 0,
            ticker_counter: 0,

            // Drawing
            should_redraw: true,

            // Loading
            is_loading: false,

            // Refreshing
            is_refreshing: false,
            last_refreshed_at: Instant::now(),

            // Error Handling
            error_message: None,

            // Quitting
            should_quit: false,
        }
    }

    pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App, config: &Config) -> Result<()> {
        // Initial draw
        terminal.draw(|f| ui::render(f, app))?;

        loop {
            // Track state before event handling to detect changes
            let prev_ticker_index = app.ticker_index;
            let prev_loading = app.is_loading;
            let prev_refreshing = app.is_refreshing;
        

            // Check for auto-refresh (every 5 minutes)
            let mut action = event::handle_events(app, config)?;

            if app.check_auto_refresh() && matches!(action, event::AppAction::None) {
                action = event::AppAction::Refresh;
            }

            match action {
                event::AppAction::Refresh => {
                    // Keep stories visible during refresh (no loading placeholder)
                    app.is_refreshing = true;
                    app.clear_error();
                    // Redraw to show "Refreshing News..." message
                    terminal.draw(|f| ui::render(f, app))?;
                    app.mark_refreshed();
                }
                event::AppAction::None => {}
            }

            // Update ticker rotation and check if clock should update
            let clock_tick = app.tick();

            // Only redraw if something actually changed
            let should_redraw = clock_tick // Redraw when clock ticks (approximately every second)
                || prev_ticker_index != app.ticker_index 
                || prev_loading != app.is_loading
                || prev_refreshing != app.is_refreshing;

            if should_redraw {
                terminal.draw(|f| ui::render(f, app))?;
            }

            if app.should_quit {
                break;
            }
        }

        Ok(())
    }

    pub fn tick(&mut self) -> bool {
        self.ticker_counter += 1;

        // Return true every 10 ticks (approximately 1 second) to trigger clock update
        self.ticker_counter.is_multiple_of(10)
    }

    pub fn check_auto_refresh(&self) -> bool {
        // Auto-refresh every 5 minutes (300 seconds)
        const AUTO_REFRESH_INTERVAL: Duration = Duration::from_secs(300);
        self.last_refreshed_at.elapsed() >= AUTO_REFRESH_INTERVAL
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn mark_refreshed(&mut self) {
        self.last_refreshed_at = Instant::now();
    }

    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
        self.is_loading = false;
        self.is_refreshing = false;
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
}

