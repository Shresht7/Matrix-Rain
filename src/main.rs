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

    //  Instantiate streams
    let mut streams = Streams::new(
        columns,
        config::STREAM_MIN_COUNT,
        config::STREAM_MAX_COUNT,
        config::MODE,
        config::STREAM_COLOR
    );

    //  Render the Matrix-Rain on screen
    utils::clear_screen(); //  Clear the screen
    loop {
        streams.render(rows); //  Render each stream
        std::thread::sleep(std::time::Duration::from_millis(1000 / config::FPS));   //  Sleep for 1/FPS seconds
    }
}
