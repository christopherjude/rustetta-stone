use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    let top_area = chunks[0];
    let bottom_area = chunks[1];

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(top_area);

    let readers_area = top_chunks[0];
    let apdu_area = top_chunks[1];

    draw_panel(frame, readers_area, "Readers");
    draw_panel(frame, apdu_area, "APDU builder");
    draw_panel(frame, bottom_area, "APDU Logs");
}

fn draw_panel(frame: &mut Frame, area: Rect, title: &str) {
    let block = Block::default().title(title).borders(Borders::ALL);

    frame.render_widget(block, area);
}
