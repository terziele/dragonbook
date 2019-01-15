//! Lexer
//!

pub struct TokenDesc {
    tag: i64,
}

macro_rules! desc {
    ($tag:expr) => {
        TokenDesc { tag: $tag }
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

use std::collections::HashMap;
use std::io::Read;
pub struct Lexer {
    peek: char,
    buf: [u8; 10],
    buf_index: usize,
    reader: Box<dyn Read>,
    words: HashMap<String, Token>,
    line: u64,
}

impl Lexer {
    pub fn new(reader: Box<dyn Read>) -> Self {
        let mut words = HashMap::new();
        macro_rules! reserve_word {
            ($word:expr, $tag:expr) => {{
                let lexeme = $word.to_string();
                let word = Token::word($tag, lexeme.clone());
                words.insert(lexeme, word);
            }};
        }

        reserve_word!("true", tag::TRUE);
        reserve_word!("false", tag::FALSE);

        let _buf = [0; 10];
        let mut l = Self {
            peek: ' ',
            buf: [0; 10],
            buf_index: 0,
            reader,
            words,
            line: 0u64,
        };
        l.read_to_buf();
        l
    }

    pub fn read(&mut self) -> Token {
        let mut peek = ' ';
        loop {
            peek = self.next_char();
            if peek == ' ' || peek == '\t' {
                continue;
            } else if peek == '\n' {
                self.line += 1;
            } else {
                break;
            }
        }
        println!("current peek: {:?}", peek);

        if peek.is_numeric() {
            let mut val: i64 = 0;
            while peek.is_numeric() {
                let digit = peek.to_digit(10).unwrap() as i64;
                val = 10 * val + digit;
                println!("val: {:?}", val);
                peek = self.next_char();
            }
            return Token::num(val);
        }
        panic!("at the disco");
    }

    fn next_char(&mut self) -> char {
        if self.buf_index < self.buf.len() {
            self.peek = self.buf[self.buf_index] as char;
            self.buf_index += 1;
            println!("buf: {:?}", self.buf);
            println!("peek: {:?}", self.peek);
            return self.peek;
        } else {
            self.read_to_buf().unwrap();
            self.buf_index = 0;
            return self.next_char();
        }
    }
    fn read_to_buf(&mut self) -> std::io::Result<usize> {
        self.reader.read(&mut self.buf)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num<'a>() {
        macro_rules! assert_num {
            ($num:expr) => {
                let mut l = &mut Lexer::new(Box::new($num.as_bytes()));
                match l.read() {
                    Token::Num(_desc, val) => assert_eq!($num.parse::<i64>().unwrap(), val),
                    _ => panic!("wrong token"),
                }
            };
        }

        assert_num!("123");
        assert_num!("1234");
        assert_num!("12345");
        assert_num!("123456");

        assert_num!("12345678912");
    }

}
