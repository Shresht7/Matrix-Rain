use std::str::FromStr;

// ---------
// RGB COLOR
// ---------

/// Holds the RGB values for a color
#[derive(Clone, Copy, Debug)]
pub struct RGBColor(pub u8, pub u8, pub u8);

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
