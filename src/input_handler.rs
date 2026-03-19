use crossterm::event::{self, Event, KeyCode};

pub fn handle_input() -> Option<char> {
    if event::poll(std::time::Duration::from_millis(100)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Up | KeyCode::Down => return Some('x'),
                KeyCode::Left | KeyCode::Right => return Some('y'),
                KeyCode::Char('e') | KeyCode::Char('r') => return Some('z'),
                KeyCode::Char('q') => std::process::exit(0),
                _ => {}
            }
        }
    }
    None
}
