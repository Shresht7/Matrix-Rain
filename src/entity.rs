//  Library
use crate::ansi;
use crate::config;
use crate::utils;
use std::str::FromStr;

// -------
// SYMBOLS
// -------

//  The character set to use for the entities
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Mode {
    Original, //  Katakana
    Binary,   //  0 or 1
    ASCII,    //  ASCII
    Braille,  //  Braille
}

// Implement the `FromStr` trait for Mode so that we can parse the command-line argument
impl FromStr for Mode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "original" => Ok(Mode::Original),
            "binary" => Ok(Mode::Binary),
            "ascii" => Ok(Mode::ASCII),
            "braille" => Ok(Mode::Braille),
            _ => Err(anyhow::Error::msg("Invalid Mode")),
        }
    }
}

//  ======
//  ENTITY
//  ======

/// Represents a single entity in the matrix-stream
pub struct Entity {
    /// x position
    pub x: f32,
    /// y position
    pub y: f32,
    /// rain-fall speed
    speed: f32,
    /// entity color
    color: ansi::RGBColor,
    /// entity symbol
    symbol: char,
    /// character set mode
    mode: Mode,
    /// frame-count since last switch
    frame_count: u16,
    /// number of frames before character switch
    switch_interval: u16,
}

impl Entity {
    /// Entity constructor
    pub fn new(x: f32, y: f32, speed: f32, config: &config::Config) -> Self {
        return Self {
            x,
            y,
            speed,
            color: config.stream_color,
            symbol: ' ',
            mode: config.mode,
            frame_count: 0,
            switch_interval: utils::random_between::<u16>(1, 60),
        };
    }

    pub fn new_leader(x: f32, y: f32, speed: f32, config: &config::Config) -> Self {
        return Self {
            x,
            y,
            speed,
            color: config.leading_entity_color,
            symbol: ' ',
            mode: config.mode,
            frame_count: 0,
            switch_interval: utils::random_between::<u16>(1, 60),
        };
    }

    /// Set Entity Symbol
    pub fn set_symbol(&mut self) {
        match self.mode {
            //  Katakana Symbols
            Mode::Original => {
                let r = utils::random_between(0x30a0, 0x30a0 + 96) as u32;
                self.symbol = std::char::from_u32(r).unwrap_or('0');
            }

            //  Binary Symbols
            Mode::Binary => {
                let r = utils::random_between(0, 2);
                self.symbol = if r == 0 { '0' } else { '1' };
            }

            //  ASCII Symbols
            Mode::ASCII => {
                let r = utils::random_between(33, 127) as u32;
                self.symbol = std::char::from_u32(r).unwrap_or('0');
            }

            //  Braille Symbols
            Mode::Braille => {
                let r = utils::random_between(0x2840, 0x2840 + 63) as u32;
                self.symbol = std::char::from_u32(r).unwrap_or('0');
            }
        }
    }

    /// Rain
    pub fn rain(&mut self) {
        self.y += self.speed
    }

    /// Render entity on screen
    pub fn render(&mut self) {
        //  Don't render if y is above screen
        if self.y < 0.0 {
            return;
        }

        //  Move cursor to position and write symbol
        ansi::cursor_move_to(self.y as u32, self.x as u32);
        print!("{}", ansi::rgb(&self.symbol, self.color));

        //  Switch symbol if frame_count exceeds switch_interval
        if self.frame_count % self.switch_interval == 0 {
            self.set_symbol()
        }
        self.frame_count += 1;
    }

    /// Cleans the last position of this entity
    pub fn clean(&self, rows: u32) {
        let last_y = self.y - self.speed;
        if last_y <= 0.0 {
            ansi::cursor_move_to(rows, self.x as u32)
        } else {
            ansi::cursor_move_to(last_y as u32, self.x as u32);
        }
        match self.mode {
            Mode::Original => print!("  "),
            Mode::ASCII => print!(" "),
            Mode::Binary => print!(" "),
            Mode::Braille => print!(" "),
        }
    }
}
