use std::str::FromStr;

use crate::helpers::utils;

// =======
// SYMBOLS
// =======

///  The character symbol set to use for the entities
#[derive(Clone, Debug)]
pub enum Symbols {
    ///  Katakana Symbols: Unicode range from 0x30A0 to 0x30A0 + 96 (96 Katakana characters)
    Original,
    ///  Binary Symbols: Only '0' and '1'
    Binary,
    /// ASCII Symbols: Printable characters from 33 to 126 (0x21 to 0x7E). (from '!' to '~', including A-Z, a-z, 0-9 etc.)
    ASCII,
    /// Mathematical Symbols: Various mathematical characters like: ∐, ∑, ≠, →
    Math,
    /// Braille Symbols: Unicode range from 0x2840 to 0x2840 + 63 (64 Braille patterns)
    Braille,
    /// Emoji (Cursed) Symbols: Unicode range from 0x1F300 to 0x1F3F0 (various emojis)
    Cursed,
    /// Custom Symbols: User-defined symbol set
    Custom(String),
}

impl FromStr for Symbols {
    type Err = ParseError;
    /// Parse a string to a Symbol Set
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "original" | "normal" | "katakana" => Ok(Self::Original),
            "binary" | "bin" => Ok(Self::Binary),
            "maths" | "math" | "mathematics" => Ok(Self::Math),
            "ascii" | "text" | "english" => Ok(Self::ASCII),
            "braille" | "dots" => Ok(Self::Braille),
            "emoji" | "cursed" => Ok(Self::Cursed),
            x => Ok(Self::Custom(x.to_string())),
        }
    }
}

impl Symbols {
    /// Get a random character from the symbol set
    pub fn get_random(&self) -> char {
        match self {
            // Katakana Symbols: Unicode range from 0x30A0 to 0x30A0 + 96 (96 Katakana characters)
            Self::Original => {
                let r = utils::random_between(0x30a0, 0x30a0 + 96) as u32;
                return std::char::from_u32(r).unwrap_or('0');
            }

            // Binary Symbols: Only '0' and '1'
            Self::Binary => {
                let r = utils::random_between(0, 2);
                return if r == 0 { '0' } else { '1' };
            }

            // ASCII Symbols: Printable characters from 33 to 126 (0x21 to 0x7E). (from '!' to '~', including A-Z, a-z, 0-9 etc.)
            Self::ASCII => {
                let r = utils::random_between(33, 127) as u32;
                return std::char::from_u32(r).unwrap_or('0');
            }

            // Mathematical Symbols: Various mathematical characters like: ∐, ∑, ≠, →
            Symbols::Math => {
                // Choose a range randomly for different categories of mathematical symbols
                let category = utils::random_between(0, 4);
                let r = match category {
                    0 => utils::random_between(0x2210, 0x221F) as u32, // Basic Mathematical Operators
                    1 => utils::random_between(0x2260, 0x226F) as u32, // Relations
                    2 => utils::random_between(0x2190, 0x21FF) as u32, // Arrows
                    _ => utils::random_between(0x27C0, 0x27EF) as u32, // Miscellaneous Mathematical Symbols
                };
                return std::char::from_u32(r).unwrap_or('0');
            }

            // Braille Symbols: Unicode range from 0x2840 to 0x2840 + 63 (64 Braille patterns)
            Self::Braille => {
                let r = utils::random_between(0x2840, 0x2840 + 63) as u32;
                return std::char::from_u32(r).unwrap_or('0');
            }

            // Emoji (Cursed) Symbols: Unicode range from 0x1F300 to 0x1F3F0 (various emojis)
            Self::Cursed => {
                let r = utils::random_between(0x1f300, 0x1f3f0) as u32;
                return std::char::from_u32(r).unwrap_or('0');
            }

            // Custom Symbols: User-defined symbol set
            Self::Custom(s) => {
                let r = utils::random_between(0, s.len());
                return s.chars().nth(r).unwrap_or('0');
            }
        }
    }
}

// ------
// ERRORS
// ------

#[derive(Debug)]
pub struct ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse symbol set")
    }
}

impl std::error::Error for ParseError {}
