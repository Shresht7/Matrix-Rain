mod entity;
mod stream;
mod streams;
mod utils;

use streams::Streams;
use utils::RGBColor;

use crossterm::terminal;

//  =============
//  CONFIGURATION
//  =============

/// Stream Color
const STREAM_COLOR: RGBColor = RGBColor(0, 255, 70);

/// Frames per second
const FPS: u64 = 60;

/// Character Symbol Set
const MODE: utils::Mode = utils::Mode::Binary;

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
    let mut streams = Streams::new(columns, MODE, STREAM_COLOR);

    //  Render Matrix-Rain
    utils::clear_screen(); //  Clear the screen
    loop {
        streams.render(rows); //  Render each stream
        std::thread::sleep(std::time::Duration::from_millis(1000 / FPS)); //  Sleep for 1/FPS seconds
    }
}
