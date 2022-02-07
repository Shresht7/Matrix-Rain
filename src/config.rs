//  Library
use crate::utils;

//  =============
//  CONFIGURATION
//  =============

/// Color of the leading entity in a stream
pub const LEADING_ENTITY_COLOR: utils::RGBColor = utils::RGBColor(200, 255, 200);

/// Minimum and Maximum number of entities per stream
pub const STREAM_MIN_COUNT: u16 = 3;
pub const STREAM_MAX_COUNT: u16 = 15;

/// Default color of the streaming entities
pub const STREAM_COLOR: utils::RGBColor = utils::RGBColor(0, 255, 70);

/// Frame-Rate
pub const FPS: u64 = 5;

/// Character Symbol Set
pub const MODE: utils::Mode = utils::Mode::ASCII;
