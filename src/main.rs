mod app;
mod events;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{poll, read, Event as CEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    while app.running {
        terminal.draw(|frame| {
            let area = frame.area();

            let message = match &app.last_key {
                None => "Press any key (press 'q' to quit)".to_string(),
                Some(key) => format!("Last key {:?} (press 'q' to quit)", key),
            };

            let block = Block::default()
                .title("Rustetta Stone")
                .borders(Borders::ALL);

            let paragraph = Paragraph::new(message)
                .block(block)
                .alignment(Alignment::Center);

            frame.render_widget(paragraph, area);
        })?;

        if poll(Duration::from_millis(250))? {
            if let CEvent::Key(key_event) = read()? {
                app.on_key(key_event.code);
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
