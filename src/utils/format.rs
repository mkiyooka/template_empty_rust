/// Formats a result with a name and value.
///
/// # Examples
///
/// ```
/// use sample::utils::format_result;
/// assert_eq!(format_result("Sum", 42), "Sum: 42");
/// ```
pub fn format_result(name: &str, value: i32) -> String {
    format!("{}: {}", name, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_result() {
        assert_eq!(format_result("Sum", 42), "Sum: 42");
        assert_eq!(format_result("Product", 100), "Product: 100");
        assert_eq!(format_result("Difference", -5), "Difference: -5");
    }
}
