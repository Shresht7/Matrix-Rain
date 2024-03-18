// Library
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
}

impl FromStr for Symbols {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "original" => Ok(Symbols::Original),
            "katakana" => Ok(Symbols::Original),
            "normal" => Ok(Symbols::Original),
            "binary" => Ok(Symbols::Binary),
            "ascii" => Ok(Symbols::ASCII),
            "braille" => Ok(Symbols::Braille),
            _ => Err(anyhow::Error::msg("Invalid Symbol Set. Please select from 'original', 'binary', 'ascii', or 'braille'")),
        }
    }
}
