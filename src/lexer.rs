pub use crate::cursor::Cursor;
use crate::cursor::EOF_CHAR;
use crate::token::{Token, TokenKind,};

impl Cursor {
    pub fn advance_token(&mut self) -> Token {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Token::new(TokenKind::Eof, 0),
        };

        let token_kind = match first_char {

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

        }
    }
}
