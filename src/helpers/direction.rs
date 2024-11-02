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
            "diagonal-left" | "bottom-left" => Ok(Self::DiagonalLeft),
            "diagonal-left-reverse" | "top-right" => Ok(Self::DiagonalLeftReverse),
            "diagonal-right" | "bottom-right" => Ok(Self::DiagonalRight),
            "diagonal-right-reverse" | "top-left" => Ok(Self::DiagonalRightReverse),
            _ => Err(ParseDirectionError::from(s.to_string())),
        }
    }
}

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
