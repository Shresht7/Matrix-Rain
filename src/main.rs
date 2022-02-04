mod entity;
mod stream;
mod utils;

use stream::Stream;
use utils::RGBColor;

//  =============
//  CONFIGURATION
//  =============

const ROWS: u8 = 80;
const COLUMNS: u8 = 120;
// const COLUMN_SPACING: u8 = 3;

const STREAM_MINCOUNT: u8 = 3;
const STREAM_MAXCOUNT: u8 = 7;
const STREAM_COLOR: RGBColor = RGBColor(0, 255, 70);

// const FPS: u8 = 15;

// const MODE

//  ====
//  MAIN
//  ====

fn main() {
    //  Instantiate streams vector
    let mut streams: Vec<Stream> = Vec::new();

    //  Generate stream entities
    for c in 0..COLUMNS {
        let mut stream = Stream::new(STREAM_MINCOUNT, STREAM_MAXCOUNT, STREAM_COLOR);
        let random_speed_offset = utils::random_between(0, 10);
        stream.generate_entities(c, random_speed_offset);
        streams.push(stream);
    }

    //  Render streams
    utils::clear_screen();
    for stream in streams.iter_mut() {
        stream.render(ROWS);
    }
}
