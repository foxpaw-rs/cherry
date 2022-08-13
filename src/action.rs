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

    /// The Arguments this Action accepts.
    arguments: Vec<Argument>,

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
            keyword: String::from(keyword),
            description: None,
            arguments: Vec::new(),
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

    /// Insert an Argument into the Action.
    ///
    /// Insert an Argument onto the Action object.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Argument};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::<()>::new("my_action")?
    ///         .insert_argument(Argument::new("my_argument")?)?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Errors occur if attempting to insert an Argument with a blank (empty)
    /// title.
    pub fn insert_argument(mut self, argument: Argument) -> error::Result<Self> {
        if argument.title.is_empty() {
            return Err(Error::new("Argument must have a non-empty title."));
        }

        self.arguments.push(argument);
        Ok(self)
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
    /// use cherry::{Action, Cherry};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::new("my_action")?
    ///         .then(|_request| println!("Hello World"));
    ///     let cherry = Cherry::new()
    ///         .insert(
    ///             Action::new("my_action")?
    ///                 .then(|_request| println!("Hello World"))
    ///         )?;
    ///     let request = cherry.parse_str("my_action")?;
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
    /// when this Action is parsed from the input.
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
        match self.then {
            Some(_) => write!(
                f,
                "Action {{ keyword: {:?}, description: {:?}, arguments: {:?}, then: Some(fn(Request<T>) -> T) }}",
                self.keyword,
                self.description,
                self.arguments
            ),
            None => write!(
                f,
                "Action {{ keyword: {:?}, description: {:?}, arguments: {:?}, then: None }}",
                self.keyword,
                self.description,
                self.arguments
            ),
        }
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
/// ```rust
/// use cherry::{Action, Argument, Cherry};
///
/// fn main() -> cherry::Result<()> {
///     let cherry = Cherry::new()
///         .insert(
///             Action::new("my_action")?
///                 .insert_argument(
///                     Argument::new("greeting")?
///                         .description("The greeting to display, must be hello.")
///                         .filter(|value| { value == "hello" })
///                 )?
///                 .then(|result| -> Option<String> { result.get_argument(0).cloned() })
///         )?;
///
///      // Will provide value "Hello"
///      cherry.parse_str("my_action hello");
///      Ok(())
/// }
/// ```
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
            title: String::from(title),
            description: None,
            filter: None,
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

    /// Update the filter callback on the Argument.
    ///
    /// The filter callback of the Argument is the method or closure that is called
    /// when this Argument is parsed from the input to determine if the input is
    /// valid.
    ///
    /// # Example
    /// ## Using a method
    /// ```rust
    /// use cherry::{Argument};
    ///
    /// fn is_valid(value: &str) -> bool {
    ///     value == "Hello"
    /// }
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Argument::new("my_action")?
    ///         .filter(is_valid);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Using a closure
    /// ```rust
    /// use cherry::Argument;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Argument::new("my_action")?
    ///         .filter(|val: &str| -> bool {
    ///             // Implement application logic.
    ///             true
    ///         });
    ///     Ok(())
    /// }
    /// ```
    pub fn filter(mut self, filter: impl Fn(&str) -> bool + 'static) -> Self {
        self.filter = Some(Box::new(filter));
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
        match self.filter {
            Some(_) => write!(
                f,
                "Argument {{ title: {:?}, description: {:?}, filter: Some(fn(&str) -> bool) }}",
                self.title, self.description,
            ),
            None => write!(
                f,
                "Argument {{ title: {:?}, description: {:?}, filter: None }}",
                self.title, self.description,
            ),
        }
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

/// Flag.
///
/// Flags are boolean switches. Flags are parsed by the Cherry instance by
/// using the full specifier `--title` or the short version `-t`. If using the
/// short version, multiple flags can be combined, `-a -b -c` is equivalent to
/// `-abc`. Flags can be mixed with Fields, however, must come at the
/// completion of the Arguemnt list. If a Flag is accidentally engaged multiple
/// times during parsing of a command, it remains active.
///
/// # Example
// Todo(Paul): Once Fields are fully implemented.
// /// ```rust
// /// use cherry::{Action, Field, Cherry};
// ///
// /// fn main() -> cherry::Result<()> {
// ///     let cherry = Cherry::new()
// ///         .insert(
// ///             Action::new("my_action")?
// ///                 .insert_field(
// ///                     Field::new("verbose")?
// ///                         .short('v')
// ///                         .description("If this action is to be run in verbose mode.")
// ///                 )?
// ///                 .then(|result| -> bool { result.get_field("verbose") })
// ///         )?;
// ///
// ///      // Will provide the status of the field
// ///      cherry.parse_str("my_action --verbose");
// ///      Ok(())
// /// }
// /// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Flag {
    /// The Flag title, the full specifier to utilise this Flag.
    title: String,

    /// The single characer short specified for this Flag.
    short: Option<char>,

    /// The Flag description for use in help text.
    description: Option<String>,
}

impl Flag {
    /// Create a new Flag.
    ///
    /// Create a new Flag instance.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Flag;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let flag = Flag::new("verbose")?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Will error when a blank (empty) title is provided. Flag must have a
    /// non-empty title assigned to them.
    pub fn new(title: &str) -> error::Result<Self> {
        if title.is_empty() {
            return Err(Error::new("Flag must have a non-empty title."));
        }

        Ok(Self {
            title: String::from(title),
            short: None,
            description: None,
        })
    }

    /// Update the description.
    ///
    /// The description of the Flag is used by the help text to assist users of
    /// the application to understand it. A good description text allows users to
    /// effectively use the application.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Flag;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let flag = Flag::new("verbose")?
    ///        .description("To run this action in verbose mode.");
    ///     Ok(())
    /// }
    /// ```
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(String::from(description));
        self
    }

    /// Update the short tag.
    ///
    /// The short tag of the Flag is used to activate the Flag without requiring
    /// the full title to be used. Flag short tags can also be combined under the
    /// same prefix, e.g. `-a -b -c` is equivalent to `-abc`.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Flag;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let flag = Flag::new("verbose")?
    ///        .short('v');
    ///     Ok(())
    /// }
    /// ```
    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
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

    /// The Argument values loaded into this Request.
    arguments: Vec<String>,
}

impl<'a, T> Request<'a, T> {
    /// Create a new Request.
    ///
    /// Create a new Request instance.
    pub(crate) fn new(action: &'a Action<T>) -> Self {
        Self {
            action,
            arguments: Vec::new(),
        }
    }

    /// Get an Argument.
    ///
    /// Retrieve an Argument value at the specified index.
    ///
    /// # Example
    pub fn get_argument(&self, index: usize) -> Option<&String> {
        self.arguments.get(index)
    }

    /// Insert an Argument.
    ///
    /// Insert an Argument into this Request. Arguments are defined on the Action
    /// and the actual Argument values loaded into the Request.
    ///
    /// # Error
    /// Will return an Error if attempting to add too many arguments to the Request
    /// for the Action, or if an Argument filter method fails.
    pub(crate) fn insert_argument(mut self, argument: &str) -> error::Result<Self> {
        let filter = self
            .action
            .arguments
            .get(self.arguments.len())
            .ok_or_else(|| Error::new("Todo: Help."))?
            .filter
            .as_ref();

        match filter {
            Some(callback) if !callback(argument) => Err(Error::new("Todo: Help.")),
            _ => {
                self.arguments.push(String::from(argument));
                Ok(self)
            }
        }
    }

    /// Run the Request.
    ///
    /// Invoke the Action's run method, consuming this Request.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Cherry};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let cherry = Cherry::new()
    ///         .insert(
    ///             Action::new("my_action")?
    ///                 .then(|_request| println!("Hello World"))
    ///         )?;
    ///     let request = cherry.parse_str("my_action")?;
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

    /// Validate the Request.
    ///
    /// Validate the Request by ensuring that enough Arguments, Fields and Flags
    /// have been supplied.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Argument, Cherry};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let cherry = Cherry::new()
    ///         .insert(
    ///             Action::new("my_action")?
    ///                 .insert_argument(Argument::new("my_argument")?)?
    ///                 .then(|_request| println!("Hello World"))
    ///         )?;
    ///     let request = cherry.parse_str("my_action value")?;
    ///     match request.validate() {
    ///        true => Ok(()),
    ///        false => Err(cherry::Error::new("Invalid!")),
    ///     }
    /// }
    /// ```
    pub fn validate(&self) -> bool {
        self.arguments.len() == self.action.arguments.len()
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
            arguments: Vec::new(),
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

    /// Action::insert_argument must insert an Argument.
    ///
    /// The insert argument method must correctly insert an Argument into the
    /// internal Vec.
    #[test]
    fn action_insert_argument() {
        let mut vec = Vec::new();
        vec.push(Argument::new("my_argument").unwrap());

        let mut expected = Action::<()>::new("my_action").unwrap();
        expected.arguments = vec;

        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap();

        assert_eq!(expected, actual);
    }

    /// Action::insert_argument must error with empty Argument title.
    ///
    /// The insert argument method must error when attempting to insert an Argument
    /// with an empty string title.
    #[test]
    fn action_insert_argument_empty() {
        let expected = Error::new("Argument must have a non-empty title.");

        let action = Action::<()>::new("my_action").unwrap();
        let mut argument = Argument::new("my_argument").unwrap();
        argument.title = String::from("");
        let actual = action.insert_argument(argument);

        assert_eq!(expected, actual.unwrap_err());
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
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap()
            .then(|_| {});
        let expected = "Action { \
                keyword: \"action\", \
                description: Some(\"Action description.\"), \
                arguments: [\
                    Argument { \
                        title: \"my_argument\", \
                        description: None, \
                        filter: None \
                    }\
                ], \
                then: Some(fn(Request<T>) -> T) \
            }";
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
        let expected = "Action { \
                keyword: \"action\", \
                description: None, \
                arguments: [], \
                then: None \
            }";
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
            title: String::from("Title"),
            description: None,
            filter: None,
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

    /// Argument::filter must correctly set the filter callback with a closure.
    ///
    /// The filter method must correctly set the internal Argument filter callback when
    /// passed a closure.
    #[test]
    fn action_filter_closure() {
        let argument = Argument::new("my_argument")
            .unwrap()
            .filter(|value: &str| -> bool { value == "Hello" });

        assert!(argument.filter.is_some());
    }

    /// Argument::filter must correctly set the filter callback with a method.
    ///
    /// The filter method must correctly set the internal Argument filter callback when
    /// passed a method.
    #[test]
    fn argument_filter_method() {
        fn callback(value: &str) -> bool {
            value == "Hello"
        }
        let argument = Argument::new("my_argument").unwrap().filter(callback);

        assert!(argument.filter.is_some());
    }

    /// Argument::fmt must debug the Argument.
    ///
    /// The custom implementation of the Debug::fmt method must correctly display
    /// the Argument.
    #[test]
    fn argument_fmt() {
        let argument = Argument::new("argument")
            .unwrap()
            .description("Argument description.")
            .filter(|_| -> bool { true });
        let expected = "Argument { \
                title: \"argument\", \
                description: Some(\"Argument description.\"), \
                filter: Some(fn(&str) -> bool) \
            }";
        let actual = format!("{:?}", argument);

        assert_eq!(expected, actual);
    }

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

    /// Flag::new must create as per struct initialisation.
    ///
    /// The new method on Flag must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn flag_new() {
        let expected = Flag {
            title: String::from("verbose"),
            short: None,
            description: None,
        };
        let actual = Flag::new("verbose").unwrap();

        assert_eq!(expected, actual);
    }

    /// Flag::new must error on empty title.
    ///
    /// The new method must correctly error when provided with an empty title
    /// during initialisation.
    #[test]
    fn flag_new_empty() {
        let expected = Error::new("Flag must have a non-empty title.");
        let actual = Flag::new("");

        assert_eq!(expected, actual.unwrap_err());
    }

    /// Flag::description must correctly set the description.
    ///
    /// The description method must correctly set the internal Flag description to
    /// the provided text.
    #[test]
    fn flag_description() {
        let flag = Flag::new("verbose")
            .unwrap()
            .description("My description.");

        assert_eq!(Some(String::from("My description.")), flag.description);
    }

    /// Flag::short must correctly set the short tag.
    ///
    /// The short method must correctly set the internal Flag short tag to the
    /// provided character.
    #[test]
    fn flag_short() {
        let flag = Flag::new("verbose")
            .unwrap()
            .short('v');

        assert_eq!(Some('v'), flag.short);
    }

    /// Request::new must create as per struct initialisation.
    ///
    /// The new method on Request must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn request_new() {
        let action = Action::<()>::new("my_action").unwrap();
        let expected = Request {
            action: &action,
            arguments: Vec::new(),
        };
        let actual = Request::new(&action);

        assert_eq!(expected, actual);
    }

    /// Request::insert_argument must insert an Argument.
    ///
    /// The insert argument method must insert an Argument into the Request.
    #[test]
    fn request_insert_argument() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap();

        let mut expected = Request::new(&action);
        expected.arguments.push(String::from("value"));

        let actual = Request::new(&action).insert_argument("value").unwrap();

        assert_eq!(expected, actual);
    }

    /// Request::insert_argument must insert if the filter is passed.
    ///
    /// The insert argument method must insert an Argument into the Request if the
    /// filter callback passess successfully.
    #[test]
    fn request_insert_argument_filter_pass() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(
                Argument::new("my_argument")
                    .unwrap()
                    .filter(|value| -> bool { value == "value" }),
            )
            .unwrap();

        let mut expected = Request::new(&action);
        expected.arguments.push(String::from("value"));

        let actual = Request::new(&action).insert_argument("value").unwrap();

        assert_eq!(expected, actual);
    }

    /// Request::insert_argument must error if the filter fails.
    ///
    /// The insert argument method must return an Error if the filter callback
    /// fails.
    #[test]
    fn request_insert_argument_filter_fail() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(
                Argument::new("my_argument")
                    .unwrap()
                    .filter(|value| -> bool { value != "value" }),
            )
            .unwrap();

        let expected = Error::new("Todo: Help.");
        let actual = Request::new(&action).insert_argument("value").unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Request::insert_argument must error if trying to insert too many Arguments.
    ///
    /// The insert argument method must return an Error if attempting to insert too
    /// many Arguments for the Action.
    #[test]
    fn request_insert_argument_overflow() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap();

        let expected = Error::new("Todo: Help.");
        let actual = Request::new(&action)
            .insert_argument("value")
            .unwrap()
            .insert_argument("value")
            .unwrap_err();

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

    /// Request::validate must successfully validate the Action.
    ///
    /// The validate method on Request must return true if the correct number of
    /// Arguments, Fields and Flags were supplied.
    #[test]
    fn request_validate() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap();
        let request = Request::new(&action).insert_argument("first").unwrap();

        assert!(request.validate());
    }

    /// Request::validate must successfully validate the Action.
    ///
    /// The validate method on Request must return false if too many Arguments were
    /// supplied.
    #[test]
    fn request_validate_argument_overflow() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap();
        let mut request = Request::new(&action);
        request.arguments.push(String::from("first"));
        request.arguments.push(String::from("second"));

        assert!(!request.validate());
    }

    /// Request::validate must successfully validate the Action.
    ///
    /// The validate method on Request must return false if too few Arguments were
    /// supplied.
    #[test]
    fn request_validate_argument_underflow() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap();
        let request = Request::new(&action);

        assert!(!request.validate());
    }
}
