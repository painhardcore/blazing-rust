/// Counts occurrences of `needle` in `haystack`. Hot loop of the tokenizer.
pub fn count_byte(haystack: &[u8], needle: u8) -> usize {
    let mut n = 0;
    for i in 0..haystack.len() {
        if haystack[i] == needle {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_newlines() {
        assert_eq!(count_byte(b"a\nb\n\nc", b'\n'), 3);
    }
}
