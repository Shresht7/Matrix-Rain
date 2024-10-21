use super::{config, symbols, utils};

mod entity;
mod stream;

use stream::Stream;

//  ======
//  MATRIX
//  ======

/// The structure that represents the Matrix
pub struct Matrix {
    // Number of rows
    rows: u16,
    // Number of columns
    columns: u16,
    // Collection of streams
    streams: Vec<Stream>,
}

impl Matrix {
    /// Construct a new Matrix instance
    pub fn new(rows: u16, columns: u16, config: &config::Config) -> Self {
        // Instantiate Self
        let mut ret = Self {
            rows,
            columns,
            streams: Vec::new(),
        };

        //  Generate a stream for each column
        for c in 0..ret.columns {
            // Space out the streams, if specified
            if c % config.stream_spacing != 0 {
                continue;
            }
            //  Generate the stream
            let height_offset = utils::random_number_between(-50, 0);
            let stream = Stream::new(c as f32, height_offset as f32, config);

            //  Add stream to vector collection
            ret.streams.push(stream);
        }

        // Return the instance
        return ret;
    }

    /// Render the Matrix
    pub fn render(&mut self, config: &config::Config) {
        for stream in self.streams.iter_mut() {
            stream.render(self.rows as i32, config);
        }
    }
}
