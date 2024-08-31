use std::str::Chars;

pub(crate) const EOF_CHAR: char = '\0';

pub struct Cursor {
    len_remaining: usize,
    chars: Chars,
    //#[cfg(debug_assertions)]
    //prev: char,

}

impl Cursor {}

