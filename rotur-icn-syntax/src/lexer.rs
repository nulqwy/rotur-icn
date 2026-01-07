pub mod display;
mod error;
mod token;

pub use error::{Error, ErrorKind};
pub use token::{Colour, Keyword, Literal, Number, Token};

pub type Loc = lexgen_util::Loc;
pub type Pos = (Loc, Loc);
pub type PToken<'s> = (Loc, Token<'s>, Loc);

lexgen::lexer! {
    Lexer(State<'err>) -> Token<'input>;

    type Error = ErrorKind;

    $$ascii_whitespace,

    // ------- KEYWORDS -------

    $$ascii_alphabetic+ => |lexer| {
        lexer.return_(Token::Keyword(Keyword { value: lexer.match_() }))
    },

    // ------- NUMBERS -------

    ['-' '+']? ($$ascii_digit+ ('.' $$ascii_digit*)?) | ('.' $$ascii_digit+) ('e' $$ascii_digit+)? => |lexer| {
        let n = lexer.match_().parse::<f64>().expect("regex guarantees a valid f64");
        lexer.return_(Token::Literal(Literal::Number(Number { value: n })))
    },

    ['-' '+'] => |lexer| {
        let pos = lexer.match_loc();
        lexer.state().errors.push(Error { pos, kind: ErrorKind::StrandedNumber });

        lexer.return_(Token::Literal(Literal::Number(Default::default())))
    },

    ['-' '+']? '.' => |lexer| {
        let pos = lexer.match_loc();
        lexer.state().errors.push(Error { pos, kind: ErrorKind::StrandedNumber });

        lexer.return_(Token::Literal(Literal::Number(Default::default())))
    },

    ['-' '+']? '.'? 'e' $$ascii_digit* => |lexer| {
        let pos = lexer.match_loc();
        lexer.state().errors.push(Error { pos, kind: ErrorKind::StrandedNumber });

        lexer.return_(Token::Literal(Literal::Number(Default::default())))
    },

    // ------- COLOURS -------

    '#' $$ascii_hexdigit $$ascii_hexdigit $$ascii_hexdigit $$ascii_hexdigit $$ascii_hexdigit $$ascii_hexdigit => |lexer| {
        let n = u32::from_str_radix(&lexer.match_()[1..], 16).expect("regex guarantees a valid u32 (u24)");
        let [_, r, g, b] = n.to_be_bytes();
        lexer.return_(Token::Literal(Literal::Colour(Colour { r, g, b })))
    },

    // #rgb -> #rrggbb
    '#' $$ascii_hexdigit $$ascii_hexdigit $$ascii_hexdigit => |lexer| {
        let match_ = lexer.match_();

        let r = u8::from_str_radix(&match_[1..2], 16)
            .expect("regex guarantees a valid u8 (R-channel)");
        let g = u8::from_str_radix(&match_[2..3], 16)
            .expect("regex guarantees a valid u8 (G-channel)");
        let b = u8::from_str_radix(&match_[3..4], 16)
            .expect("regex guarantees a valid u8 (B-channel)");

        lexer.return_(Token::Literal(Literal::Colour(Colour { r: r * 17, g: g * 17, b: b * 17 })))
    },

    '#' $$ascii_alphanumeric+ => |lexer| {
        let pos = lexer.match_loc();
        lexer.state().errors.push(Error { pos, kind: ErrorKind::InvalidColour });

        lexer.return_(Token::Literal(Literal::Colour(Default::default())))
    },

    '#' => |lexer| {
        let pos = lexer.match_loc();
        lexer.state().errors.push(Error { pos, kind: ErrorKind::StrandedColour });

        lexer.return_(Token::Literal(Literal::Colour(Default::default())))
    },

    // ------- FALLBACK -------

    ($$alphanumeric | $$ascii_punctuation)+ => |lexer| {
        let pos = lexer.match_loc();
        lexer.state().errors.push(Error { pos, kind: ErrorKind::InvalidToken });

        lexer.reset_match();
        lexer.continue_()
    },
}

struct State<'err> {
    errors: &'err mut Vec<Error>,
}

pub fn lex<'err, 's>(
    errors_buf: &'err mut Vec<Error>,
    src: &'s str,
) -> impl Iterator<Item = PToken<'s>> + use<'err, 's> {
    Lexer::new_with_state(src, State { errors: errors_buf })
        .map(|r| r.expect("all errors should be collected in a buffer instead"))
}
