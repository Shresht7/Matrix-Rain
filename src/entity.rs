//  Library
use crate::config;
use crate::utils;

//  ======
//  ENTITY
//  ======

/// Represents a single entity in the matrix-stream
pub struct Entity {
    /// x position
    x: f32,
    /// y position
    y: f32,
    /// rain-fall speed
    speed: f32,
    /// entity color
    color: utils::RGBColor,
    /// entity symbol
    symbol: char,
    /// character set mode
    mode: utils::Mode,
    /// frame-count since last switch
    frame_count: u16,
    /// number of frames before character switch
    switch_interval: u16,
}

impl Entity {
    /// Entity constructor
    pub fn new(
        x: f32,
        y: f32,
        speed: f32,
        color: utils::RGBColor,
        mode: utils::Mode,
        is_first: bool,
    ) -> Self {
        return Self {
            x,
            y,
            speed,
            color: if is_first {
                config::LEADING_ENTITY_COLOR
            } else {
                color
            },
            symbol: ' ',
            mode,
            frame_count: 0,
            switch_interval: utils::random_between::<u16>(1, 20),
        };
    }

    /// Set Entity Symbol
    pub fn set_symbol(&mut self) {
        match self.mode {
            //  Katakana Symbols
            utils::Mode::Original => {
                let r = utils::random_between(0x30a0, 0x30a0 + 96) as u32;
                self.symbol = std::char::from_u32(r).unwrap_or('0');
            }

            //  Binary Symbols
            utils::Mode::Binary => {
                let r = utils::random_between(0, 2);
                self.symbol = if r == 0 { '0' } else { '1' };
            }

            //  ASCII Symbols
            utils::Mode::ASCII => {
                let r = utils::random_between(33, 127) as u32;
                self.symbol = std::char::from_u32(r).unwrap_or('0');
            }

            //  Braille Symbols
            utils::Mode::Braille => {
                let r = utils::random_between(0x2840, 0x2840 + 63) as u32;
                self.symbol = std::char::from_u32(r).unwrap_or('0');
            }
        }
    }

    /// Rain
    pub fn rain(&mut self, rows: i32) {
        self.y = if self.y > (rows as f32) {
            //  if y position is beyond max rows...
            utils::random_between::<f32>(-100.0, 0.0) //  ... reset it's position above the screen.
        } else {
            //  else...
            self.y + self.speed //  ...keep raining
        }
    }

    /// Render entity on screen
    pub fn render(&mut self) {
        //  Don't render if y is above screen
        if self.y < 0.0 {
            return;
        }

        //  Move cursor to position and write symbol
        utils::cursor_move_to(self.y as u32, self.x as u32);
        print!("{}", utils::rgb(&self.symbol, self.color));

        //  Switch symbol if frame_count exceeds switch_interval
        if self.frame_count % self.switch_interval == 0 {
            self.set_symbol()
        }
        self.frame_count += 1;
    }

    /// Cleans the last position of this entity
    pub fn clean(&self) {
        utils::cursor_move_to((self.y - self.speed) as u32, self.x as u32);
        match self.mode {
            utils::Mode::Original => print!("  "),
            utils::Mode::ASCII => print!(" "),
            utils::Mode::Binary => print!(" "),
            utils::Mode::Braille => print!(" "),
        }
    }
}
