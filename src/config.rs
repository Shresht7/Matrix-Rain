//  Library
use crate::ansi;
use crate::utils;

//  =============
//  CONFIGURATION
//  =============

/// Color of the leading entity in a stream
pub const LEADING_ENTITY_COLOR: ansi::RGBColor = ansi::RGBColor(200, 255, 200);

/// Minimum and Maximum number of entities per stream
pub const STREAM_MIN_COUNT: u16 = 5;
pub const STREAM_MAX_COUNT: u16 = 25;

/// Default color of the streaming entities
pub const STREAM_COLOR: ansi::RGBColor = ansi::RGBColor(0, 255, 70);

/// Stream Spacing
pub const STREAM_SPACING: u16 = 2;

/// Frame-Rate
pub const FPS: u64 = 15;

/// Character Symbol Set
pub const MODE: utils::Mode = utils::Mode::Original;
