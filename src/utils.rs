use rand::Rng;

//  =========
//  UTILITIES
//  =========

/// ANSI RGB Color
#[derive(Debug, Copy, Clone)]
pub struct RGBColor(pub u8, pub u8, pub u8);

/// Generate a random number between min and max
pub fn random_between(min: u8, max: u8) -> u8 {
    rand::thread_rng().gen_range(min..max)
}

/// Clear the entire screen
pub fn clear_screen() {
    print!("\u{001b}[2J")
}

/// Move cursor to position
pub fn cursor_move_to(r: u8, c: u8) {
    print!("\u{001b}[{};{}H", r, c);
}

/// Color string with RGB
pub fn rgb(s: &String, color: RGBColor) -> String {
    format!(
        "\u{001b}[38;2;{};{};{}m{}\u{001b}[0m",
        color.0, color.1, color.2, s
    )
}
