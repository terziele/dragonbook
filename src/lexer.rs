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
pub struct Lexer {
    peek: char,
    buf: [u8;10],
    buf_index: usize,
    reader: Box<dyn Read>,
    words: HashMap<String, Token>,
    line: u64,
}

impl Lexer {
    pub fn new(reader: Box<dyn Read>) -> Self {
        let mut words = HashMap::new();
        macro_rules! reserve_word {
            ($word:expr, $tag:expr) => {
                {
                    let lexeme = $word.to_string();
                    let word = Token::word($tag, lexeme.clone());
                    words.insert(lexeme, word) ;
                }
            };
        }

        reserve_word!("true", tag::TRUE);
        reserve_word!("false", tag::FALSE);

        Self {
            peek: ' ', 
            buf: [0;10],
            buf_index: 0,
            reader,
            words,
            line: 0u64,
        }
    }

    pub fn read(&mut self) -> Token {
        loop {
            let peek = self.next_char();

            if peek == ' ' || peek == '\t' {
                continue;
            } else if peek == '\n' {
                self.line += 1;
            } else {
                break;
            }

            if peek.is_digit(0) {
                let val = 0;
            }


        }



        Token::num(10)
    }

    fn next_char(&mut self) -> char {
        if self.buf_index <  self.buf.len() {
            self.peek = self.buf[self.buf_index] as char;
            self.buf_index += 1;
            return self.peek;
        } else {
            // fill the buffer
            self.buf_index = 0;
            self.reader.read(&mut self.buf);
            if self.buf.is_empty() {
                return '\0';
            }
            return self.next_char();
        }
    }
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
