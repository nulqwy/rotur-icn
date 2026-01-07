use std::fmt;

use super::{Pos, display::PosDisplay};

#[derive(Debug, Clone)]
pub struct Error {
    pub pos: Pos,
    pub kind: ErrorKind,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidColour,
    StrandedNumber,
    StrandedColour,
    InvalidToken,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", PosDisplay(&self.pos), self.kind,)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidColour => write!(f, "found an invalid colour"),
            Self::StrandedNumber => write!(f, "found a stranded number"),
            Self::StrandedColour => write!(f, "found a stranded colour"),
            Self::InvalidToken => write!(f, "found an invalid token"),
        }
    }
}

impl ErrorKind {
    pub fn help(&self) -> &'static str {
        match self {
            Self::StrandedColour | Self::InvalidColour => {
                "colour is a 6-char or 3-char long HEX code (#rgb or #rrggbb)"
            }
            Self::StrandedNumber => "is this supposed to be a number?",
            Self::InvalidToken => "?? this abomination of a token is ignored",
        }
    }
}

impl std::error::Error for Error {}
