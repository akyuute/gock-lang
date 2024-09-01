#![allow(unused_imports)]
#![allow(dead_code)]

pub mod token;
pub mod cursor;
pub use crate::token::{Token, TokenKind};
pub use crate::cursor::Cursor;
use crate::token::LiteralKind::*;
use crate::token::TokenKind::*;

#[cfg(test)]
mod tests;

pub fn is_whitespace(c: char) -> bool {
    false
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Token {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Token::new(TokenKind::Eof, 0),
        };

        let token_kind = match first_char {
//            '/' => match self.next() {
//                '/' => self.line_comment(),
//                '*' => self.block_comment(),
//                _ => TokenKind::Slash,
//            },


            '"' => DoubleQuote,
            '\'' => SingleQuote,
            '`' => Tick,
            '(' => OpenPar,
            ')' => ClosePar,
            '[' => OpenSqb,
            ']' => CloseSqb,
            '{' => OpenBrace,
            '}' => CloseBrace,
            ':' => Colon,
            ',' => Comma,
            ';' => Semi,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
                '/' => Slash,
            '%' => Percent,
            '!' => Bang,
            '|' => Pipe,
            '&' => Amper,
            '<' => Less,
            '>' => Greater,
            '=' => Equal,
            '.' => Dot,
            '~' => Tilde,
            '^' => Caret,
            '?' => Question,
            '#' => Pound,
            '$' => Dollar,
            '@' => At,
            _ => Unknown,

        };

        Token::new(token_kind, 1)
    }
}

