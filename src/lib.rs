//! Todo(Paul): Crate Documentation once 0.1 complete.
//!
//! # Usage
//!
//! # Example
//! ```rust
//! ```

#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

pub mod action;
pub mod error;

pub use action::{Action, Request};
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
/// # Example
/// ```rust
/// use cherry::{self, Action, Cherry};
///
/// fn init_cherry() -> cherry::Result<Cherry> {
///     let cherry = Cherry::new()
///         .insert(Action::new("my_action")?)?;
///     Ok(cherry)
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cherry {
    /// The available actions inserted into the Cherry instance.
    actions: HashMap<String, Action>,
}

impl Cherry {
    /// Create a new Cherry.
    ///
    /// Create a new Cherry instance.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Cherry;
    ///
    /// let cherry = Cherry::new();
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
    /// use cherry::{self, Action, Cherry};
    ///
    /// fn init_cherry() -> cherry::Result<Cherry> {
    ///     let cherry = Cherry::new()
    ///         .insert(Action::new("my_action")?)?;
    ///     Ok(cherry)
    /// }
    /// ```
    ///
    /// # Errors
    /// Errors occur if attempting to insert an action with a blank (empty)
    /// keyword. Will also error if a collision occurs when attempting to insert.
    pub fn insert(mut self, action: Action) -> Result<Cherry> {
        if action.keyword.is_empty() {
            return Err(Error::new("Action must have a non-empty keyword."));
        }

        if self.actions.contains_key(&action.keyword) {
            return Err(Error::new(
                &(String::from("Key \'") + &action.keyword + "\' already exists."),
            ));
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
    /// # Example: Load a command
    /// ```rust
    /// use cherry::{Action, Cherry};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let mut cherry = Cherry::new()
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
    /// # Example: Load command with arguments
    /// Todo(Paul): Implement example with arguments once supported.
    ///
    /// # Errors
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
    pub fn parse<T, U>(&self, mut command: T) -> Result<Request>
    where
        T: Iterator<Item = U>,
        U: AsRef<str> + Eq + Hash,
    {
        let keyword = command.next().ok_or_else(|| Error::new("Todo: Help."))?;
        let action = self
            .actions
            .get(keyword.as_ref())
            .ok_or_else(|| Error::new("Todo: Help."))?;

        Ok(Request::new(action))
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
    ///     let mut cherry = Cherry::new()
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
    /// # Errors
    /// Will error if the underlying parse method errors.
    pub fn parse_args(&self, mut command: Args) -> Result<Request> {
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
    ///     let mut cherry = Cherry::new()
    ///         .insert(Action::new("my_action")?)?;
    ///
    ///     let request = cherry.parse_slice(&["my_action"])?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Will error if the underlying parse method errors.
    pub fn parse_slice(&self, command: &[&str]) -> Result<Request> {
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
    ///     let mut cherry = Cherry::new()
    ///         .insert(Action::new("my_action")?)?;
    ///
    ///     let request = cherry.parse_str("my_action")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Will error if the underlying parse method errors.
    pub fn parse_str(&self, command: &str) -> Result<Request> {
        self.parse(command.split(' '))
    }
}

impl Default for Cherry {
    /// Create a new Cherry.
    ///
    /// Create a new Cherry instance. Note that this is identical to the new
    /// method.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Cherry;
    ///
    /// let cherry = Cherry::new();
    /// ```
    fn default() -> Self {
        Cherry::new()
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
        let actual = Cherry::new();
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
        let actual = Cherry::default();
        assert_eq!(expected, actual);
    }

    /// Cherry::parse must correctly parse a Request
    ///
    /// The parse method must correctly parse a Request, linked to the correctly
    /// selected Action type.
    #[test]
    fn cherry_parse() {
        let cherry = Cherry::new()
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
        let actual = Cherry::new().parse(["my_action"].into_iter()).unwrap_err();

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
        let actual = Cherry::new()
            .insert(Action::new("my_action").unwrap())
            .unwrap()
            .parse(args.into_iter())
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Cherry::parse_slice must correctly parse a Request
    ///
    /// The parse_slice method must correctly parse a Request, linked to the
    /// correctly selected Action type.
    #[test]
    fn cherry_parse_slice() {
        let cherry = Cherry::new()
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
        let actual = Cherry::new()
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
        let cherry = Cherry::new()
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
        let actual = Cherry::new()
            .insert(Action::new("my_action").unwrap())
            .unwrap()
            .parse_str("")
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Cherry::insert must insert an Action.
    ///
    /// The insert method must correctly insert an Action into the internal
    /// hashmap, using the keyword of the Action as the hashmap key.
    #[test]
    fn cherry_insert() {
        let mut map = HashMap::new();
        map.insert(String::from("my_action"), Action::new("my_action").unwrap());
        let expected = Cherry { actions: map };

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

        let cherry = Cherry::new();
        let actual = cherry.insert(Action {
            keyword: String::from(""),
            description: None,
        });

        assert_eq!(expected, actual.unwrap_err());
    }

    /// Cherry::insert must error when a collision occurs.
    ///
    /// The insert method must error when attempting to insert an Action with a
    /// duplicate keyword.
    #[test]
    fn cherry_insert_collision() {
        let expected = Error::new("Key \'my_action\' already exists.");
        let cherry = Cherry::new();
        let actual = cherry
            .insert(Action::new("my_action").unwrap())
            .unwrap()
            .insert(Action::new("my_action").unwrap())
            .unwrap_err();

        assert_eq!(expected, actual);
    }
}
