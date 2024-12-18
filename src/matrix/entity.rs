use crossterm::cursor;
use crossterm::style::Print;
use crossterm::QueueableCommand;

use crate::config;
use crate::helpers::{colors, utils};

use super::symbols;

//  ======
//  ENTITY
//  ======

/// Represents a single entity in the [matrix-stream](super::Stream).
/// Holds information about the position, speed and character symbol.
pub struct Entity {
    /// The x-position
    pub x: f32,
    /// The y-position
    pub y: f32,
    /// The speed along the x-axis
    speed_x: f32,
    /// The speed along the y-axis
    speed_y: f32,

    /// The symbol the entity represents
    symbol: char,
    /// The color of the symbol
    color: colors::RGBColor,
    /// The character set to use for the symbols
    mode: symbols::Symbols,
    /// The frame-count since last symbol switch.
    /// When this number reaches the `switch_interval`, the character symbol
    /// is swapped for another one, chosen randomly, from the symbol character set.
    frame_count: u16,
    /// The number of frames before a symbol switch.
    /// When the `frame_count` reaches this value, the symbol is swapped
    /// for another one, chosen randomly, from the symbol character set.
    switch_interval: u16,
}

impl Entity {
    /// Constructs a new matrix [Entity]
    pub fn new(
        x: f32,
        y: f32,
        speed_x: f32,
        speed_y: f32,
        color: colors::RGBColor,
        config: &config::Config,
    ) -> Self {
        Self {
            x,
            y,
            speed_x,
            speed_y,
            color,
            symbol: ' ',
            mode: config.mode.clone(),
            frame_count: 0,
            switch_interval: utils::random_between::<u16>(1, config.switch_interval * config.fps),
        }
    }

    /// Rain. Updates the position of the [Entity] using the rain speed.
    pub fn rain(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;
    }

    /// Updates the [Entity] symbol by picking one randomly from the symbol set
    pub fn set_symbol(&mut self) {
        self.symbol = self.mode.get_random();
    }

    /// If the `frame_count` has exceeded `switch_interval` switch the [Entity] symbol to
    /// another one from the character set.
    fn switch_symbol(&mut self) {
        if self.frame_count % self.switch_interval == 0 {
            self.set_symbol();
        }
        self.frame_count += 1;
    }

    /// Render Entity on screen
    pub fn render(
        &mut self,
        rows: i32,
        columns: i32,
        stdout: &mut std::io::Stdout,
    ) -> std::io::Result<()> {
        // Don't render if the entity is off-screen
        if self.x < 0.0 || self.x >= columns as f32 || self.y < 0.0 || self.y >= rows as f32 {
            return Ok(());
        }

        // Move cursor to position and write symbol
        stdout
            .queue(cursor::MoveTo(self.x as u16, self.y as u16))?
            .queue(Print(utils::ansi_rgb(&self.symbol, self.color)))?;

        // Switch symbol if `frame_count` exceeds `switch_interval`
        if self.switch_interval != 0 {
            self.switch_symbol();
        }

        Ok(())
    }
}
