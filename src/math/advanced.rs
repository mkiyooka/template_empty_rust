/// Calculates the power of a number.
///
/// # Examples
///
/// ```
/// use sample::math::power;
/// assert_eq!(power(2, 3), 8);
/// assert_eq!(power(5, 2), 25);
/// ```
pub fn power(base: i32, exp: u32) -> i32 {
    base.pow(exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(power(2, 3), 8);
        assert_eq!(power(5, 2), 25);
        assert_eq!(power(10, 0), 1);
        assert_eq!(power(-2, 3), -8);
    }
}
