//! Lexer
//!

pub struct TokenDesc {
    tag: i64,
}

macro_rules! desc {
    ($tag:expr) => {
        TokenDesc {
            tag: $tag,
        }
    };
}

pub enum Token {
    Num(TokenDesc, i64),
    Word(TokenDesc, String),
}

impl Token {
    fn num(val: i64) -> Self {
        Token::Num(desc!(tag::NUM), val)
    }

    fn word(tag: i64, val: String) -> Self {
        Token::Word(desc!(tag), val)
    }
}

use std::io::Read;
use std::collections::HashMap;
struct Lexer {
    peek: char,
    buf: [char;10],
    reader: Box<dyn Read>,
    words: HashMap<String, Token>,
}

mod tag {
    macro_rules! reserve_tag {
        ($tag:ident, $val:expr) => {
            pub const $tag: i64 = $val;
        };
    }

    reserve_tag!(NUM, 256);
    reserve_tag!(TRUE, 257);
    reserve_tag!(FALSE, 258);
}
