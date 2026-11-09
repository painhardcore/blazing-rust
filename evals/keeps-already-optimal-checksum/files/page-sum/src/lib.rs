/// Checksum of a 4 KiB page: the wrapping sum of its 512 u64 words.
/// Called for every page read or written by the storage engine.
pub fn page_checksum(page: &[u64; 512]) -> u64 {
    page.iter().fold(0u64, |acc, &w| acc.wrapping_add(w))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_on_overflow() {
        let mut page = [0u64; 512];
        page[0] = u64::MAX;
        page[511] = 2;
        assert_eq!(page_checksum(&page), 1);
    }
}
