use crate::stream::Stream;
use crate::utils;

//  =======
//  STREAMS
//  =======

pub struct Streams {
    streams: Vec<Stream>,
}

impl Streams {
    pub fn new(count: u16, mode: utils::Mode, color: utils::RGBColor) -> Self {
        //  Instantiate streams vector
        let mut streams: Vec<Stream> = Vec::new();

        //  Generate stream entities
        for c in 0..=count {
            //  Generate a stream
            let mut stream = Stream::new(0, 10, color);
            let random_speed = utils::random_between(5, 20);
            let height_offset = utils::random_between(-50, 1);
            stream.generate_entities(c as i32, height_offset, random_speed, mode);

            //  Add stream to streams collection
            streams.push(stream);
        }

        Self { streams }
    }

    /// Render all streams
    pub fn render(&mut self, rows: u16) {
        for stream in self.streams.iter_mut() {
            stream.render(rows as i32);
        }
    }
}
