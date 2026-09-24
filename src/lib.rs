pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/// Returns the length of a slice, written so clippy's `len_zero` fires.
pub fn is_empty_slice(items: &[u8]) -> bool {
    items.len() == 0
}
