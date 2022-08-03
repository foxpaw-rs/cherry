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

pub use action::Action;
pub use error::{Error, Result};
use std::collections::HashMap;

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
