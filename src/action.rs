//! Action module
//!
//! The Action module houses specifics surrounding the Action type. Actions are
//! the application specific building blocks that add functionality to your CLI
//! application.

use crate::error::{self, Error};

/// Action.
///
/// Actions are the customised application specific commands. Actions are
/// defined and inserted into the Cherry CLI runner as a base Action, or
/// inserted into another Action as a child Action. Actions can be abstract,
/// wherein the action is simply a parent Action that houses child Actions.
/// In this instance, simply do not define an execute method.
///
/// Actions house three types of data:
/// * Arguments;
/// * Options; and
/// * Flags.
///
/// Arguments are the values that follow the execution word, e.g.
/// `my_action arg0 arg1 arg2`. Arguments can have validation filters using the
/// filter command on the Argument. If the incorrect number of arguments are
/// supplied to the Action, an error message will be returned.
///
/// Options are optional arguments. Options are invoked either using the long
/// name, or an optionally defined single character short name, followed by the
/// option value e.g. `my_action --option value -o v`. Options can also utilise
/// validation filters.
///
/// Flags are booleans that can be set for the Action. Flags can be invoked
/// similar to Options, by either using the long name, or the optional defined
/// short name. Flags do not take an option value. Flags using the short name
/// can be combined e.g `my_action --verbose -a -bcd`.
///
/// # Example: Create an action.
/// ```rust
/// // Todo(Paul): When actions have been completed.
/// ```
///
/// # Exmaple: Parent and child actions.
/// ```rust
/// // Todo(Paul): When actions have parent-child relationships.
/// ```
///
/// # Exmaple: Abstract parent action.
/// ```rust
/// // Todo(Paul): When actions have parent-child relationships.
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Action {
    /// The keyword to invoke this Action.
    pub keyword: String,
}

impl Action {
    /// Create a new Action.
    ///
    /// Create a new Action instance.
    ///
    /// # Examples
    /// ```rust
    /// use cherry::Action;
    ///
    /// let action = Action::new("my_action");
    /// ```
    pub fn new(keyword: &str) -> error::Result<Action> {
        if keyword.is_empty() {
            return Err(Error::new("Action must have a non-empty keyword."));
        }

        Ok(Action {
            keyword: String::from(keyword),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Action::new must create as per struct initialisation.
    ///
    /// The new method on Action must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn action_new() {
        let expected = Action {
            keyword: String::from("my_action"),
        };
        let actual = Action::new("my_action").unwrap();

        assert_eq!(expected, actual);
    }

    /// Action::new must error on empty keyword.
    ///
    /// The new method must correctly error when provided with an empty keyword
    /// during initialisation.
    #[test]
    fn action_new_empty() {
        let expected = Error::new("Action must have a non-empty keyword.");
        let actual = Action::new("");

        assert_eq!(expected, actual.unwrap_err());
    }
}
