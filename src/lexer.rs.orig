//! Lexer
//!

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TokenDesc {
    tag: i64,
}

macro_rules! desc {
    ($tag:expr) => {
        TokenDesc { tag: $tag }
    };
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    /// constant integer value
    Num(TokenDesc, i64),
    /// Reserved word or identificator
    Word(TokenDesc, String),
    /// Operator
    Op(TokenDesc, char),
    /// No token
    None,
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
    buf_index: usize,
    reader: Box<dyn Read>,
    words: HashMap<String, Token>,
    line: u64,
    content: String,
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

        let mut l = Self {
            buf_index: 0,
            reader,
            words,
            line: 0u64,
            content: String::new(),
        };
        l.read_to_buf().unwrap();
        l
    }

    pub fn read(&mut self) -> Token {
        let mut peek;
        loop {
            peek = self.next();
            if peek == ' ' || peek == '\t' {
                continue;
            } else if peek == '\n' {
                self.line += 1;
            } else {
                break;
            }
        }

        println!("current peek: {:?}", peek);

        if peek == '\0' {
            return Token::None;
        }

        // Skip comments if neccessary
        if peek == '/' {
            let mut next = self.next();
            if next == '/' {
                // Singleline comments end with the line
                // But we must not skip the '\n' char for
                // its we be processed in further `self.read()` call
                while !(self.see_next() == '\n' || next == '\0') {
                    next = self.next();
                }
                return self.read();
            } else if next == '*' {
                // For multiline comments
                // we must skip everything until we find '*/' construction
                while !(self.next() == '*' && self.next() == '/') { /*no-op*/ }
                return self.read();
            }
        }

        if peek.is_numeric() {
            let mut val: i64 = 0;
            while peek.is_numeric() {
                let digit = peek.to_digit(10).unwrap() as i64;
                val = 10 * val + digit;
                peek = self.next();
            }
            return Token::num(val);
        }

        if peek.is_alphabetic() {
            let s = &mut String::new();
            while peek.is_alphanumeric() {
                s.push(peek);
                peek = self.next();
            }
            let word = match self.words.get(s) {
                Some(word) => word.clone(),
                None => Token::id(s.to_string()),
            };
            return word;
        }
        Token::op(peek)
    }

    /// Reads next char in buffer and returns it.
    /// Incrementing `buf_index`.
    fn next(&mut self) -> char {
        let peek = self.char_at(self.buf_index);
        if peek != '\0' {
            self.buf_index += 1;
        }
        peek
    }

    /// Reads next char in buffer and returns it
    /// but do not increments the `buf_index` counter
    fn see_next(&self) -> char {
        self.char_at(self.buf_index)
    }

    fn char_at(&self, i: usize) -> char {
        if self.buf_index < self.content.len() {
            return self.content.as_bytes()[i] as char;
        }
        return '\0';
    }

    fn read_to_buf(&mut self) -> std::io::Result<usize> {
        self.reader.read_to_string(&mut self.content)
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
        assert_num!("56789032312");
    }

    macro_rules! assert_word {
        ($w:expr, $tag:ident) => {{
            let l = &mut Lexer::new(Box::new($w.as_bytes()));
            assert_eq!(Token::word(tag::$tag, $w.to_string()), l.read());
        }};
    }

    #[test]
    fn test_true() {
        assert_word!("true", TRUE);
    }

    #[test]
    fn test_false() {
        assert_word!("false", FALSE);
    }

    #[test]
    fn test_word() {
        assert_word!("example", ID);
    }

    #[test]
    fn test_singleline_comment() {
        let source = r#"// comment
            token"#;
        let l = &mut Lexer::new(Box::new(source.as_bytes()));
        assert_eq!(Token::word(tag::ID, "token".to_string()), l.read());
    }

    #[test]
<<<<<<< HEAD
    fn test_operator() {
        let l = &mut Lexer::new(Box::new("+".as_bytes()));
        match l.read() {
            Token::Op(desc, w) => {
                assert_eq!(desc.tag, '+' as i64);
                assert_eq!(w, '+');
            }
            _ => panic!("wrong token"),
        }

=======
    fn test_multilined_comment() {
        let s = r#"/* comment
                    * here
                    * and here
                    */
                    token"#;
        let l = &mut Lexer::new(Box::new(s.as_bytes()));
        assert_eq!(Token::word(tag::ID, "token".to_string()), l.read());
    }

    macro_rules! assert_op {
        ($op:expr) => {{
            let l = &mut Lexer::new(Box::new($op.as_bytes()));
            assert_eq!(Token::op($op.chars().next().unwrap()), l.read());
        }};
    }

    #[test]
    fn test_op() {
        assert_op!("+");
        assert_op!("-");
        assert_op!("/");
        assert_op!("/ some other stuff");
>>>>>>> e34a1bea8b73cf672777b18a6f2f0dce54af92f4
    }

    #[test]
    fn test_char_at() {
        let source = r#"// comment
            token"#;
        let l = &mut Lexer::new(Box::new(source.as_bytes()));
        assert_eq!('\n', l.char_at(10));
    }
}
