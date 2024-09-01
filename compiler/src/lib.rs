#![allow(unused_imports)]
#![allow(dead_code)]

pub mod token;
pub mod cursor;
pub use crate::token::{Token, TokenKind};
pub use crate::cursor::Cursor;

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


            '"' => TokenKind::DoubleQuote,
            '\'' => TokenKind::SingleQuote,
            '`' => TokenKind::Tick,
            '(' => TokenKind::OpenPar,
            ')' => TokenKind::ClosePar,
            '[' => TokenKind::OpenSqb,
            ']' => TokenKind::CloseSqb,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semi,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '!' => TokenKind::Bang,
            '|' => TokenKind::Pipe,
            '&' => TokenKind::Amper,
            '<' => TokenKind::Less,
            '>' => TokenKind::Greater,
            '=' => TokenKind::Equal,
            '.' => TokenKind::Dot,
            '~' => TokenKind::Tilde,
            '^' => TokenKind::Caret,
            '?' => TokenKind::Question,
            '#' => TokenKind::Pound,
            '$' => TokenKind::Dollar,
            '@' => TokenKind::At,
            _ => TokenKind::Unknown,

        };

        Token::new(token_kind, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let my_tok = Token {
            kind: TokenKind::Dollar,
            len: 1
        };
        assert_eq!(format!("{:?}", my_tok), "Token { kind: \"$\", len: 1 }");
    }

    #[test]
    fn test_cursor_chars() {
        let my_cur = Cursor::new("Hello");
        assert_eq!(my_cur.second(), 'e');
    }
}
