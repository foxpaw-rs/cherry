//! Validate module
//!
//! The Validate module provides some default validation methods to be used
//! with the filters on Arguments and Fields. Note that all methods perform
//! validation against arabic numerals and english alphabet.

/// Determine if the value is a number.
///
/// Determine whether the provided value represents a valid number. Valid
/// numbers are defined as `-?[0-9]*?\.?[0-9]+`.
pub fn is_numeric(value: &str) -> bool {
    value.parse::<f32>().is_ok()
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Method is_numeric must pass on number.
    ///
    /// If provided a number, is_numeric must return true.
    #[test]
    fn is_numeric_number() {
        assert!(is_numeric("0"));
    }

    /// Method is_numeric must pass on number dot number.
    ///
    /// If provided a number, followed by a dot character followed by a number,
    /// is_numeric must return true.
    #[test]
    fn is_numeric_number_dot_number() {
        assert!(is_numeric("0.0"));
    }

    /// Method is_numeric must pass on dot number.
    ///
    /// If provided a dot character followed by a number, is_numeric must return
    /// true.
    #[test]
    fn is_numeric_dot_number() {
        assert!(is_numeric(".0"));
    }

    /// Method is_numeric must fail on a number dot.
    ///
    /// If provided a number followed by a dot character, is_numeric must fail.
    #[test]
    fn is_numeric_number_dot() {
        assert!(is_numeric("0."));
    }

    /// Method is_numeric must pass on negative number.
    ///
    /// If provided a negative character followed by a number, is_numeric must
    /// return true.
    #[test]
    fn is_numeric_negative_number() {
        assert!(is_numeric("-0"));
    }

    /// Method is_numeric must pass on negative number dot number.
    ///
    /// If provided a negative character followed by a number, followed by a dot
    /// character followed by a number, is_numeric must return true.
    #[test]
    fn is_numeric_negative_number_dot_number() {
        assert!(is_numeric("-0.0"));
    }

    /// Method is_numeric must pass on negative dot number.
    ///
    /// If provided a negative character followed by a dot character followed by a
    /// number, is_numeric must return true.
    #[test]
    fn is_numeric_negative_dot_number() {
        assert!(is_numeric("-.0"));
    }

    /// Method is_numeric must fail on a negative number dot.
    ///
    /// If provided a negative followed by a number followed by a dot character,
    /// is_numeric must fail.
    #[test]
    fn is_numeric_negative_number_dot() {
        assert!(is_numeric("-0."));
    }

    /// Method is_numeric must fail on a non number.
    ///
    /// If provided a non number character, is_numeric must fail.
    #[test]
    fn is_numeric_non_number() {
        assert!(!is_numeric("a"));
    }

    /// Method is_numeric must fail on a non number contained in a number.
    ///
    /// If provided a non number character in a number, is_numeric must fail.
    #[test]
    fn is_numeric_number_non_number() {
        assert!(!is_numeric("0a"));
    }

    /// Method is_numeric must fail on a negative character only.
    ///
    /// If provided a negative character only, is_numeric must fail.
    #[test]
    fn is_numeric_negative_only() {
        assert!(!is_numeric("-"));
    }
}