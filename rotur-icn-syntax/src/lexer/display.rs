use std::fmt;

use super::{Colour, Keyword, Literal, LiteralKind, Loc, Number, Pos, Token};

pub struct LocDisplay<'l>(pub &'l Loc);

impl fmt::Display for LocDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@ {:0>3}:{:0>2}", self.0.line + 1, self.0.col + 1)
    }
}

pub struct PosDisplay<'p>(pub &'p Pos);

impl fmt::Display for PosDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "@ {:0>3}:{:0>2}..{:0>3}:{:0>2}",
            self.0.0.line + 1,
            self.0.0.col + 1,
            self.0.1.line + 1,
            self.0.1.col + 1
        )
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keyword(kw) => write!(f, "kw {kw}"),
            Self::Literal(lit) => write!(f, "lit {lit}"),
        }
    }
}

impl fmt::Display for Keyword<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Colour(col) => write!(f, "{col}"),
            Self::Number(n) => write!(f, "{n}"),
        }
    }
}

impl fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Colour => write!(f, "colour"),
            Self::Number => write!(f, "number"),
        }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:0>2x}{:0>2x}{:0>2x}", self.r, self.g, self.b)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
