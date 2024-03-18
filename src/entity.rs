//  Library
use crate::ansi;
use crate::config;
use crate::symbols;
use crate::utils;

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
    mode: symbols::Symbols,
    /// frame-count since last switch
    frame_count: u16,
    /// number of frames before character switch
    switch_interval: u16,
}

impl Entity {
    /// Entity constructor
    pub fn new(x: f32, y: f32, speed: f32, color: ansi::RGBColor, config: &config::Config) -> Self {
        return Self {
            x,
            y,
            speed,
            color,
            symbol: ' ',
            mode: config.mode.clone(),
            frame_count: 0,
            switch_interval: utils::random_between::<u16>(1, 60),
        };
    }

    /// Set Entity Symbol
    pub fn set_symbol(&mut self) {
        self.symbol = self.mode.get_random();
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
}
