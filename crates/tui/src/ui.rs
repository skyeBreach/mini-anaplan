use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::App;

pub fn render(frame: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Footer (no border)
        ])
        .split(frame.area());

    render_header(frame, main_chunks[0], app);

    render_footer(frame, main_chunks[2], app);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    // Add offline indicator if in offline mode
    let title_text = "Mini Polaris";

    let header_text = vec![Line::from(Span::styled(
        title_text,
        Style::default()
            .fg(app.theme.accent_fg)
            .add_modifier(Modifier::BOLD),
    ))];

    let header = Paragraph::new(header_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .style(Style::default().bg(app.theme.accent)),
        );

    f.render_widget(header, area);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    use chrono::Local;

    // Split footer into left and right sections
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),    // Left: status/ticker
            Constraint::Length(8), // Right: time (HH:MM:SS)
        ])
        .split(area);

    // Left side
    let footer_left = Paragraph::new("q: quit | r: refresh".to_string())
        .style(
            Style::default()
                .fg(app.theme.fg_primary)
                .bg(app.theme.bg_accent),
        )
        .alignment(Alignment::Left);

    // Right side: current time with seconds
    let current_time = Local::now().format("%H:%M:%S").to_string();
    let footer_right = Paragraph::new(current_time)
        .style(
            Style::default()
                .fg(app.theme.fg_primary)
                .bg(app.theme.bg_accent),
        )
        .alignment(Alignment::Right);

    f.render_widget(footer_left, footer_chunks[0]);
    f.render_widget(footer_right, footer_chunks[1]);
}
