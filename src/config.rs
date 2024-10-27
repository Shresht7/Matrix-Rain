use std::str::FromStr;
use std::string::ParseError;

use clap::Parser;

use crate::helpers::colors;
use crate::symbols;

//  =============
//  CONFIGURATION
//  =============

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Config {
    /// The character symbol set to use
    ///
    /// Valid Options:
    /// - "Original" | "Normal" | "Katakana" -> Original Katakana Symbols (e.g. ア, カ, サ, ナ)
    /// - "Binary" | "Bin"                   -> 0s and 1s
    /// - "Decimal" | "Numbers" | "Digits"   -> Decimal numbers from 0 to 9
    /// - "Math" | "Maths" | "Mathematics"   -> Mathematical Symbols (e.g. ∐, ∑, ≠, →)
    /// - "ASCII" | "Text" | "English"       -> ASCII Characters (from '!' to '~', including A-Z, a-z, 0-9 etc.)
    /// - "Braille" | "Dots"                 -> Braille patterns (e.g  ⠇, ⠾, ⣿)
    /// - "Emoji" | "Cursed"                 -> Emojis
    /// - "abc123"                           -> Custom character set consisting of "a", "b", "c", "1", "2" and "3"
    ///
    /// The options can be in all lower-case.
    #[clap(long, default_value = "Original", verbatim_doc_comment)]
    pub mode: symbols::Symbols,

    /// The color of the streaming entities
    ///
    /// The color gradually fades across the stream as given by the --stream-color-gradient-factor.
    ///
    /// Defaults to the original matrix green color.
    #[clap(long, default_value = "0,255,70")]
    pub stream_color: colors::RGBColor,

    /// The multiplier that describes the extent of the gradient in the stream color
    ///
    /// The main color for the gradient is described by the --stream-color
    #[clap(long, default_value_t = 0.33)]
    pub stream_color_gradient_factor: f32,

    /// Color of the leading entity in a stream
    ///
    /// The leading entity in a stream can have a different color from the rest.
    #[clap(long, default_value = "200,255,200")]
    pub leading_entity_color: colors::RGBColor,

    /// Leaves the trail intact
    ///
    /// As the streams pass-by, the leave behind a visible trail. Use this option if you want even more characters on screen.
    #[clap(long)]
    pub leave_trail: bool,

    /// The Frame-Rate to run at. The screen will rerender this many times each second.
    #[clap(long, default_value_t = 60)]
    pub fps: u16,

    /// Minimum number of entities per stream
    #[clap(long, default_value_t = 5)]
    pub stream_min_count: u16,

    /// Maximum number of entities per stream
    #[clap(long, default_value_t = 25)]
    pub stream_max_count: u16,

    /// The spacing between the streams.
    ///
    /// By default, a stream will be created every 2nd column.
    #[clap(long, default_value_t = 2)]
    pub stream_spacing: u16,

    /// The max number-of-seconds within which an entity randomly switches it's symbol.
    ///
    /// This adds a bit of randomness to each stream, as the entities can switch their
    /// symbol as they rain down the screen.
    #[clap(long, default_value_t = 1)]
    pub switch_interval: u16,

    /// The direction of motion for the stream particles
    #[clap(long, default_value = "down")]
    pub direction: Direction,
}

#[derive(Default, Clone, Debug)]
pub enum Direction {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" | "vertical-reverse" => Ok(Self::Up),
            "down" | "vertical" => Ok(Self::Down),
            "left" | "horizontal" => Ok(Self::Left),
            "right" | "horizontal-reverse" => Ok(Self::Right),
            _ => Ok(Self::default()),
        }
    }
}
