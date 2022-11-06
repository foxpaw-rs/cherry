//! Validate module
//!
//! The Validate module provides some default validation methods to be used
//! with the filters on Arguments and Fields. Note that all methods perform
//! validation against arabic numerals and english alphabet.

/// Determine if the valud is alphanumeric.
///
/// Determine whether the provided value contains only alphanumeric characters.
/// Alphanumeric characters are defined as `[a-zA-Z0-9]+`
///
/// # Example
/// ```rust
/// use cherry::validate::is_alphanumeric;
///
/// // Alphanumeric
/// assert!(is_alphanumeric("a"));
/// assert!(is_alphanumeric("1"));
/// assert!(is_alphanumeric("A1"));
///
/// // Not alphanumeric
/// assert!(!is_alphanumeric("1.0"));
/// assert!(!is_alphanumeric("-a"));
/// assert!(!is_alphanumeric("$"));
/// ```
pub fn is_alphanumeric(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9'))
}

/// Determine if the value is an integer.
///
/// Determine whether the provided value represents a valid integer.
///
/// # Example
/// ```rust
/// use cherry::validate::is_integer;
///
/// // Integers
/// assert!(is_integer("10"));
/// assert!(is_integer("-10"));
/// assert!(is_integer("0"));
///
/// // Not integers
/// assert!(!is_integer("a"));
/// assert!(!is_integer("10.2"));
/// ```
pub fn is_integer(value: &str) -> bool {
    value.parse::<i32>().is_ok()
}

/// Determine if a value is a negative number.
///
/// Determine whether the provided value is a negative number.
///
/// # Example
/// ```rust
/// use cherry::validate::is_negative;
///
/// assert!(is_negative("-1.5"));
///
/// assert!(!is_negative("1"));
/// assert!(!is_negative("0"));
/// ```
pub fn is_negative(value: &str) -> bool {
    value.parse::<f32>().map_or(false, |value| value < 0.0)
}

/// Determine if the value is a number.
///
/// Determine whether the provided value represents a valid number. Valid
/// numbers are defined as `-?[0-9]*?\.?[0-9]+`.
///
/// # Example
/// ```rust
/// use cherry::validate::is_numeric;
///
/// // Numeric
/// assert!(is_numeric("10.2"));
/// assert!(is_numeric("10."));
/// assert!(is_numeric("-.10"));
///
/// // Not numeric
/// assert!(!is_numeric("a"));
/// assert!(!is_numeric("1a"));
/// assert!(!is_numeric("-"));
/// ```
pub fn is_numeric(value: &str) -> bool {
    value.parse::<f32>().is_ok()
}

/// Determine if a value is a positive number.
///
/// Determine whether the provided value is a positive number.
///
/// # Example
/// ```rust
/// use cherry::validate::is_positive;
///
/// assert!(is_positive("1.5"));
///
/// assert!(!is_positive("-1"));
/// assert!(!is_positive("0"));
/// ```
pub fn is_positive(value: &str) -> bool {
    value.parse::<f32>().map_or(false, |value| value > 0.0)
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Method is_alphanumeric must pass on alpha tokens.
    ///
    /// If provided alpha tokens, is_alphanumeric must return true.
    #[test]
    fn is_alphanumeric_alpha() {
        assert!(is_alphanumeric("abc"));
    }

    /// Method is_alphanumeric must pass on numeric tokens.
    ///
    /// If provided numeric tokens, is_alphanumeric must return true.
    #[test]
    fn is_alphanumeric_number() {
        assert!(is_alphanumeric("123"));
    }

    /// Method is_alphanumeric must pass on alphanumeric tokens.
    ///
    /// If provided alphanumeric tokens, is_alphanumeric must return true.
    #[test]
    fn is_alphanumeric_alphanumeric() {
        assert!(is_alphanumeric("abc123"));
    }

    /// Method is_alphanumeric must fail on non alphanumeric tokens.
    ///
    /// If provided non alphanumeric tokens, is_alphanumeric must return false.
    #[test]
    fn is_alphanumeric_fail() {
        assert!(!is_alphanumeric("1.0"));
    }

    /// Method is_integer must pass on integer.
    ///
    /// If provided an integer, is_integer must return true.
    #[test]
    fn is_integer_number() {
        assert!(is_integer("1"));
    }

    /// Method is_integer must pass on negative integer.
    ///
    /// If provided a negative integer, is_integer must return true.
    #[test]
    fn is_integer_negative_number() {
        assert!(is_integer("-1"));
    }

    /// Method is_integer must pass on zero.
    ///
    /// If provided zero, is_integer must return true.
    #[test]
    fn is_integer_zero() {
        assert!(is_integer("0"));
    }

    /// Method is_integer must fail on float.
    ///
    /// If provided a floating point number, is_integer must return false.
    #[test]
    fn is_integer_float() {
        assert!(!is_integer("1.5"));
    }

    /// Method is_integer must fail on a non number.
    ///
    /// If provided a non number token, is_integer must return false.
    #[test]
    fn is_integer_non_number() {
        assert!(!is_integer("a"));
    }

    /// Method is_negative must pass on negative number.
    ///
    /// If provided a negative number, is_negative must return true.
    #[test]
    fn is_negative_true() {
        assert!(is_negative("-1.5"));
    }

    /// Method is_negative must fail on zero.
    ///
    /// If provided zero, is_negative must return false.
    #[test]
    fn is_negative_zero() {
        assert!(!is_negative("0"));
    }

    /// Method is_negative must fail on negative number.
    ///
    /// If provided a negative, is_negative must return false.
    #[test]
    fn is_negative_negative() {
        assert!(!(is_negative("1.5")));
    }

    /// Method is_negative must fail on non_number.
    ///
    /// If provided a non number token, is_negative must return false.
    #[test]
    fn is_negative_non_number() {
        assert!(!is_negative("a"));
    }

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

    /// Method is_positive must pass on positive number.
    ///
    /// If provided a positive number, is_positive must return true.
    #[test]
    fn is_positive_true() {
        assert!(is_positive("1.5"));
    }

    /// Method is_positive must fail on zero.
    ///
    /// If provided zero, is_positive must return false.
    #[test]
    fn is_positive_zero() {
        assert!(!is_positive("0"));
    }

    /// Method is_positive must fail on negative number.
    ///
    /// If provided a negative, is_positive must return false.
    #[test]
    fn is_positive_negative() {
        assert!(!(is_positive("-1.5")));
    }

    /// Method is_positive must fail on non_number.
    ///
    /// If provided a non number token, is_positive must return false.
    #[test]
    fn is_positive_non_number() {
        assert!(!is_positive("a"));
    }
}
