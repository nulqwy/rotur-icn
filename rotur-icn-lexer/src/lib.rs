pub mod display;
mod error;
pub mod token;

use rotur_icn_units::{Colour, Number};

pub use error::{Error, ErrorKind};
use token::{FToken, Identifier, Literal, LiteralKind, Loc, PFToken, Span, Token};

lexgen::lexer! {
    Lexer -> FToken<'input>;

    $$ascii_whitespace,

    // ------- KEYWORDS -------

    $$ascii_alphabetic+ => |lexer| {
        lexer.return_((
            Some(Token::Identifier(Identifier { value: lexer.match_() })),
            None,
        ))
    },

    // ------- NUMBERS -------

    ['-' '+']? (($$ascii_digit+ ('.' $$ascii_digit*)?) | ('.' $$ascii_digit+)) ('e' $$ascii_digit+)? => |lexer| {
        let n = lexer.match_().parse::<Number>().expect("regex guarantees a valid f64");
        lexer.return_((
            Some(Token::Literal(Literal::Number(n))),
            None,
        ))
    },

    ['-' '+'] => |lexer| {
        let span = lexer.match_loc();
        lexer.return_((
            Some(Token::Literal(Literal::Number(Default::default()))),
            Some(Error { span, kind: ErrorKind::StrandedNumber }),
        ))
    },

    ['-' '+']? '.' => |lexer| {
        let span = lexer.match_loc();

        lexer.return_((
            Some(Token::Literal(Literal::Number(Default::default()))),
            Some(Error { span, kind: ErrorKind::StrandedNumber }),
        ))
    },

    ['-' '+']? '.'? 'e' $$ascii_digit* => |lexer| {
        let span = lexer.match_loc();
        lexer.return_((
            Some(Token::Literal(Literal::Number(Default::default()))),
            Some(Error { span, kind: ErrorKind::StrandedNumber }),
        ))
    },

    // ------- COLOURS -------

    '#' $$ascii_hexdigit $$ascii_hexdigit $$ascii_hexdigit $$ascii_hexdigit $$ascii_hexdigit $$ascii_hexdigit => |lexer| {
        let n = u32::from_str_radix(&lexer.match_()[1..], 16).expect("regex guarantees a valid u32 (u24)");
        lexer.return_((
            Some(Token::Literal(Literal::Colour(n.try_into().expect("regex only allows for u24-sized u32")))),
            None,
        ))
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

        lexer.return_((
            Some(Token::Literal(Literal::Colour(Colour { r: r * 17, g: g * 17, b: b * 17, a: 0xff }))),
            None,
        ))
    },

    '#' $$ascii_alphanumeric+ => |lexer| {
        let span = lexer.match_loc();
        lexer.return_((
            Some(Token::Literal(Literal::Colour(Colour::default()))),
            Some(Error { span, kind: ErrorKind::InvalidColour }),
        ))
    },

    '#' => |lexer| {
        let span = lexer.match_loc();
        lexer.return_((
            Some(Token::Literal(Literal::Colour(Colour::default()))),
            Some(Error { span, kind: ErrorKind::StrandedColour }),
        ))
    },

    // ------- FALLBACK -------

    ($$alphanumeric | $$ascii_punctuation)+ => |lexer| {
        let span = lexer.match_loc();
        lexer.return_((
            None,
            Some(Error { span, kind: ErrorKind::InvalidToken })
        ))
    },
}

pub fn lex(src: &str) -> impl Iterator<Item = PFToken<'_>> {
    #[expect(clippy::missing_panics_doc, reason = "for bug catching")]
    Lexer::new(src).map(|r| r.expect("all errors should be recoverable"))
}
