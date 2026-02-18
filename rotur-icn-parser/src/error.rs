use std::fmt;

use rotur_icn_lexer::{display::SpanDisplay, token};

#[derive(Debug, Clone)]
pub enum Error {
    TooManyArguments {
        keyword_span: token::Span,
        overflow_span: token::Span,
    },
    StrandedArguments {
        stranded_span: token::Span,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyArguments {
                keyword_span: keyword_pos,
                overflow_span: overflow_pos,
            } => {
                write!(
                    f,
                    "found too many arguments {} while parsing command {}",
                    SpanDisplay(overflow_pos),
                    SpanDisplay(keyword_pos)
                )
            }
            Self::StrandedArguments {
                stranded_span: stranded_pos,
            } => {
                write!(
                    f,
                    "found arguments {} at the beginning of the source",
                    SpanDisplay(stranded_pos)
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
