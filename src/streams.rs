//  Library
use crate::config;
use crate::stream::Stream;
use crate::utils;

//  =======
//  STREAMS
//  =======

pub struct Streams {
    rows: u16,
    columns: u16,
    streams: Vec<Stream>,
}

impl Streams {
    pub fn new(rows: u16, columns: u16, config: &config::Config) -> Self {
        // Instantiate Self
        let mut ret = Self {
            rows,
            columns,
            streams: Vec::new(),
        };

        //  Generate stream entities
        for c in 0..ret.columns {
            if c % config.stream_spacing != 0 {
                continue;
            }
            //  Generate a stream
            let height_offset = utils::random_between(-50, 0);
            let stream = Stream::new(c as i32, height_offset, config);

            //  Add stream to streams collection
            ret.streams.push(stream);
        }

        return ret;
    }

    /// Render all streams
    pub fn render(&mut self, config: &config::Config) {
        for stream in self.streams.iter_mut() {
            stream.render(self.rows as i32, config);
        }
    }
}
