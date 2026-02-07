/// Checks if a number is even.
///
/// # Examples
///
/// ```
/// use sample::utils::is_even;
/// assert_eq!(is_even(4), true);
/// assert_eq!(is_even(5), false);
/// ```
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(0));
        assert!(is_even(2));
        assert!(is_even(4));
        assert!(is_even(-2));
        assert!(!is_even(1));
        assert!(!is_even(3));
        assert!(!is_even(-1));
    }
}
