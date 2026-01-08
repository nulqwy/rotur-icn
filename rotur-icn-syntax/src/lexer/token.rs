#[derive(Debug, Clone)]
pub enum Token<'s> {
    Keyword(Keyword<'s>),
    Literal(Literal),
}

#[derive(Debug, Clone, Copy)]
pub struct Keyword<'s> {
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

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

impl Default for Colour {
    fn default() -> Self {
        Self {
            r: 0xff,
            g: 0,
            b: 0xff,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Number {
    pub value: f64,
}
