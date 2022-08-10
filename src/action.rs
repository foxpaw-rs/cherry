//! Action module
//!
//! The Action module houses specifics surrounding the Action type. Actions are
//! the application specific building blocks that add functionality to your CLI
//! application.

use crate::error::{self, Error};
use core::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};

/// Action.
///
/// Actions are the customised application specific commands. Actions are
/// defined and inserted into the Cherry CLI runner as a base Action, or
/// inserted into another Action as a child Action. Actions can be abstract,
/// wherein the action is simply a parent Action that houses child Actions.
/// In this instance, simply do not define an callback method.
///
/// Actions hold data within:
///
/// * Arguments.
/// * Options.
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
/// # Example
/// ## Create an action
/// ```rust
/// // Todo(Paul): When actions have been completed.
/// ```
///
/// ## Parent and child actions
/// ```rust
/// // Todo(Paul): When actions have parent-child relationships.
/// ```
///
/// ## Abstract parent action
/// ```rust
/// // Todo(Paul): When actions have parent-child relationships.
/// ```
pub struct Action<T> {
    /// The description for this Action.
    pub description: Option<String>,

    /// The keyword to invoke this Action.
    pub keyword: String,

    /// The callback method attached to the Action.
    pub then: Option<Box<dyn Fn(Request<T>) -> T>>,
}

impl<T> Action<T> {
    /// Create a new Action.
    ///
    /// Create a new Action instance.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Action;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::<()>::new("my_action")?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Will error when a blank (empty) keyword is provided. Actions must have a
    /// non-empty keyword assigned to them.
    pub fn new(keyword: &str) -> error::Result<Self> {
        if keyword.is_empty() {
            return Err(Error::new("Action must have a non-empty keyword."));
        }

        Ok(Action {
            description: None,
            keyword: String::from(keyword),
            then: None,
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
    ///     let action = Action::<()>::new("my_action")?
    ///        .description("The action description");
    ///     Ok(())
    /// }
    /// ```
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(String::from(description));
        self
    }

    /// Update the then callback on the Action.
    ///
    /// The then callback of the Action is the method or closure that is called
    /// when this action is parsed from the input.
    ///
    /// # Example
    /// ## Using a method
    /// ```rust
    /// use cherry::{Action, Request};
    ///
    /// fn hello(request: Request<()>) {
    ///     println!("Hello World");
    /// }
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::<()>::new("my_action")?
    ///         .then(hello);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Using a closure
    /// ```rust
    /// use cherry::Action;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::<()>::new("my_action")?
    ///         .then(|request| {
    ///             // Implement application logic.
    ///         });
    ///     Ok(())
    /// }
    /// ```
    pub fn then(mut self, then: impl Fn(Request<T>) -> T + 'static) -> Self {
        self.then = Some(Box::new(then));
        self
    }
}

impl<T> Debug for Action<T> {
    /// Format an Action for debug.
    ///
    /// Formats the Action for debug printing.
    ///
    /// # Example
    /// ```
    /// use cherry::Action;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::<()>::new("action")?;
    ///     println!("{:?}", action);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Will error if the underlying write macro fails.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Action (\n\tkeyword: {},\n\t description: {:?}",
            self.keyword, self.description
        )
    }
}

impl<T> Eq for Action<T> {}

impl<T> Ord for Action<T> {
    /// Ordering implementation.
    ///
    /// Defines how Actions should be ordered using comparison operators.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Action;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let first = Action::<()>::new("a")?;
    ///     let last = Action::<()>::new("z")?;
    ///     assert!(first < last);
    ///     Ok(())
    /// }
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.keyword.cmp(&other.keyword)
    }
}

impl<T> PartialEq for Action<T> {
    /// Partial Equality implementation.
    ///
    /// Defines how Actions should be considered equal.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Action;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let first = Action::<()>::new("a")?.description("desc");
    ///     let last = Action::<()>::new("a")?.description("desc");
    ///     assert_eq!(first, last);
    ///     Ok(())
    /// }
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description && self.keyword == other.keyword
    }
}

impl<T> PartialOrd for Action<T> {
    /// Partial Ordering implementation.
    ///
    /// Defines how Actions should be ordered using comparison operators.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Action;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let first = Action::<()>::new("a")?;
    ///     let last = Action::<()>::new("z")?;
    ///     assert!(first < last);
    ///     Ok(())
    /// }
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.keyword.partial_cmp(&other.keyword)
    }
}

/// Request.
///
/// Requests are the data structure parsed from a Cherry instance. Requests
/// hold the parsed data and are linked to the Action the application parsed
/// from. Typical interaction with Requests is to retrieve them from the Cherry
/// instance through parsing, before running the Action's callback method.
///
/// # Example
/// ```rust
/// // Todo(Paul): When actions have a callback.
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Request<'a, T> {
    /// The Action this Request is bound to.
    action: &'a Action<T>,
}

impl<'a, T> Request<'a, T> {
    /// Create a new Request.
    ///
    /// Create a new Request instance.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Request};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::<()>::new("my_action")?;
    ///     let cherry = Request::new(&action);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(action: &'a Action<T>) -> Self {
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
            description: None,
            keyword: String::from("my_action"),
            then: None,
        };
        let actual = Action::<()>::new("my_action").unwrap();

        assert_eq!(expected, actual);
    }

    /// Action::new must error on empty keyword.
    ///
    /// The new method must correctly error when provided with an empty keyword
    /// during initialisation.
    #[test]
    fn action_new_empty() {
        let expected = Error::new("Action must have a non-empty keyword.");
        let actual = Action::<()>::new("");

        assert_eq!(expected, actual.unwrap_err());
    }

    /// Action::description must correctly set the description.
    ///
    /// The description method must correctly set the internal Action description
    /// to the provided text.
    #[test]
    fn action_description() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .description("My description.");

        assert_eq!(Some(String::from("My description.")), action.description);
    }

    /// Action::then must correctly set the then callback with a closure.
    ///
    /// The then method must correctly set the internal Action then callback when
    /// passed a closure.
    #[test]
    fn action_then_closure() {
        let text = "Hello World!";
        let action = Action::new("my_action")
            .unwrap()
            .then(move |_request: Request<()>| println!("{}", text));

        assert!(action.then.is_some());
    }

    /// Action::then must correctly set the then callback with a method.
    ///
    /// The then method must correctly set the internal Action then callback when
    /// passed a method.
    #[test]
    fn action_then_method() {
        fn callback(_request: Request<()>) -> () {
            println!("Hello World!");
        }
        let action = Action::new("my_action").unwrap().then(callback);

        assert!(action.then.is_some());
    }

    /// Request::new must create as per struct initialisation.
    ///
    /// The new method on Request must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn request_new() {
        let action = Action::<()>::new("my_action").unwrap();
        let expected = Request { action: &action };
        let actual = Request::new(&action);

        assert_eq!(expected, actual);
    }
}
