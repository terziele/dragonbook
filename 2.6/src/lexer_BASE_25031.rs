//! Lexer
//!

#[derive(Clone, Copy)]
pub struct TokenDesc {
    tag: i64,
}

macro_rules! desc {
    ($tag:expr) => {
        TokenDesc { tag: $tag }
    };
}

#[derive(Clone)]
pub enum Token {
    /// constant integer value
    Num(TokenDesc, i64),
    /// Reserved word or identificator
    Word(TokenDesc, String),
    /// Operator
    Op(TokenDesc, char),
}

impl Token {
    fn num(val: i64) -> Self {
        Token::Num(desc!(tag::NUM), val)
    }

    fn word(tag: i64, val: String) -> Self {
        Token::Word(desc!(tag), val)
    }

    fn id(val: String) -> Self {
        Token::Word(desc!(tag::ID), val)
    }

    fn op(op: char) -> Self {
        Token::Op(desc!(op as i64), op)
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
        l.read_to_buf().unwrap();
        l
    }

    pub fn read(&mut self) -> Token {
        let mut peek;
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
                peek = self.next_char();
            }
            return Token::num(val);
        }

        if peek.is_alphabetic() {
            let s = &mut String::new();
            while peek.is_alphanumeric() {
                s.push(peek);
                peek = self.next_char();
            }
            let word = match self.words.get(s) {
                Some(word) => word.clone(),
                None => Token::id(s.to_string()),
            };
            return word;
        }
        Token::op(peek)
    }

    fn next_char(&mut self) -> char {
        let buf = &mut self.buf;
        let i = &mut self.buf_index;
        let peek = &mut self.peek;
        if *i < buf.len() {
            *peek = buf[*i] as char;
            // read and remove chunk from buffer
            buf[*i] = 0;
            *i += 1;

            return *peek;
        } else {
            *i = 0;
            self.read_to_buf().unwrap();
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
    reserve_tag!(ID, 259);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num<'a>() {
        macro_rules! assert_num {
            ($num:expr) => {
                let l = &mut Lexer::new(Box::new($num.as_bytes()));
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

    #[test]
    fn test_true() {
        let l = &mut Lexer::new(Box::new("true".as_bytes()));
        match l.read() {
            Token::Word(desc, w) => {
                assert_eq!(desc.tag, tag::TRUE);
                assert_eq!(w, "true".to_string());
            }
            _ => panic!("wrong token"),
        }
    }

    #[test]
    fn test_false() {

        let l = &mut Lexer::new(Box::new("false".as_bytes()));
        match l.read() {
            Token::Word(desc, w) => {
                assert_eq!(desc.tag, tag::FALSE);
                assert_eq!(w, "false".to_string());
            }
            _ => panic!("wrong token"),
        }
    }

    #[test]
    fn test_word() {
        let l = &mut Lexer::new(Box::new("example".as_bytes()));
        match l.read() {
            Token::Word(desc, w) => {
                assert_eq!(desc.tag, tag::ID);
                assert_eq!(w, "example".to_string());
            }
            _ => panic!("wrong token"),
        }

    }


}
