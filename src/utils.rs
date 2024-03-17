//  Library
use rand::Rng;
use std::str::FromStr;

//  =========
//  UTILITIES
//  =========

//  Matrix mode determines the character set to use for the entities
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Mode {
    Original,
    Binary,
    ASCII,
    Braille,
}

// Implement the FromStr trait for Mode to parse the command-line argument
impl FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "original" => Ok(Mode::Original),
            "binary" => Ok(Mode::Binary),
            "ascii" => Ok(Mode::ASCII),
            "braille" => Ok(Mode::Braille),
            _ => Err(anyhow::Error::msg("Invalid Mode")),
        }
    }
}

/// Generate a random number between min and max
pub fn random_between<T: PartialOrd + rand::distributions::uniform::SampleUniform>(
    min: T,
    max: T,
) -> T {
    rand::thread_rng().gen_range(min..max)
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
