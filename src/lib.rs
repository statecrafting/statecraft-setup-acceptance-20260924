/// Adds two numbers.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Adds two numbers, returning `None` instead of overflowing.
pub fn checked_add(left: u64, right: u64) -> Option<u64> {
    left.checked_add(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn checked_add_refuses_overflow() {
        assert_eq!(checked_add(2, 2), Some(4));
        assert_eq!(checked_add(u64::MAX, 1), None);
    }
}
