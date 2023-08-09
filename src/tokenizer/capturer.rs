pub enum Capture {
    Char(char),
}
/**
 * Doesn't include the current, or final char
 */
pub fn capture_until(bytes: &[u8], i: &mut usize, stop: Capture) -> String {
    let mut chars = String::from("");
    loop {
        *i += 1;
        let inside_char = bytes[*i] as char;
        if match stop {
            Capture::Char(chr) => inside_char == chr,
        } {
            break;
        }
        chars.push(inside_char);
    }
    chars
}
