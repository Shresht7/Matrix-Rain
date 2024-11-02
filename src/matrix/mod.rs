use std::io::Write;

use super::{
    config,
    helpers::{direction::Direction, utils},
    symbols,
};

mod entity;
mod stream;

use crossterm::{
    cursor::{self, MoveToNextLine},
    style::Print,
    QueueableCommand,
};
use stream::Stream;

//  ======
//  MATRIX
//  ======

/// The structure that represents the Matrix
pub struct Matrix {
    /// Number of rows
    rows: u16,
    /// Number of columns
    columns: u16,

    /// Collection of matrix [entity](entity::Entity) streams
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

        //  Determine the count of streams to generate
        let count = match config.direction {
            Direction::Up | Direction::Down => ret.columns,
            Direction::Right | Direction::Left => ret.rows,
            Direction::DiagonalLeft
            | Direction::DiagonalLeftReverse
            | Direction::DiagonalRight
            | Direction::DiagonalRightReverse => ret.columns * 2,
        };

        // Generate the Matrix Streams
        for c in 0..count {
            // Space out the streams, if specified in the configuration
            if c % config.stream_spacing != 0 {
                continue;
            }

            // Determine the starting x and y positions based on the direction of flow
            let (x, y) = match config.direction {
                Direction::Down => {
                    let offset = utils::random_between(-50, 0);
                    (c as f32, offset as f32)
                }
                Direction::Up => {
                    let offset = utils::random_between(ret.rows, ret.rows + 50);
                    (c as f32, offset as f32)
                }
                Direction::Right => {
                    let offset = utils::random_between(-50, 0);
                    (offset as f32, c as f32)
                }
                Direction::Left => {
                    let offset = utils::random_between(ret.columns, ret.columns + 50);
                    (offset as f32, c as f32)
                }
                Direction::DiagonalLeft | Direction::DiagonalRight => {
                    let x_offset = c as f32 - (ret.columns as f32 / 2.0);
                    let y_offset = utils::random_between(-50, 0);
                    (x_offset, y_offset as f32)
                }
                Direction::DiagonalLeftReverse | Direction::DiagonalRightReverse => {
                    let x_offset = c as f32 - (ret.columns as f32 / 2.0);
                    let y_offset = utils::random_between(ret.rows, ret.rows + 50);
                    (x_offset, y_offset as f32)
                }
            };

            // Instantiate a Stream
            let stream = Stream::new(x, y, config);

            //  Add stream to vector collection
            ret.streams.push(stream);
        }

        // Return the instance
        return ret;
    }

    /// The setup function is called once before the draw loop starts
    pub fn setup(
        &mut self,
        config: &config::Config,
        stdout: &mut std::io::Stdout,
    ) -> std::io::Result<()> {
        // Pre-populate the matrix background with random symbols if the `--leave_trail` option was set
        if config.leave_trail {
            self.populate_background(config, stdout)?;
        }
        Ok(())
    }

    /// Render the Matrix
    pub fn render(
        &mut self,
        config: &config::Config,
        stdout: &mut std::io::Stdout,
    ) -> std::io::Result<()> {
        for stream in self.streams.iter_mut() {
            stream.render(self.rows as i32, self.columns as i32, config, stdout)?;
        }
        stdout.flush()?;
        Ok(())
    }

    /// Pre-populate the background with faint symbols before the first render
    pub fn populate_background(
        &mut self,
        config: &config::Config,
        stdout: &mut std::io::Stdout,
    ) -> std::io::Result<()> {
        // Determine the faint color of the trailing end of streams
        let clr = config.stream_color * config.stream_color_gradient_factor;
        // Go to the home position and set the RGB ANSI code
        stdout.queue(cursor::MoveTo(0, 0))?.queue(Print(format!(
            "\x1b[38;2;{};{};{}m",
            clr.r(),
            clr.g(),
            clr.b()
        )))?;
        // Iterate over each row and pre-render random faint symbols
        for _ in 0..self.rows {
            let random_symbol_str =
                String::from_iter((0..self.columns).map(|_| config.mode.get_random()));
            stdout
                .queue(Print(random_symbol_str))?
                .queue(MoveToNextLine(1))?;
        }
        // Reset the ANSI color codes
        stdout.queue(Print("\x1b[0m"))?.flush()?;
        Ok(())
    }
}
