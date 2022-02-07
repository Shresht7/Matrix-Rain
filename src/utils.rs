use rand::Rng;

//  =========
//  UTILITIES
//  =========

/// ANSI RGB Color Tuple Struct (Red, Green, Blue)
#[derive(Clone, Copy)]
pub struct RGBColor(pub i32, pub i32, pub i32);

//  Matrix mode determines the character set to use for the entities
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Mode {
    Original,
    Binary,
    ASCII,
    Braille,
}

/// Generate a random number between min and max
pub fn random_between<T: PartialOrd + rand::distributions::uniform::SampleUniform>(
    min: T,
    max: T,
) -> T {
    rand::thread_rng().gen_range(min..max)
}

/// Clear the entire screen
pub fn clear_screen() {
    print!("\u{001b}[2J")
}

/// Move cursor to position
pub fn cursor_move_to(r: u32, c: u32) {
    print!("\u{001b}[{};{}H", r, c);
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

// #[cfg(tests)]
mod tests {

    #[test]
    fn no_random_number_between_0_and_1() {
        assert_eq!(super::random_between(0, 1), 0);
    }

    #[test]
    fn random_number_within_range() {
        let min = -15;
        let max = 45;
        for _ in 0..100 {
            let x = super::random_between(min, max);
            if x < min || x >= max {
                panic!("Random number not between specified range")
            }
        }
    }

    #[test]
    fn generates_a_new_random_number_everytime() {
        let first = super::random_between(0, 10);
        let second = super::random_between(0, 10);
        let third = super::random_between(0, 10);
        let all_different = first != second && second != third && third != first;
        assert_eq!(all_different, true);
    }

    /// Returns the type of the variable
    #[allow(dead_code)]
    fn type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }
    #[test]
    fn random_i32s_return_i32() {
        let number = super::random_between(-15, 32);
        let type_of_number = type_of(&number);
        assert_eq!(type_of_number, "i32");
    }

    #[test]
    fn random_u16s_return_u16() {
        let number = super::random_between::<u16>(0, 120);
        let type_of_number = type_of(&number);
        assert_eq!(type_of_number, "u16");
    }
}
