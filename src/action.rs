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
/// In this instance, simply do not define an callback method.
///
/// Actions house three types of data:
///
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

    /// The description for this Action.
    pub description: Option<String>,
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
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::new("my_action")?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Will error when a blank (empty) keyword is provided. Actions must have a
    /// non-empty keyword assigned to them.
    pub fn new(keyword: &str) -> error::Result<Action> {
        if keyword.is_empty() {
            return Err(Error::new("Action must have a non-empty keyword."));
        }

        Ok(Action {
            keyword: String::from(keyword),
            description: None,
        })
    }

    /// Update the description.
    ///
    /// The description of the Action is used by the help text to assist users of
    /// the application to understand it. A good description text allows users to
    /// effectively use the application.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Action;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::new("my_action")?
    ///        .description("The action description");
    ///     Ok(())
    /// }
    /// ```
    pub fn description(mut self, description: &str) -> Action {
        self.description = Some(String::from(description));
        self
    }
}

/// Request.
///
/// Requests are the data structure loaded from a Cherry instance. Requests
/// hold the loaded data and are linked to the Action the application loaded
/// from. Typical interaction with Requests is to retrieve them from the Cherry
/// instance through loading, before running the Action's callback method.
///
/// # Example: Running a loaded request.
/// ```rust
/// // Todo(Paul): When actions have a callback.
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Request<'a> {
    /// The Action this Request is bound to.
    action: &'a Action,
}

impl<'a> Request<'a> {
    /// Create a new Request.
    ///
    /// Create a new Request instance.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Request};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::new("my_action")?;
    ///     let cherry = Request::new(&action);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(action: &'a Action) -> Self {
        Self { action }
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
            description: None,
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

    /// Action::description must correctly set the description.
    ///
    /// The description method must correctly set the internal Action description
    /// to the provided text.
    #[test]
    fn action_description() {
        let expected = Action {
            keyword: String::from("my_action"),
            description: Some(String::from("My description.")),
        };
        let actual = Action::new("my_action")
            .unwrap()
            .description("My description.");

        assert_eq!(expected, actual);
    }

    /// Request::new must create as per struct initialisation.
    ///
    /// The new method on Request must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn request_new() {
        let action = Action::new("my_action").unwrap();
        let expected = Request { action: &action };
        let actual = Request::new(&action);

        assert_eq!(expected, actual);
    }
}
