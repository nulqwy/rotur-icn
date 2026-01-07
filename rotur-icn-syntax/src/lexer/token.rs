#[derive(Debug, Clone)]
pub enum Token<'s> {
    Keyword(Keyword<'s>),
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub struct Keyword<'s> {
    pub value: &'s str,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Colour(Colour),
    Number(Number),
}

#[derive(Debug, Clone, Default)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Default)]
pub struct Number {
    pub value: f64,
}
