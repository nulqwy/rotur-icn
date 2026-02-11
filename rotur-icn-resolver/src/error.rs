use std::fmt;

use rotur_icn_lexer::{display::PosDisplay, token};

pub struct Error {
    pub cmd_pos: token::Pos,
    pub cmd_index: usize,
    pub kind: ErrorKind,
}

pub enum ErrorKind {
    DanglingContinuedLine,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cmd #{} {} : {}",
            self.cmd_index + 1,
            PosDisplay(&self.cmd_pos),
            self.kind,
        )
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DanglingContinuedLine => {
                write!(
                    f,
                    "there is no last point recorded to continue the line from"
                )
            }
        }
    }
}

impl ErrorKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DanglingContinuedLine => "ER00",
        }
    }

    pub fn help(&self) -> Option<&'static str> {
        match self {
            Self::DanglingContinuedLine => {
                Some("not all commands define a point to then continue the line from")
            }
        }
    }
}
