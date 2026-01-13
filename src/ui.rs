use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppMode};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3), // Help/Status bar
        ])
        .split(f.area());

    // Main Content
    let title = if let Some(n) = app.current_scp_number {
        format!(" SCP-{:03} ", n)
    } else {
        " SCP Reader ".to_string()
    };

    let content_text = Text::from(app.content.clone());
    let paragraph = Paragraph::new(content_text)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: true })
        .scroll((app.scroll, 0)); // (row, col)

    f.render_widget(paragraph, chunks[0]);

    // Status/Help Bar
    let status_text = match app.mode {
        AppMode::Normal => "Normal Mode | [q] Quit | [r] Random | [/] Search | [j/k] Scroll",
        AppMode::Input => "Input Mode | [Esc] Cancel | [Enter] Confirm",
        AppMode::Loading => "Loading...",
    };
    
    let error_text = if let Some(err) = &app.error_msg {
        format!(" | Error: {}", err)
    } else {
        String::new()
    };

    let status_paragraph = Paragraph::new(format!("{}{}", status_text, error_text))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status_paragraph, chunks[1]);

    // Input Popup
    if let AppMode::Input = app.mode {
        let block = Block::default().title(" Enter SCP Number ").borders(Borders::ALL);
        let area = centered_rect(60, 20, f.area());
        f.render_widget(Clear, area); // Clear background
        
        let input_text = Paragraph::new(app.input_buffer.clone())
            .block(block)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(input_text, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);

    layout[1]
}
