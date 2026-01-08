use std::fmt;

use rotur_icn_syntax::lexer::{self, display::PosDisplay};

#[derive(Debug, Clone)]
pub struct Error {
    pub cmd_pos: lexer::Pos,
    pub kind: ErrorKind,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    TooManyArguments {
        overflow_pos: lexer::Pos,
        exp: usize,
        got: usize,
    },
    TooFewArguments {
        args_end_loc: lexer::Loc,
        exp: usize,
        got: usize,
    },
    UnexpectedLiteralKind {
        arg_pos: lexer::Pos,
        arg_index: usize,
        exp: lexer::LiteralKind,
        got: lexer::LiteralKind,
    },
    InvalidNumericColour {
        arg_pos: lexer::Pos,
        arg_index: usize,
    },
    InvalidCommand,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cmd {} : {}", PosDisplay(&self.cmd_pos), self.kind)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyArguments {
                overflow_pos: _,
                exp,
                got,
            } => {
                write!(f, "the command expected fewer ({exp}) args, received {got}")
            }
            Self::TooFewArguments {
                args_end_loc: _,
                exp,
                got,
            } => {
                write!(f, "the command expected more ({exp}) args, received {got}")
            }
            Self::UnexpectedLiteralKind {
                arg_pos: _,
                arg_index,
                exp,
                got,
            } => {
                write!(
                    f,
                    "the command's #{} arg is supposed to be {exp}, but got {got}",
                    arg_index + 1
                )
            }
            Self::InvalidNumericColour {
                arg_pos: _,
                arg_index,
            } => {
                write!(
                    f,
                    "the command's #{} arg is not a valid numerical colour",
                    arg_index + 1
                )
            }
            Self::InvalidCommand => {
                write!(f, "unknown command")
            }
        }
    }
}

impl ErrorKind {
    pub fn help(&self) -> Option<&'static str> {
        match self {
            Self::InvalidNumericColour { .. } => {
                Some("numerical colours should fit in a u24 (max: 16777215)")
            }
            Self::UnexpectedLiteralKind {
                got: lexer::LiteralKind::Colour,
                ..
            } => Some("colours cannot be used as hex numbers; did you use correct command?"),
            // TODO help for invalid commands
            _ => None,
        }
    }
}
