use std::error::Error;
use std::fmt;

// 1. Finish the definition
#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

impl Error for ParsePercentageError {}

impl fmt::Display for ParsePercentageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsePercentageError::InvalidInput => {
                write!(f, "Err(ParsePercentageError::InvalidInput)")
            }
            ParsePercentageError::OutOfRange => {
                write!(f, "Err(ParsePercentageError::OutOfRange)")
            }
        }
    }
}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    match input.parse::<u8>() {
        Ok(uvalue) => {
            if uvalue > 100 {
                return Err(ParsePercentageError::OutOfRange);
            } else {
                Ok(uvalue)
            }
        }
        Err(..) => Err(ParsePercentageError::InvalidInput),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
