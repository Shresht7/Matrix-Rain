// Library
use std::result::Result;
use std::str::FromStr;

// RGB COLOR
// ---------

/// ANSI RGB Color Tuple Struct (Red, Green, Blue)
#[derive(Clone, Copy, Debug)]
pub struct RGBColor(pub u8, pub u8, pub u8);

// Implement the FromStr trait for RGBColor to parse the command-line argument
impl FromStr for RGBColor {
    type Err = anyhow::Error;

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
            return Err(anyhow::Error::msg("RGBColor must be in the format 'r,g,b'"));
        }

        return Ok(RGBColor(
            parts[0].parse()?,
            parts[1].parse()?,
            parts[2].parse()?,
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

pub fn parse_hex_color(color: &str) -> Result<RGBColor, anyhow::Error> {
    let color = color.trim_start_matches('#');
    let r = u8::from_str_radix(&color[0..2], 16)?;
    let g = u8::from_str_radix(&color[2..4], 16)?;
    let b = u8::from_str_radix(&color[4..6], 16)?;
    Ok(RGBColor(r, g, b))
}

pub fn parse_named_color(color: &str) -> Result<RGBColor, anyhow::Error> {
    match color.to_lowercase().as_str() {
        "black" => Ok(RGBColor(0, 0, 0)),
        "red" => Ok(RGBColor(255, 0, 0)),
        "green" => Ok(RGBColor(0, 255, 0)),
        "yellow" => Ok(RGBColor(255, 255, 0)),
        "blue" => Ok(RGBColor(0, 0, 255)),
        "magenta" => Ok(RGBColor(255, 0, 255)),
        "cyan" => Ok(RGBColor(0, 255, 255)),
        "white" => Ok(RGBColor(255, 255, 255)),
        _ => Err(anyhow::Error::msg("Invalid color name")),
    }
}
