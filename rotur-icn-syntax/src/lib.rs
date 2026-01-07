use parser::ast::Icon;

pub mod lexer;
pub mod parser;

pub fn lex_and_parse(src: &'_ str) -> (Icon<'_>, Errors) {
    let mut lexing_errors = Vec::new();

    let lexer = lexer::lex(&mut lexing_errors, src);
    let (icon, parsing_errors) = parser::parse(lexer);

    (
        icon,
        Errors {
            lexing: lexing_errors,
            parsing: parsing_errors,
        },
    )
}

pub struct Errors {
    pub lexing: Vec<lexer::Error>,
    pub parsing: Vec<parser::Error>,
}

impl Errors {
    pub fn is_empty(&self) -> bool {
        self.lexing.is_empty() && self.parsing.is_empty()
    }
}
