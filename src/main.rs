mod entity;
mod stream;
mod utils;

use stream::Stream;
use utils::RGBColor;

//  =============
//  CONFIGURATION
//  =============

const ROWS: i32 = 30;
const COLUMNS: i32 = 120;
const COLUMN_SPACING: i32 = 3;

const STREAM_MINCOUNT: i32 = 5;
const STREAM_MAXCOUNT: i32 = 10;
const STREAM_COLOR: RGBColor = RGBColor(0, 255, 70);

const FPS: u64 = 60;

const MODE: utils::Mode = utils::Mode::Binary;

//  ====
//  MAIN
//  ====

fn main() {
    //  Instantiate streams vector
    let mut streams: Vec<Stream> = Vec::new();

    //  Generate stream entities
    for c in 0..COLUMNS {
        if c % COLUMN_SPACING != 0 {
            continue;
        }
        let mut stream = Stream::new(STREAM_MINCOUNT, STREAM_MAXCOUNT, STREAM_COLOR);
        let random_speed = utils::random_between(5, 20);
        let height_offset = utils::random_between(-50, 1);
        stream.generate_entities(c, height_offset, random_speed, MODE);
        streams.push(stream);
    }

    //  Render streams
    utils::clear_screen();
    loop {
        for stream in streams.iter_mut() {
            stream.render(ROWS);
        }
        std::thread::sleep(std::time::Duration::from_millis(1000 / FPS));
    }
}
