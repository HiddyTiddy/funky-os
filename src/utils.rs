/// takes slice of chars (as u8) and returns the parsed int
/// so [b'1', b'2', b'3'] -> 123
pub fn str_to_int(string: &[u8]) -> u32 {
    let mut out = 0;
    let mut factor = 1;
    for i in string.iter().rev() {
        let digit = (*i - b'0') as u32;
        out += digit * factor;
        factor *= 10;
    }
    out
}
