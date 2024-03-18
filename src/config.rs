//  Library
use crate::ansi;
use crate::entity;
use clap::Parser;

//  =============
//  CONFIGURATION
//  =============

#[derive(Parser, Debug)]
#[command(version, about, long_about = "None")]
pub struct Config {
    /// Character Symbol Set
    #[clap(long, default_value = "Original")]
    pub mode: entity::Symbols,

    /// Default color of the streaming entities
    #[clap(long, default_value = "0,255,70")]
    pub stream_color: ansi::RGBColor,

    /// Color of the leading entity in a stream
    #[clap(long, default_value = "200,255,200")]
    pub leading_entity_color: ansi::RGBColor,

    /// Frame-Rate
    #[clap(long, default_value = "60")]
    pub fps: u64,

    /// Minimum number of entities per stream
    #[clap(long, default_value = "5")]
    pub stream_min_count: u16,

    /// Maximum number of entities per stream
    #[clap(long, default_value = "25")]
    pub stream_max_count: u16,

    /// Stream Spacing
    #[clap(long, default_value = "2")]
    pub stream_spacing: u16,
}
