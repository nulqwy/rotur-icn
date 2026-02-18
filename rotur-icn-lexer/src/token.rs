use rotur_icn_units::{Colour, Number};

use crate::Error;

pub type Loc = lexgen_util::Loc;
pub type Span = (Loc, Loc);
pub type PToken<'s> = (Loc, Token<'s>, Loc);
pub type FToken<'s> = (Option<Token<'s>>, Option<Error>);
pub type PFToken<'s> = (Loc, FToken<'s>, Loc);

#[derive(Debug, Clone, Copy)]
pub enum Token<'s> {
    Identifier(Identifier<'s>),
    Literal(Literal),
}

#[derive(Debug, Clone, Copy)]
pub struct Identifier<'s> {
    pub value: &'s str,
}

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    Colour(Colour),
    Number(Number),
}

impl Literal {
    pub fn kind(&self) -> LiteralKind {
        match self {
            Self::Colour(_) => LiteralKind::Colour,
            Self::Number(_) => LiteralKind::Number,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LiteralKind {
    Colour,
    Number,
}

impl From<Literal> for LiteralKind {
    fn from(value: Literal) -> Self {
        value.kind()
    }
}

impl<'l> From<&'l Literal> for LiteralKind {
    fn from(value: &'l Literal) -> Self {
        value.kind()
    }
}
