mod ansi;
mod config;
mod entity;
mod matrix;
mod stream;
mod symbols;
mod utils;

use clap::Parser;
use crossterm::{event, terminal};
use std::time::Duration;

//  ====
//  MAIN
//  ====

fn main() {
    //  Parse command-line arguments as the configuration
    let config = config::Config::parse();

    //  Get Terminal Window Size to determine the number of rows and columns
    let (columns, rows) = terminal::size().unwrap_or((40, 120));

    //  Instantiate streams
    let mut matrix = matrix::Matrix::new(rows, columns, &config);

    // Switch to the alternate screen buffer
    terminal::enable_raw_mode().unwrap();

    //  Render the Matrix-Rain on screen
    ansi::clear_screen();
    ansi::hide_cursor();
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
        matrix.render(&config);

        //  Sleep for 1/FPS seconds
        std::thread::sleep(Duration::from_millis(1000 / config.fps));
    }

    //  Clear screen and disable raw mode before exiting
    ansi::clear_screen();
    ansi::cursor_move_to(0, 0);
    ansi::show_cursor();
    terminal::disable_raw_mode().unwrap();
}
