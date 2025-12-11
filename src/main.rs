mod app;
mod events;
mod ui;

use crate::app::App;
use crossterm::{
    event::{poll, read, Event as CEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    while app.running {
        terminal.draw(|frame| {
            ui::draw(frame, &app);
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
