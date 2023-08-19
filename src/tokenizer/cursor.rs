use std::str::Chars;

pub struct Cursor<'a> {
    next_i: usize,
    chars: Chars<'a>,
}
impl<'a> Cursor<'a> {
    fn next(&mut self) -> char {
        let chr = self.peek();
        self.next_i += 1;
        chr
    }
    fn peek(&mut self) -> char {
        self.chars.nth(self.next_i).unwrap()
    }
}
