use std::fmt;

use rotur_icn_lexer::{display::PosDisplay, token};
use rotur_icn_units::Number;

#[derive(Debug, Clone)]
pub struct Error {
    pub cmd_pos: token::Pos,
    pub cmd_index: usize,
    pub kind: ErrorKind,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    TooManyArguments {
        overflow_pos: token::Pos,
        exp: usize,
        got: usize,
    },
    TooFewArguments {
        args_end_loc: token::Loc,
        exp: usize,
        got: usize,
    },
    UnexpectedLiteralKind {
        arg_pos: token::Pos,
        arg_index: usize,
        exp: token::LiteralKind,
        got: token::LiteralKind,
    },
    InvalidNumericColour {
        arg_pos: token::Pos,
        arg_index: usize,
    },
    ArgOutOfRange {
        arg_pos: token::Pos,
        arg_index: usize,
        range_start: Option<(Number, bool)>,
        range_end: Option<(Number, bool)>,
    },
    InvalidCommand,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cmd #{} {} : {}",
            self.cmd_index + 1,
            PosDisplay(&self.cmd_pos),
            self.kind
        )
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
                    arg_index + 1,
                )
            }
            Self::InvalidNumericColour {
                arg_pos: _,
                arg_index,
            } => {
                write!(
                    f,
                    "the command's #{} arg is not a valid numerical colour",
                    arg_index + 1,
                )
            }
            Self::ArgOutOfRange {
                arg_pos: _,
                arg_index,
                range_start,
                range_end,
            } => {
                write!(
                    f,
                    "the command's #{} arg specifies a value outside of valid range of ",
                    arg_index + 1,
                )?;

                if let Some((bound, inclusive)) = range_start {
                    if *inclusive {
                        write!(f, "[")?;
                    } else {
                        write!(f, "(")?;
                    }

                    write!(f, "{bound}")?;
                } else {
                    write!(f, "(")?;
                }

                write!(f, "..")?;

                if let Some((bound, inclusive)) = range_end {
                    write!(f, "{bound}")?;

                    if *inclusive {
                        write!(f, "]")?;
                    } else {
                        write!(f, ")")?;
                    }
                } else {
                    write!(f, ")")?;
                }

                Ok(())
            }
            Self::InvalidCommand => {
                write!(f, "unknown command")
            }
        }
    }
}

impl ErrorKind {
    pub fn code(&self) -> &'static str {
        match self {
            ErrorKind::TooManyArguments { .. } => "EW00",
            ErrorKind::TooFewArguments { .. } => "EW01",
            ErrorKind::UnexpectedLiteralKind { .. } => "EW02",
            ErrorKind::InvalidNumericColour { .. } => "EW03",
            ErrorKind::ArgOutOfRange { .. } => "EW04",
            ErrorKind::InvalidCommand => "EW05",
        }
    }

    pub fn help(&self) -> Option<&'static str> {
        match self {
            Self::InvalidNumericColour { .. } => {
                Some("numerical colours should fit in a u24 (max: 16777215)")
            }
            Self::UnexpectedLiteralKind {
                got: token::LiteralKind::Colour,
                ..
            } => Some("colours cannot be used as hex numbers; did you use a correct command?"),
            // TODO help for invalid commands
            _ => None,
        }
    }
}
