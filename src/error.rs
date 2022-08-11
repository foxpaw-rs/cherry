//! Error Module.
//!
//! This module houses the Error type used in the library, and exports a
//! simplified type result. These types are to be used throughout the library
//! for error handling.

use core::fmt::{self, Display, Formatter};
use core::result;
use std::error::Error as StdError;

/// Error.
///
/// Typed error for the library. Utilised for all errors raised from this
/// library. Uses a provided String as the internal error message. Can be used
/// in a core::result::Result, however, for convenience a Result type is
/// provided in this module.
///
/// Example
/// ```rust
/// use cherry::{self, Error};
///
/// fn is_not_zero(number: u8) -> cherry::Result<()> {
///     if number == 0 { return Err(Error::new("Zero!")); }
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error {
    /// The error message.
    message: String,
}

impl Error {
    /// Create a new Error.
    ///
    /// Construct a new Error with the provided message.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Error;
    ///
    /// Error::new("Something went wrong...");
    /// ```
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
}

impl Display for Error {
    /// Format an Error for display.
    ///
    /// Formats the error for display and pretty printing.
    ///
    /// # Example
    /// ```
    /// use cherry::Error;
    ///
    /// let error = Error::new("Message");
    /// eprintln!("{}", error);
    /// ```
    ///
    /// # Error
    /// Will error if the underlying write macro fails.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl StdError for Error {}

/// Result.
///
/// This Result is a simple newtype for Result<T, E> which defaults the Error
/// arm to the library defined Error.
///
/// # Example
/// ```rust
/// use cherry::{self, Error};
///
/// fn is_not_zero(number: u8) -> cherry::Result<()> {
///     if number == 0 { return Err(Error::new("Zero!")); }
///     Ok(())
/// }
/// ```
pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {

    use super::*;

    /// Error::new must create as per struct initialisation.
    ///
    /// The new method on Error must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn error_new() {
        let expected = Error {
            message: String::from("Message"),
        };
        let actual = Error::new("Message");
        assert_eq!(expected, actual);
    }

    /// Error::fmt must display the Error.
    ///
    /// The Display trait provides a to_string method which allows testing that the
    /// Error formats correctly when displayed or converted to string types.
    #[test]
    fn error_fmt() {
        let expected = "Error: Message";
        let actual = Error::new("Message").to_string();

        assert_eq!(expected, actual);
    }
}
