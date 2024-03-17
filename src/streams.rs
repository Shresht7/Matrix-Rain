//  Library
use crate::config;
use crate::stream::Stream;
use crate::utils;

//  =======
//  STREAMS
//  =======

pub struct Streams {
    streams: Vec<Stream>,
}

impl Streams {
    pub fn new(count: u16, config: &config::Config) -> Self {
        //  Instantiate streams vector
        let mut streams: Vec<Stream> = Vec::new();

        //  Generate stream entities
        for c in 0..count {
            if c % config.stream_spacing != 0 {
                continue;
            }
            //  Generate a stream
            let height_offset = utils::random_between(-50, 0);
            let stream = Stream::new(c as i32, height_offset, config);

            //  Add stream to streams collection
            streams.push(stream);
        }

        Self { streams }
    }

    /// Render all streams
    pub fn render(&mut self, rows: u16, config: &config::Config) {
        for stream in self.streams.iter_mut() {
            stream.render(rows as i32, config);
        }
    }
}
