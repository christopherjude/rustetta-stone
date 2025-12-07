use crossterm::event::KeyCode;

pub struct App {
    pub running: bool,
    pub last_key: Option<KeyCode>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            last_key: None,
        }
    }

    pub fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => {
                self.running = false;
            }

            other => {
                self.last_key = Some(other);
            }
        }
    }
}
