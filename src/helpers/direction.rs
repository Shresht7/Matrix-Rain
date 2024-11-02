/// Describes the direction of motion of the Matrix Streams
#[derive(Default, Clone, Debug)]
pub enum Direction {
    #[default]
    Down,
    Up,
    Left,
    Right,
    DiagonalLeft,
    DiagonalLeftReverse,
    DiagonalRight,
    DiagonalRightReverse,
}

impl std::str::FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" | "vertical-reverse" => Ok(Self::Up),
            "down" | "vertical" => Ok(Self::Down),
            "left" | "horizontal" => Ok(Self::Left),
            "right" | "horizontal-reverse" => Ok(Self::Right),
            "diagonal-left" | "top-left" => Ok(Self::DiagonalLeft),
            "diagonal-left-reverse" | "bottom-right" => Ok(Self::DiagonalLeftReverse),
            "diagonal-right" | "top-right" => Ok(Self::DiagonalRight),
            "diagonal-right-reverse" | "bottom-left" => Ok(Self::DiagonalRightReverse),
            _ => Err(ParseDirectionError::from(s.to_string())),
        }
    }
}

// impl Direction {
//     /// Returns a boolean indicating whether the direction is vertical
//     pub fn is_vertical(&self) -> bool {
//         match self {
//             Direction::Up | Direction::Down => true,
//             _ => false,
//         }
//     }

//     /// Returns a boolean indicating whether the direction is horizontal
//     pub fn is_horizontal(&self) -> bool {
//         match self {
//             Direction::Left | Direction::Right => true,
//             _ => false,
//         }
//     }

//      /// Returns a boolean indicating whether the direction is diagonal
//      pub fn is_diagonal(&self) -> bool {
//          match self {
//              Direction::DiagonalLeft => true,
//              _ => false
//          }
//      }
// }

// ERROR
// -----

#[derive(Debug)]
pub struct ParseDirectionError {
    value: String,
}

impl From<String> for ParseDirectionError {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl std::fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unsupported direction: {}", self.value)
    }
}

impl std::error::Error for ParseDirectionError {}
