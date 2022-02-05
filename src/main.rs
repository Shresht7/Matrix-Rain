mod entity;
mod stream;
mod utils;

use stream::Stream;
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

    //  Instantiate streams vector
    let mut streams: Vec<Stream> = Vec::new();

    //  Generate stream entities
    for c in 0..=columns {
        //  Generate a stream
        let mut stream = Stream::new(0, 10, STREAM_COLOR);
        let random_speed = utils::random_between(5, 20);
        let height_offset = utils::random_between(-50, 1);
        stream.generate_entities(c as i32, height_offset, random_speed, MODE);

        //  Add stream to streams collection
        streams.push(stream);
    }

    //  Render streams
    utils::clear_screen();
    loop {
        for stream in streams.iter_mut() {
            stream.render(rows as i32);
        }
        std::thread::sleep(std::time::Duration::from_millis(1000 / FPS));
    }
}
