/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use sample::math::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers.
///
/// # Examples
///
/// ```
/// use sample::math::multiply;
/// assert_eq!(multiply(3, 4), 12);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Subtracts the second number from the first.
///
/// # Examples
///
/// ```
/// use sample::math::subtract;
/// assert_eq!(subtract(10, 3), 7);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Subtracts the second number from the first.
///
/// # Examples
///
/// ```
/// use sample::math::subtract;
/// assert_eq!(subtract(10, 3), 7);
/// ```
#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivideByZero,
}

pub fn divide(a: i32, b: i32) -> Result<i32, CalcError> {
    if b == 0 {
        Err(CalcError::DivideByZero)
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(0, 5), 0);
        assert_eq!(multiply(-2, 3), -6);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 3), 7);
        assert_eq!(subtract(5, 5), 0);
        assert_eq!(subtract(0, 10), -10);
    }

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(12, 4), Ok(3));
        assert_eq!(divide(12, 5), Ok(2));
        assert_eq!(divide(0, 5), Ok(0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), Err(CalcError::DivideByZero));
        assert_eq!(divide(0, 0), Err(CalcError::DivideByZero));
    }
}
