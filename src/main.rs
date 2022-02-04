mod entity;
mod stream;
mod utils;

use stream::Stream;
use utils::RGBColor;

//  =============
//  CONFIGURATION
//  =============

const ROWS: i32 = 80;
const COLUMNS: i32 = 120;
// const COLUMN_SPACING: i32 = 3;

const STREAM_MINCOUNT: i32 = 3;
const STREAM_MAXCOUNT: i32 = 7;
const STREAM_COLOR: RGBColor = RGBColor(0, 255, 70);

const FPS: u64 = 15;

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
        let random_speed = utils::random_between(6, 20);
        stream.generate_entities(c, 0, random_speed);
        streams.push(stream);
    }

    //  Render streams
    loop {
        utils::clear_screen();
        for stream in streams.iter_mut() {
            stream.render(ROWS);
        }
        std::thread::sleep(std::time::Duration::from_millis(1000 / FPS));
    }
}
