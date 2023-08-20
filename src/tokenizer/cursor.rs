use std::{iter::Peekable, str::Chars};

/**
 * Peekable iteration over chars
 */
pub struct Cursor<'a> {
    started_iterating: bool,
    pub pos: usize,
    pub chars: Peekable<Chars<'a>>,
}
impl<'a> Cursor<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            started_iterating: false,
            pos: 0,
            chars: chars.peekable(),
        }
    }
    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.started_iterating {
            self.pos += 1;
        }
        let chr = self.chars.next();
        self.started_iterating = true;
        chr
    }
}
