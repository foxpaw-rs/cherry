//! Action module
//!
//! The Action module houses specifics surrounding the Action type. Actions are
//! the application specific building blocks that add functionality to your CLI
//! application.

use crate::error::{self, Error};
use core::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};

/// Action<T>.
///
/// Actions are the customised application specific commands. Actions are
/// defined and inserted into the Cherry CLI runner as a base Action, or
/// inserted into another Action as a child Action. Actions can be abstract,
/// wherein the action is simply a parent Action that houses child Actions.
/// In this instance, simply do not define an callback method.
///
/// Actions are generic over the type returned from the callback method. This
/// can be explicitly specified during creation or inferred if chaining a call
/// to the then method.
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
    /// The keyword to invoke this Action.
    pub keyword: String,

    /// The description for this Action.
    description: Option<String>,

    /// The callback method attached to the Action.
    then: Option<Box<dyn Fn(Request<T>) -> T>>,
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
    /// # Error
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

    /// Run this Action.
    ///
    /// Execute this Action's callback using the provided Request. The Request
    /// provided must have an Action reference to this Action. This method will
    /// usually be invoked through the Request, returned when parsing through
    /// the Cherry instance.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Request};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::new("my_action")?
    ///         .then(|_request| println!("Hello world!"));
    ///     let request = Request::new(&action);
    ///
    ///     action.run(request);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// This method will error if:
    /// * The Request does not hold a reference to this Action.
    /// * There is no then callback to run.
    ///
    /// If there is no then callback, the Error will be a help message on how to
    /// correctly use this Action (e.g. using child Actions).
    pub fn run(&self, request: Request<T>) -> error::Result<T> {
        if request.action != self {
            return Err(Error::new(&format!(
                "Cannot run request for action '{}' on action '{}'.",
                &request.action.keyword, &self.keyword
            )));
        }

        self.then
            .as_ref()
            .map_or_else(|| Err(Error::new("Todo: Help.")), |then| Ok(then(request)))
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
    /// # Error
    /// Will error if the underlying write macro fails.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Action {{ keyword: {:?}, description: {:?}, then: {:?} }}",
            self.keyword,
            self.description,
            self.then
                .as_ref()
                .map_or_else(|| None, |_| Some("fn(Request<T>) -> T"))
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
        self.keyword == other.keyword && self.description == other.description
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

/// Argument.
///
/// Arguments are the initial separated values parsed by the Cherry instance.
/// Arguments are consumed immediately after an Action is selected. If
/// Arguments have a filter method, this filter is run against the provided
/// value to determine if the provided value is valid, and therefore if the
/// command provided to the Cherry instance was valid.
///
/// # Example
/// Todo(Paul): Uncomment once Argument completed.
// /// ```rust
// /// use cherry::Action;
// ///
// /// fn main() -> cherry::Result<()> {
// ///     let action = Action::new("my_action")?
// ///         .push_argument(
// ///              Argument::new("greeting")
// ///                  .description("The greeting to display, must be hello.")
// ///                  .filter(|value| { value == "hello" })
// ///          )
// ///         .then(|result| -> String { result.arguments[0] });
// ///      let cherry = Cherry::new()
// ///          .insert(action)?;
// ///
// ///      // Will provide value "Hello"
// ///      cherry.parse_str("my_action hello")
// ///      Ok(())
// /// }
// /// ```
pub struct Argument {
    /// The Argument title for use in help text.
    pub title: String,

    /// The Argument description for use in help text.
    description: Option<String>,

    /// The filter to determine if the provided value is valid.
    filter: Option<Box<dyn Fn(&str) -> bool>>,
}

impl Argument {
    /// Create a new Argument.
    ///
    /// Create a new Argument instance.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Argument;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let argument = Argument::new("name")?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Will error when a blank (empty) title is provided. Arguments must have a
    /// non-empty title assigned to them.
    pub fn new(title: &str) -> error::Result<Self> {
        if title.is_empty() {
            return Err(Error::new("Argument must have a non-empty title."));
        }

        Ok(Self {
            description: None,
            filter: None,
            title: String::from(title),
        })
    }

    /// Update the description.
    ///
    /// The description of the Argument is used by the help text to assist users of
    /// the application to understand it. A good description text allows users to
    /// effectively use the application.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Argument;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let argument = Argument::new("my_argument")?
    ///        .description("The argument description");
    ///     Ok(())
    /// }
    /// ```
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(String::from(description));
        self
    }
}

impl Debug for Argument {
    /// Format an Argument for debug.
    ///
    /// Formats the Argument for debug printing.
    ///
    /// # Example
    /// ```
    /// use cherry::Argument;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let argument = Argument::new("argument")?;
    ///     println!("{:?}", argument);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Will error if the underlying write macro fails.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Argument {{ title: {:?}, description: {:?}, filter: {:?} }}",
            self.title,
            self.description,
            self.filter
                .as_ref()
                .map_or_else(|| None, |_| Some("fn(&str) -> bool"))
        )
    }
}

impl Eq for Argument {}

impl Ord for Argument {
    /// Ordering implementation.
    ///
    /// Defines how Arguments should be ordered using comparison operators.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Argument;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let first = Argument::new("a")?;
    ///     let last = Argument::new("z")?;
    ///     assert!(first < last);
    ///     Ok(())
    /// }
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.cmp(&other.title)
    }
}

impl PartialEq for Argument {
    /// Partial Equality implementation.
    ///
    /// Defines how Arguments should be considered equal.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Argument;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let first = Argument::new("a")?;
    ///     let last = Argument::new("a")?;
    ///     assert_eq!(first, last);
    ///     Ok(())
    /// }
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.description == other.description
    }
}

impl PartialOrd for Argument {
    /// Partial Ordering implementation.
    ///
    /// Defines how Arguments should be ordered using comparison operators.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Argument;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let first = Argument::new("a")?;
    ///     let last = Argument::new("z")?;
    ///     assert!(first < last);
    ///     Ok(())
    /// }
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.title.partial_cmp(&other.title)
    }
}

/// Request<T>.
///
/// Requests are the data structure parsed from a Cherry instance. Requests
/// hold the parsed data and are linked to the Action the application parsed
/// from. Typical interaction with Requests is to retrieve them from the Cherry
/// instance through parsing, before running the Action's callback method.
///
/// Requests are generic over the type expected to be returned from the Action.
/// This will be inferred when creating a Request object as the Action is
/// supplied.
///
/// # Example
/// ```rust
/// use cherry::{Action, Cherry};
///
/// fn main() -> cherry::Result<()> {
///     let cherry = Cherry::new()
///         .insert(
///             Action::new("my_action")?
///                 .then(|request| {
///                     // Do something...
///                 })
///         )?;
///     let request = cherry.parse_str("my_action")?;
///     request.run();
///     Ok(())
/// }
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

    /// Run the Request.
    ///
    /// Invoke the Action's run method, consuming this Request.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Request};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::new("my_action")?
    ///         .then(|_request| println!("Hello World"));
    ///     let request = Request::new(&action);
    ///
    ///     request.run();
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Will error if the underlying call to the Action's run method retuens an
    /// error.
    pub fn run(self) -> error::Result<T> {
        self.action.run(self)
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

    /// Action::run must correctly run the method.
    ///
    /// The run method must correctly call the Action's then callback with the
    /// provided Request.
    #[test]
    fn action_run() {
        let action = Action::new("my_action")
            .unwrap()
            .then(|_request| -> u8 { 1_u8 });

        let request = Request::new(&action);

        assert_eq!(1, action.run(request).unwrap());
    }

    /// Action::run must error on a reference mismatch.
    ///
    /// The run method must return an Error when the Result does not refernece the
    /// correct Action.
    #[test]
    fn action_run_reference_mismatch() {
        let action = Action::new("my_action")
            .unwrap()
            .then(|_request| -> u8 { 1_u8 });

        let ref_action = Action::new("my_ref_action")
            .unwrap()
            .then(|_request| -> u8 { 2_u8 });

        let request = Request::new(&ref_action);
        let error =
            Error::new("Cannot run request for action 'my_ref_action' on action 'my_action'.");

        assert_eq!(error, action.run(request).unwrap_err());
    }

    /// Action::run must error when no then callback.
    ///
    /// The run method must return an Error when the Action does not have a set
    /// then callback
    #[test]
    fn action_run_missing_then() {
        let action = Action::<()>::new("my_action").unwrap();

        let request = Request::new(&action);
        let error = Error::new("Todo: Help.");

        assert_eq!(error, action.run(request).unwrap_err());
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

    /// Action::fmt must debug the Action.
    ///
    /// The custom implementation of the Debug::fmt method must correctly display
    /// the Action.
    #[test]
    fn action_fmt() {
        let action = Action::new("action")
            .unwrap()
            .description("Action description.")
            .then(|_| {});
        let expected = "Action { keyword: \"action\", description: Some(\"Action description.\"), then: Some(\"fn(Request<T>) -> T\") }";
        let actual = format!("{:?}", action);

        assert_eq!(expected, actual);
    }

    /// Action::fmt must handle a missing Options.
    ///
    /// The custom implementation of the Debug::fmt method must correctly display
    /// the Action when all Options are None.
    #[test]
    fn action_fmt_missing_options() {
        let action = Action::<()>::new("action").unwrap();
        let expected = "Action { keyword: \"action\", description: None, then: None }";
        let actual = format!("{:?}", action);

        assert_eq!(expected, actual);
    }

    /// Argument::new must create as per struct initialisation.
    ///
    /// The new method on Argument must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn argument_new() {
        let expected = Argument {
            description: None,
            filter: None,
            title: String::from("Title"),
        };
        let actual = Argument::new("Title").unwrap();

        assert_eq!(expected, actual);
    }

    /// Argument::new must error on empty title.
    ///
    /// The new method must correctly error when provided with an empty title
    /// during initialisation.
    #[test]
    fn argument_new_empty() {
        let expected = Error::new("Argument must have a non-empty title.");
        let actual = Argument::new("");

        assert_eq!(expected, actual.unwrap_err());
    }

    /// Argument::description must correctly set the description.
    ///
    /// The description method must correctly set the internal Argument description
    /// to the provided text.
    #[test]
    fn argument_description() {
        let argument = Argument::new("my_argument")
            .unwrap()
            .description("My description.");

        assert_eq!(Some(String::from("My description.")), argument.description);
    }

    // Todo(Paul): When Argument complete.
    // /// Argument::fmt must debug the Argument.
    // ///
    // /// The custom implementation of the Debug::fmt method must correctly display
    // /// the Argument.
    // #[test]
    // fn argument_fmt() {
    //     let argument = Argument::new("argument")
    //         .unwrap()
    //         .description("Argument description.")
    //         .filter(|_| {});
    //     let expected = "Argument { title: \"argument\", description: Some(\"Argument description.\"), filter: Some(\"fn(&str) -> bool\") }";
    //     let actual = format!("{:?}", argument);

    //     assert_eq!(expected, actual);
    // }

    /// Argument::fmt must handle a missing Options.
    ///
    /// The custom implementation of the Debug::fmt method must correctly display
    /// the Argument when all Options are None.
    #[test]
    fn argument_fmt_missing_options() {
        let argument = Argument::new("argument").unwrap();
        let expected = "Argument { title: \"argument\", description: None, filter: None }";
        let actual = format!("{:?}", argument);

        assert_eq!(expected, actual);
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

    /// Request::run must run the Action's then callback.
    ///
    /// The run method on Request must run the Action's callback it references.
    #[test]
    fn request_run() {
        let action = Action::new("my_action")
            .unwrap()
            .then(|_request| -> u8 { 1_u8 });
        let request = Request::new(&action);

        assert_eq!(1, request.run().unwrap());
    }
}
