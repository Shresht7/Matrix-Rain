use clap::Parser;

use crate::helpers::colors;
use crate::symbols;

//  =============
//  CONFIGURATION
//  =============

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Config {
    /// Character Symbol Set
    #[clap(long, default_value = "Original")]
    pub mode: symbols::Symbols,

    /// Default color of the streaming entities
    #[clap(long, default_value = "0,255,70")]
    pub stream_color: colors::RGBColor,

    /// The multiplier that describes the extent of the gradient in the stream color
    #[clap(long, default_value_t = 0.33)]
    pub stream_color_gradient_factor: f32,

    /// Color of the leading entity in a stream
    #[clap(long, default_value = "200,255,200")]
    pub leading_entity_color: colors::RGBColor,

    /// The Frame-Rate to run at. The screen will rerender this many times each second.
    #[clap(long, default_value = "60")]
    pub fps: u16,

    /// Minimum number of entities per stream
    #[clap(long, default_value = "5")]
    pub stream_min_count: u16,

    /// Maximum number of entities per stream
    #[clap(long, default_value = "25")]
    pub stream_max_count: u16,

    /// Stream Spacing
    #[clap(long, default_value = "2")]
    pub stream_spacing: u16,

    /// The max number-of-seconds within which an entity randomly switches it's symbol.
    ///
    /// This adds a bit of randomness to each stream, as the entities can switch their
    /// symbol as they rain down the screen.
    #[clap(long, default_value = "1")]
    pub switch_interval: u16,
}
