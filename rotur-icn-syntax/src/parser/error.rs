use std::fmt;

use crate::lexer::{self, display::PosDisplay};

#[derive(Debug, Clone)]
pub enum Error {
    TooManyArguments {
        keyword_pos: lexer::Pos,
        overflow_pos: lexer::Pos,
    },
    StrandedArguments {
        stranded_pos: lexer::Pos,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyArguments {
                keyword_pos,
                overflow_pos,
            } => {
                write!(
                    f,
                    "found too many arguments {} while parsing command {}",
                    PosDisplay(overflow_pos),
                    PosDisplay(keyword_pos)
                )
            }
            Self::StrandedArguments { stranded_pos } => {
                write!(
                    f,
                    "found arguments {} at the beginning of the source",
                    PosDisplay(stranded_pos)
                )
            }
        }
    }
}

impl Error {
    pub fn code(&self) -> &'static str {
        match self {
            Self::TooManyArguments { .. } => "EP00",
            Self::StrandedArguments { .. } => "EP01",
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::TooManyArguments { .. } => "found too many arguments while parsing a command",
            Self::StrandedArguments { .. } => "found arguments at the beginning of the source",
        }
    }

    pub fn help(&self) -> &'static str {
        match self {
            Self::TooManyArguments { .. } => {
                "in ICN commands have at most 6 arguments; did you miss a command somewhere within the arguments?"
            }
            Self::StrandedArguments { .. } => "did you miss a command before these arguments?",
        }
    }
}

impl std::error::Error for Error {}
