// Library
use std::result::Result;
use std::str::FromStr;

// ====
// ANSI
// ====

/// Clear the entire screen
pub fn clear_screen() {
    print!("\u{001b}[2J")
}

/// Move cursor to position
pub fn cursor_move_to(r: u32, c: u32) {
    print!("\u{001b}[{};{}H", r, c);
}

// RGB COLOR
// ---------

/// ANSI RGB Color Tuple Struct (Red, Green, Blue)
#[derive(Clone, Copy, Debug)]
pub struct RGBColor(pub i32, pub i32, pub i32);

// Implement the FromStr trait for RGBColor to parse the command-line argument
impl FromStr for RGBColor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 3 {
            return Err(anyhow::Error::msg("RGBColor must be in the format 'r,g,b'"));
        }

        Ok(RGBColor(
            parts[0].parse::<i32>()?,
            parts[1].parse::<i32>()?,
            parts[2].parse::<i32>()?,
        ))
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
