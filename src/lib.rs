//! Todo(Paul): Crate Documentation once 0.1 complete.
//! * Include a note about the crate examples requiring generic specifiers, what
//!   it means, and how to avoid it when implementing
//!
//! # Usage
//!
//! # Examples
//! ## Setup actions
//! ```rust
//! ```
//!
//! ## Run action
//! Using the actions set up from the above example.
//! ```rust
//! ```
//!
//! ## Using the action to have input on the application state
//! ```rust
//! ```
//!
//! For further detailed examples, refer to the documentation which contain
//! some tutorial applications using Cherry.

#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

pub mod action;
pub mod error;

pub use action::{Action, Argument, Request};
pub use error::{Error, Result};
use std::cmp::Eq;
use std::collections::HashMap;
use std::env::Args;
use std::hash::Hash;

/// Cherry.
///
/// The Cherry structure is the CLI application runner. A Cherry object is
/// created, Actions are pushed onto it and then the CLI is run through it.
/// Usually, only one Cherry object is required and the pushed Action objects
/// form a tree, navigated by the Cherry.
///
/// Cherry is generic over the type returned by the Actions. This can be
/// explicitly defined if desired, or inferred if insert calls are chained on
/// object creation.
///
/// # Example
/// ```rust
/// use cherry::{Action, Cherry};
///
/// fn main() -> cherry::Result<()> {
///     let cherry = Cherry::<()>::new()
///         .insert(Action::new("my_action")?)?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Cherry<T> {
    /// The available actions inserted into the Cherry instance.
    actions: HashMap<String, Action<T>>,
}

impl<T> Cherry<T> {
    /// Create a new Cherry.
    ///
    /// Create a new Cherry instance.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Cherry;
    ///
    /// let cherry = Cherry::<()>::new();
    /// ```
    pub fn new() -> Self {
        Cherry {
            actions: HashMap::new(),
        }
    }

    /// Insert onto the Cherry object.
    ///
    /// Insert an Action onto the Cherry object.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Cherry};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let cherry = Cherry::<()>::new()
    ///         .insert(Action::new("my_action")?)?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Errors occur if attempting to insert an Action with a blank (empty)
    /// keyword. Will also error if a collision occurs when attempting to insert.
    pub fn insert(mut self, action: Action<T>) -> Result<Self> {
        if action.keyword.is_empty() {
            return Err(Error::new("Action must have a non-empty keyword."));
        }

        if self.actions.contains_key(&action.keyword) {
            return Err(Error::new(&format!(
                "Action \'{}\' already exists.",
                &action.keyword
            )));
        }

        self.actions.insert(action.keyword.clone(), action);
        Ok(self)
    }

    /// Load the command into Cherry.
    ///
    /// The parse command takes an Iterator of String types. This is parsed into the
    /// Cherry object, and returns an Action if the command matches an Action
    /// keyword or an Error if not. Most commonly used with environment args.
    ///
    /// # Example
    /// ## Load a command
    /// ```rust
    /// use cherry::{Action, Cherry};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let mut cherry = Cherry::<()>::new()
    ///         .insert(Action::new("my_action")?)?;
    ///
    ///     // Usually, obtain arguments either from the environment or stdio.
    ///     let args = ["my_action"].into_iter();
    ///     let request = cherry.parse(args)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Example
    /// ## Arguments, Fields and Flags
    /// Todo(Paul): Update with fields and Flags as supported.
    /// ```rust
    /// use cherry::{Action, Argument, Cherry};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let mut cherry = Cherry::<()>::new()
    ///         .insert(
    ///             Action::new("my_action")?
    ///                 .insert_argument(
    ///                     Argument::new("my_argument")?
    ///                         .description("My argument, must be longer than 3 characters.")
    ///                         .filter(|value| -> bool { value.len() > 3 })
    ///                  )?
    ///                 .then(|_| println!("Hello World!"))
    ///         )?;
    ///
    ///     // Usually, obtain arguments either from the environment or stdio.
    ///     let args = ["my_action", "my_argument_value"].into_iter();
    ///     let request = cherry.parse(args)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Will error if no Action is found matching the command through:
    ///
    /// * Unknown keyword.
    /// * Incorrect number of arguments.
    /// * Unknown option or flag.
    /// * Validation rule failure.
    ///
    /// Upon erroring while parsing, the most relevant help text will be returned.
    /// In the event of a unknown keyword, the help text for the parent will be
    /// given, in all other cases the help text for the located Action will be
    /// provided.
    pub fn parse<C, D>(&self, mut command: C) -> Result<Request<T>>
    where
        C: Iterator<Item = D>,
        D: AsRef<str> + Eq + Hash,
    {
        // Select the Action.
        let keyword = command.next().ok_or_else(|| Error::new("Todo: Help."))?;
        let action = self
            .actions
            .get(keyword.as_ref())
            .ok_or_else(|| Error::new("Todo: Help."))?;
        let mut request = Request::new(action);

        // Obtain Arguments.
        while let Some(value) = command.next() {
            request = request.insert_argument(value.as_ref())?;
        }

        // Validate the Request.
        match request.validate() {
            true => Ok(request),
            false => Err(Error::new("Todo: Help.")),
        }
    }

    /// Load the command into Cherry from command line arguments.
    ///
    /// Preferable method when parsing from command line arguments. Handles the
    /// first argument being the executable before passing off to the parse method
    /// for processing.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Cherry};
    /// use std::env;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let mut cherry = Cherry::<()>::new()
    ///         .insert(Action::new("my_action")?)?;
    ///
    ///     # | | -> cherry::Result<()> {
    ///     let request = cherry.parse_args(env::args())?;
    ///     # Ok(())
    ///     # };
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Will error if the underlying parse method errors.
    pub fn parse_args(&self, mut command: Args) -> Result<Request<T>> {
        command.next();
        self.parse(command)
    }

    /// Load the command into Cherry from a slice.
    ///
    /// Helper method when wanting to parse command arguments from a slice of str.
    /// Simply passes through to the parse method.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Cherry};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let mut cherry = Cherry::<()>::new()
    ///         .insert(Action::new("my_action")?)?;
    ///
    ///     let request = cherry.parse_slice(&["my_action"])?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Will error if the underlying parse method errors.
    pub fn parse_slice(&self, command: &[&str]) -> Result<Request<T>> {
        self.parse(command.iter())
    }

    /// Load the command into Cherry from a str.
    ///
    /// Helper method when wanting to parse command arguments from a str slice.
    /// Simply passes through to the parse method. Commonly used in a CLI
    /// application with user input from stdio.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Cherry};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let mut cherry = Cherry::<()>::new()
    ///         .insert(Action::new("my_action")?)?;
    ///
    ///     let request = cherry.parse_str("my_action")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Will error if the underlying parse method errors.
    pub fn parse_str(&self, command: &str) -> Result<Request<T>> {
        self.parse(command.split(' '))
    }
}

impl Default for Cherry<()> {
    /// Create a new Cherry.
    ///
    /// Create a new Cherry instance. Note that this is identical to the new
    /// method.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Cherry;
    ///
    /// let cherry = Cherry::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Cherry::new must create as per struct initialisation.
    ///
    /// The new method on Cherry must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn cherry_new() {
        let expected = Cherry {
            actions: HashMap::new(),
        };
        let actual = Cherry::<()>::new();
        assert_eq!(expected, actual);
    }

    /// Cherry::default must create as per struct initialisation.
    ///
    /// The default method on Cherry must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn cherry_default() {
        let expected = Cherry {
            actions: HashMap::new(),
        };
        let actual = Cherry::<()>::default();
        assert_eq!(expected, actual);
    }

    /// Cherry::parse must correctly parse a Request
    ///
    /// The parse method must correctly parse a Request, linked to the correctly
    /// selected Action type.
    #[test]
    fn cherry_parse() {
        let cherry = Cherry::<()>::new()
            .insert(Action::new("my_action").unwrap())
            .unwrap();

        let expected = Request::new(&cherry.actions.get("my_action").unwrap());

        let actual = cherry.parse(["my_action"].into_iter()).unwrap();

        assert_eq!(expected, actual);
    }

    /// Cherry::parse must error when no Actions.
    ///
    /// The parse method must error when no Actions are parsed into the Cherry
    /// object.
    #[test]
    fn cherry_parse_empty_actions() {
        let expected = Error::new("Todo: Help.");
        let actual = Cherry::<()>::new()
            .parse(["my_action"].into_iter())
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Cherry::parse must error when no command.
    ///
    /// The parse method must error when no command is provided when parsing the
    /// Cherry object.
    #[test]
    fn cherry_parse_empty_command() {
        let args: [&str; 0] = [];
        let expected = Error::new("Todo: Help.");
        let actual = Cherry::<()>::new()
            .insert(Action::new("my_action").unwrap())
            .unwrap()
            .parse(args.into_iter())
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Cherry::parse must correctly parse Arguments.
    ///
    /// The parse method must correctly Parse a Request, linked to the correctly
    /// selected Action type, and parse out the Arguments.
    #[test]
    fn cherry_parse_argument() {
        let cherry = Cherry::<()>::new()
            .insert(
                Action::new("my_action")
                    .unwrap()
                    .insert_argument(Argument::new("my_argument").unwrap())
                    .unwrap(),
            )
            .unwrap();

        let expected = Request::new(&cherry.actions.get("my_action").unwrap())
            .insert_argument("first")
            .unwrap();
        let actual = cherry.parse(["my_action", "first"].into_iter()).unwrap();

        assert_eq!(expected, actual);
    }

    /// Cherry::parse must error if too many Arguments supplied.
    ///
    /// The parse method error if too many Arguments supplied to the Action.
    #[test]
    fn cherry_parse_argument_overflow() {
        let cherry = Cherry::<()>::new()
            .insert(
                Action::new("my_action")
                    .unwrap()
                    .insert_argument(Argument::new("my_argument").unwrap())
                    .unwrap(),
            )
            .unwrap();

        let expected = Error::new("Todo: Help.");
        let actual = cherry
            .parse(["my_action", "first", "second"].into_iter())
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Cherry::parse must error if too few Arguments supplied.
    ///
    /// The parse method error if too few Arguments supplied to the Action.
    #[test]
    fn cherry_parse_argument_underflow() {
        let cherry = Cherry::<()>::new()
            .insert(
                Action::new("my_action")
                    .unwrap()
                    .insert_argument(Argument::new("my_argument").unwrap())
                    .unwrap(),
            )
            .unwrap();

        let expected = Error::new("Todo: Help.");
        let actual = cherry.parse(["my_action"].into_iter()).unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Cherry::parse_slice must correctly parse a Request
    ///
    /// The parse_slice method must correctly parse a Request, linked to the
    /// correctly selected Action type.
    #[test]
    fn cherry_parse_slice() {
        let cherry = Cherry::<()>::new()
            .insert(Action::new("my_action").unwrap())
            .unwrap();

        let expected = Request::new(&cherry.actions.get("my_action").unwrap());
        let actual = cherry.parse_slice(&["my_action"]).unwrap();

        assert_eq!(expected, actual);
    }

    /// Cherry::parse_slice must error when no command.
    ///
    /// The parse_slice method must error when no command is provided when parsing
    /// the Cherry object.
    #[test]
    fn cherry_parse_slice_empty_command() {
        let expected = Error::new("Todo: Help.");
        let actual = Cherry::<()>::new()
            .insert(Action::new("my_action").unwrap())
            .unwrap()
            .parse_slice(&[""])
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Cherry::parse_str must correctly parse a Request
    ///
    /// The parse_str method must correctly parse a Request, linked to the correctly
    /// selected Action type.
    #[test]
    fn cherry_parse_str() {
        let cherry = Cherry::<()>::new()
            .insert(Action::new("my_action").unwrap())
            .unwrap();

        let expected = Request::new(&cherry.actions.get("my_action").unwrap());
        let actual = cherry.parse_str("my_action").unwrap();

        assert_eq!(expected, actual);
    }

    /// Cherry::parse_str must error when no command.
    ///
    /// The parse_str method must error when no command is provided when parsing the
    /// Cherry object.
    #[test]
    fn cherry_parse_str_empty_command() {
        let expected = Error::new("Todo: Help.");
        let actual = Cherry::<()>::new()
            .insert(Action::new("my_action").unwrap())
            .unwrap()
            .parse_str("")
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Cherry::insert must insert an Action.
    ///
    /// The insert method must correctly insert an Action into the internal
    /// HashMap, using the keyword of the Action as the HashMap key.
    #[test]
    fn cherry_insert() {
        let mut map = HashMap::new();
        map.insert(
            String::from("my_action"),
            Action::<()>::new("my_action").unwrap(),
        );

        let mut expected = Cherry::new();
        expected.actions = map;

        let actual = Cherry::new()
            .insert(Action::new("my_action").unwrap())
            .unwrap();

        assert_eq!(expected, actual);
    }

    /// Cherry::insert must error with empty Action keyword.
    ///
    /// The insert method must error when attempting to insert an Action with an
    /// empty string keyword.
    #[test]
    fn cherry_insert_empty() {
        let expected = Error::new("Action must have a non-empty keyword.");

        let cherry = Cherry::<()>::new();
        let mut action = Action::new("action").unwrap();
        action.keyword = String::from("");
        let actual = cherry.insert(action);

        assert_eq!(expected, actual.unwrap_err());
    }

    /// Cherry::insert must error when a collision occurs.
    ///
    /// The insert method must error when attempting to insert an Action with a
    /// duplicate keyword.
    #[test]
    fn cherry_insert_collision() {
        let expected = Error::new("Action \'my_action\' already exists.");
        let cherry = Cherry::<()>::new();
        let actual = cherry
            .insert(Action::new("my_action").unwrap())
            .unwrap()
            .insert(Action::new("my_action").unwrap())
            .unwrap_err();

        assert_eq!(expected, actual);
    }
}
