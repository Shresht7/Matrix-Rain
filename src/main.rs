mod config;
mod entity;
mod stream;
mod streams;
mod utils;

use streams::Streams;

use crossterm::terminal;

//  ====
//  MAIN
//  ====

fn main() {
    //  Get Terminal Window Size
    let (columns, rows) = terminal::size().unwrap_or((40, 120));

    //  Render the Matrix-Rain on screen
    render(columns, rows);
}

/// Render the Matrix-Rain on the screen
fn render(columns: u16, rows: u16) {
    //  Instantiate streams
    let mut streams = Streams::new(columns, config::MODE, config::STREAM_COLOR);

    //  Render Matrix-Rain
    utils::clear_screen(); //  Clear the screen
    loop {
        streams.render(rows); //  Render each stream
        std::thread::sleep(std::time::Duration::from_millis(1000 / config::FPS));
        //  Sleep for 1/FPS seconds
    }
}
