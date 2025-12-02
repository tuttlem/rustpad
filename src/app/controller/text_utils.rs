pub fn char_to_byte(text: &str, char_index: usize) -> usize {
    if char_index == 0 {
        return 0;
    }
    let mut count = 0;
    for (byte, _) in text.char_indices() {
        if count == char_index {
            return byte;
        }
        count += 1;
    }
    text.len()
}
