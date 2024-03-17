mod config;
mod entity;
mod stream;
mod streams;
mod utils;

use std::time::Duration;

use streams::Streams;

use crossterm::{event, terminal};

//  ====
//  MAIN
//  ====

fn main() {
    //  Get Terminal Window Size
    let (columns, rows) = terminal::size().unwrap_or((40, 120));

    //  Instantiate streams
    let mut streams = Streams::new(
        columns,
        config::STREAM_MIN_COUNT,
        config::STREAM_MAX_COUNT,
        config::MODE,
        config::STREAM_COLOR,
    );

    // Switch to the alternate screen buffer
    terminal::enable_raw_mode().unwrap();

    //  Render the Matrix-Rain on screen
    utils::clear_screen();
    loop {
        // Check if 'q' or Ctrl+C has been pressed
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let event::Event::Key(event) = event::read().unwrap() {
                if event.code == event::KeyCode::Char('q')
                    || (event.modifiers == event::KeyModifiers::CONTROL
                        && event.code == event::KeyCode::Char('c'))
                {
                    break;
                }
            }
        }

        //  Render each stream
        streams.render(rows);

        //  Sleep for 1/FPS seconds
        std::thread::sleep(Duration::from_millis(1000 / config::FPS));
    }

    //  Clear screen and disable raw mode before exiting
    utils::clear_screen();
    utils::cursor_move_to(0, 0);
    terminal::disable_raw_mode().unwrap();
}
