use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// Instructs the main loop what to do
pub enum Action {
    /// Do nothing
    None,
    /// Exit the loop
    Exit,
}

/// Processes and handles [crossterm events](crossterm::event). Returns an [`Action`] as a response.
pub fn handle_events() -> std::io::Result<Action> {
    match crossterm::event::read()? {
        crossterm::event::Event::Key(event) if event.kind == KeyEventKind::Press => {
            return Ok(handle_key_event(event))
        }
        _ => (),
    }
    Ok(Action::None)
}

/// Handles keyboard events and returns an [`Action`] based on the key pressed.
fn handle_key_event(event: KeyEvent) -> Action {
    match event {
        // Check if 'q', `Esc` or `Ctrl+C` has been pressed ...
        KeyEvent {
            code: KeyCode::Char('q'),
            ..
        }
        | KeyEvent {
            code: KeyCode::Esc, ..
        }
        | KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Action::Exit, // ... then respond with exit.
        _ => Action::None, // ... otherwise, respond with none.
    }
}
