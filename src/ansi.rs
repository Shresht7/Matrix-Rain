use std::str::FromStr;

// ---------
// RGB COLOR
// ---------

/// ANSI RGB Color Tuple Struct (Red, Green, Blue)
#[derive(Clone, Copy, Debug)]
pub struct RGBColor(pub u8, pub u8, pub u8);

// Implement the FromStr trait for RGBColor to parse the command-line argument
impl FromStr for RGBColor {
    type Err = ParseErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() == 1 {
            if parts[0].starts_with('#') {
                return parse_hex_color(parts[0]);
            } else {
                return parse_named_color(parts[0]);
            }
        }

        if parts.len() != 3 {
            return Err(ParseErrorKind::InvalidFormat(s.to_string()));
        }

        return Ok(RGBColor(
            parts[0].parse().map_err(ParseErrorKind::InvalidHexValue)?,
            parts[1].parse().map_err(ParseErrorKind::InvalidHexValue)?,
            parts[2].parse().map_err(ParseErrorKind::InvalidHexValue)?,
        ));
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

// HELPER FUNCTIONS
// ----------------

/// Parses a hex-color (#rrggbb format) into a [RGBColor] value
pub fn parse_hex_color(color: &str) -> Result<RGBColor, ParseErrorKind> {
    let color = color.trim_start_matches('#');
    let r = u8::from_str_radix(&color[0..2], 16).map_err(ParseErrorKind::InvalidHexValue)?;
    let g = u8::from_str_radix(&color[2..4], 16).map_err(ParseErrorKind::InvalidHexValue)?;
    let b = u8::from_str_radix(&color[4..6], 16).map_err(ParseErrorKind::InvalidHexValue)?;
    Ok(RGBColor(r, g, b))
}

/// Parses a named-color into a [RGBColor] value
pub fn parse_named_color(color: &str) -> Result<RGBColor, ParseErrorKind> {
    match color.to_lowercase().as_str() {
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
