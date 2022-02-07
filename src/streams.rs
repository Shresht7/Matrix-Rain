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
            let mut stream = Stream::new(3, 7, color);
            let height_offset = utils::random_between(-50.0, 0.0);
            stream.generate_entities(c as f32, height_offset, mode);

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
