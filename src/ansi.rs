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

/// ANSI RGB Color Tuple Struct (Red, Green, Blue)
#[derive(Clone, Copy)]
pub struct RGBColor(pub i32, pub i32, pub i32);

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
