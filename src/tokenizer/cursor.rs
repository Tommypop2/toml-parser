use std::str::Chars;

/**
 * Peekable iteration over chars
 */
pub struct Cursor<'a> {
    pub next_i: usize,
    pub chars: Chars<'a>,
}
impl<'a> Cursor<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self { next_i: 0, chars }
    }
    pub fn next(&mut self) -> char {
        let chr = self.peek();
        self.next_i += 1;
        chr
    }
    pub fn peek(&mut self) -> char {
        self.chars.nth(self.next_i).unwrap()
    }
}
