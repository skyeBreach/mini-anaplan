use std::io;

use anyhow::{Context, Result};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use mini_polaris_tui::{Config, app::App, theme::Theme};
use ratatui::{DefaultTerminal, Terminal, prelude::CrosstermBackend, run};

fn main() -> Result<()> {
    run(run_tui).context("failed to run app")
}

fn run_tui(_: &mut DefaultTerminal) -> Result<()> {
    // Load configuration
    let config = Config::load().unwrap_or_default();

    // Get theme from config
    let theme = Theme::from_name(&config.theme);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state with theme
    let mut app = App::new(theme);

    // Run the app
    let result = App::run(&mut terminal, &mut app, &config);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}
