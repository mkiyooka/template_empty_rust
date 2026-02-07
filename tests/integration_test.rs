use sample::math::CalcError;
use sample::{math, utils};

#[test]
fn test_integration_add() {
    assert_eq!(math::add(10, 20), 30);
    assert_eq!(math::add(5, 5), 10);
}

#[test]
fn test_integration_add_negative() {
    assert_eq!(math::add(-5, 5), 0);
    assert_eq!(math::add(-10, -20), -30);
}

#[test]
fn test_integration_math_multiply() {
    assert_eq!(math::multiply(3, 4), 12);
    assert_eq!(math::multiply(0, 10), 0);
}

#[test]
fn test_integration_math_subtract() {
    assert_eq!(math::subtract(10, 3), 7);
    assert_eq!(math::subtract(5, 10), -5);
}

#[test]
fn test_integration_math_divide_success() {
    assert_eq!(math::divide(12, 4), Ok(3));
    assert_eq!(math::divide(12, 5), Ok(2));
    assert_eq!(math::divide(0, 5), Ok(0));
}

#[test]
fn test_integration_math_divide_by_zero() {
    assert_eq!(math::divide(10, 0), Err(CalcError::DivideByZero));
    assert_eq!(math::divide(0, 0), Err(CalcError::DivideByZero));
}

#[test]
fn test_integration_utils_format_result() {
    assert_eq!(utils::format_result("Test", 123), "Test: 123");
}

#[test]
fn test_integration_utils_is_even() {
    assert!(utils::is_even(10));
    assert!(!utils::is_even(7));
}
