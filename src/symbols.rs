// Library
use crate::utils;
use std::str::FromStr;

// =======
// SYMBOLS
// =======

///  The character symbol set to use for the entities
#[derive(Clone, Copy, Debug)]
pub enum Symbols {
    Original, //  Katakana
    Binary,   //  0 or 1
    ASCII,    //  ASCII
    Braille,  //  Braille
    Cursed,   //  Emoji
}

impl FromStr for Symbols {
    type Err = anyhow::Error;
    /// Parse a string to a Symbol Set
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "original" => Ok(Symbols::Original),
            "katakana" => Ok(Symbols::Original),
            "normal" => Ok(Symbols::Original),
            "binary" => Ok(Symbols::Binary),
            "ascii" => Ok(Symbols::ASCII),
            "braille" => Ok(Symbols::Braille),
            "emoji" => Ok(Symbols::Cursed),
            "cursed" => Ok(Symbols::Cursed),
            // TODO: Custom Symbols
            _ => Err(anyhow::Error::msg("Invalid Symbol Set. Please select from 'original', 'binary', 'ascii', or 'braille'")),
        }
    }
}

impl Symbols {
    /// Get a random character from the symbol set
    pub fn get_random(&self) -> char {
        match self {
            //  Katakana Symbols
            Symbols::Original => {
                let r = utils::random_between(0x30a0, 0x30a0 + 96) as u32;
                return std::char::from_u32(r).unwrap_or('0');
            }

            //  Binary Symbols
            Symbols::Binary => {
                let r = utils::random_between(0, 2);
                return if r == 0 { '0' } else { '1' };
            }

            //  ASCII Symbols
            Symbols::ASCII => {
                let r = utils::random_between(33, 127) as u32;
                return std::char::from_u32(r).unwrap_or('0');
            }

            //  Braille Symbols
            Symbols::Braille => {
                let r = utils::random_between(0x2840, 0x2840 + 63) as u32;
                return std::char::from_u32(r).unwrap_or('0');
            }

            //  (Cursed) Emoji Symbols
            Symbols::Cursed => {
                let r = utils::random_between(0x1f300, 0x1f3f0) as u32;
                return std::char::from_u32(r).unwrap_or('0');
            }
        }
    }
}
