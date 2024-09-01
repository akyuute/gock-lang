use std::str::Chars;

use crate::token::{Token, TokenKind};

pub(crate) const EOF_CHAR: char = '\0';

pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
    //#[cfg(debug_assertions)]
    //prev: char,

}

impl<'a> Cursor<'a> {

    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len_remaining: input.len(),
            chars: input.chars(),
        }
    }

    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn third(&self) -> char {
        let mut iter = self.chars.clone();
            iter.next();
            iter.next();
            iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        Some(c)
    }

}
