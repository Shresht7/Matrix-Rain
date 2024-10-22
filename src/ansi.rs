use std::{slice::Iter, str::FromStr};

// ---------
// RGB COLOR
// ---------

/// Holds the RGB values for a color
#[derive(Clone, Copy, Debug)]
pub struct RGBColor(pub u8, pub u8, pub u8);

impl RGBColor {
    /// Returns the red value of the [RGBColor]
    pub fn r(&self) -> u8 {
        self.0
    }
    /// Returns the green value of the [RGBColor]
    pub fn g(&self) -> u8 {
        self.1
    }
    /// Returns the blue value of the [RGBColor]
    pub fn b(&self) -> u8 {
        self.0
    }
    /// Returns the ANSI escape code for the [RGBColor]
    pub fn ansi(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.0, self.1, self.2)
    }
    /// Returns the ANSI escape code for the background [RGBColor]
    pub fn ansi_bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.0, self.1, self.2)
    }
}

/// Parse a [RGBColor] from a tuple of (u8, u8, u8)
impl From<(u8, u8, u8)> for RGBColor {
    fn from(value: (u8, u8, u8)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

// Implement the FromStr trait for RGBColor to parse the command-line argument
impl FromStr for RGBColor {
    type Err = ParseErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            return RGBColor::from_hex_str(s);
        } else if s.contains(',') {
            return RGBColor::from_rgb_str(s);
        } else {
            return RGBColor::from_named_color(s);
        }
    }
}

impl RGBColor {
    /// Parses a hex-color into a [RGBColor] value
    fn from_hex_str(s: &str) -> Result<Self, ParseErrorKind> {
        let color = s.trim_start_matches('#');
        let r = u8::from_str_radix(&color[0..2], 16).map_err(ParseErrorKind::InvalidHexValue)?;
        let g = u8::from_str_radix(&color[2..4], 16).map_err(ParseErrorKind::InvalidHexValue)?;
        let b = u8::from_str_radix(&color[4..6], 16).map_err(ParseErrorKind::InvalidHexValue)?;
        Ok(Self(r, g, b))
    }

    /// Parses a rgb color into a [RGBColor] value
    fn from_rgb_str(s: &str) -> Result<Self, ParseErrorKind> {
        let parts: Vec<&str> = s.split(',').map(|part| part.trim()).collect();
        if parts.len() == 3 {
            if let (Ok(r), Ok(g), Ok(b)) = (parts[0].parse(), parts[1].parse(), parts[2].parse()) {
                return Ok(Self(r, g, b));
            } else {
                return Err(ParseErrorKind::InvalidFormat(s.to_string()));
            }
        } else {
            Err(ParseErrorKind::InvalidFormat(s.to_string()))
        }
    }

    /// Parses a named-color into a [RGBColor] value
    fn from_named_color(s: &str) -> Result<Self, ParseErrorKind> {
        match s.to_lowercase().as_str() {
            "black" => Ok(RGBColor(0, 0, 0)),
            "red" => Ok(RGBColor(255, 0, 0)),
            "green" => Ok(RGBColor(0, 255, 0)),
            "yellow" => Ok(RGBColor(255, 255, 0)),
            "blue" => Ok(RGBColor(0, 0, 255)),
            "magenta" => Ok(RGBColor(255, 0, 255)),
            "cyan" => Ok(RGBColor(0, 255, 255)),
            "white" => Ok(RGBColor(255, 255, 255)),
            name => Err(ParseErrorKind::UnsupportedName(name.to_string())),
        }
    }
}

/// Color string with ANSI RGB color code
pub fn rgb(s: &char, color: RGBColor) -> String {
    format!(
        "\u{001b}[38;2;{};{};{}m{}\u{001b}[0m",
        color.0,
        color.1,
        color.2,
        s.to_string()
    )
}

// ---------
// GRADIENTS
// ---------

pub struct LinearGradient {
    start: RGBColor,
    end: RGBColor,
    delta: (i16, i16, i16),
}

impl LinearGradient {
    /// Instantiate a new linear gradient
    pub fn new(start: RGBColor, end: RGBColor) -> Self {
        let delta = (
            end.r() as i16 - start.r() as i16,
            end.g() as i16 - start.g() as i16,
            end.b() as i16 - start.b() as i16,
        );
        Self { start, end, delta }
    }

    /// Interpolate between two colors. The factor has to be between 0 and 1
    pub fn interpolate(&self, factor: f32) -> RGBColor {
        assert!(
            factor >= 0.0 && factor <= 1.0,
            "The factor value must be between 0 and 1"
        );
        let r = self.start.r() as f32 + factor * self.delta.0 as f32;
        let g = self.start.g() as f32 + factor * self.delta.1 as f32;
        let b = self.start.b() as f32 + factor * self.delta.2 as f32;
        RGBColor(r.round() as u8, g.round() as u8, b.round() as u8)
    }
}

// ------
// ERRORS
// ------

/// Errors that can occur when trying to parse [RGBColor]
#[derive(Debug)]
pub enum ParseErrorKind {
    /// The provided format was invalid
    InvalidFormat(String),
    /// The color name is not supported
    UnsupportedName(String),
    /// Failed to parse hex value
    InvalidHexValue(std::num::ParseIntError),
}

impl std::fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::InvalidFormat(format) => write!(f, "Invalid color format: {format}"),
            ParseErrorKind::UnsupportedName(name) => write!(f, "Unsupported color name: {name}"),
            ParseErrorKind::InvalidHexValue(v) => write!(f, "Invalid hex value: {v}"),
        }
    }
}

impl std::error::Error for ParseErrorKind {}
