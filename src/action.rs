//! Action module
//!
//! The Action module houses specifics surrounding the Action type. Actions are
//! the application specific building blocks that add functionality to your CLI
//! application.

use crate::error::{self, Error};
use core::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

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
/// use cherry::{Action, Argument, Field, Flag};
///
/// fn main() -> cherry::Result<()> {
///     let action = Action::new("my_action")?
///         .insert_argument(
///             Argument::new("my_argument")?
///                 .description("My argument, must be longer than 3 characters.")
///                 .filter(|value| -> bool { value.len() > 3 })
///          )?
///          .insert_field(
///             Field::new("my_field")?
///                 .description("My field.")
///                 .short('f')
///                 .default("value")
///                 .filter(|value| -> bool { value.len() < 3 })
///          )?
///         .insert_flag(
///             Flag::new("my_flag")?
///                 .description("My flag.")
///                 .short('m')
///          )?
///         .then(|_| println!("Hello World!"));
///     Ok(())
/// }
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

    /// The Fields this Action accepts.
    fields: HashMap<String, Field>,

    /// The Flags this Action accepts.
    flags: HashMap<String, Flag>,

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
            fields: HashMap::new(),
            flags: HashMap::new(),
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

    /// Insert a Field into the Action.
    ///
    /// Insert a Field onto the Action object.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Field};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::<()>::new("my_action")?
    ///         .insert_field(Field::new("my_field")?)?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Errors occur if attempting to insert a Field with a blank (empty)
    /// title. Will also error if sharing a title or short with a Flag or existing
    /// Field.
    pub fn insert_field(mut self, field: Field) -> error::Result<Self> {
        if field.title.is_empty() {
            return Err(Error::new("Field must have a non-empty title."));
        }

        if self.fields.contains_key(&field.title) {
            return Err(Error::new(&format!(
                "Action '{}' already contains a Field '{}'.",
                self.keyword, &field.title
            )));
        }

        if self.flags.contains_key(&field.title) {
            return Err(Error::new(&format!(
                "Action '{}' already contains a Flag '{}'.",
                self.keyword, &field.title
            )));
        }

        if let Some(short) = field.short {
            let value = String::from(short);
            if self.fields.contains_key(&value) {
                return Err(Error::new(&format!(
                    "Action '{}' already contains a Field with short tag '{}'.",
                    self.keyword, &value
                )));
            }

            if self.flags.contains_key(&value) {
                return Err(Error::new(&format!(
                    "Action '{}' already contains a Flag with short tag '{}'.",
                    self.keyword, &value
                )));
            }
        }

        if let Some(short) = field.short {
            self.fields.insert(String::from(short), field.clone());
        }
        self.fields.insert(field.title.clone(), field);
        Ok(self)
    }

    /// Insert a Flag into the Action.
    ///
    /// Insert a Flag onto the Action object.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Flag};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let action = Action::<()>::new("my_action")?
    ///         .insert_flag(Flag::new("my_flag")?)?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Errors occur if attempting to insert a Flag with a blank (empty)
    /// title. Will also error if sharing a title or short with a Field or existing
    /// Flag.
    pub fn insert_flag(mut self, flag: Flag) -> error::Result<Self> {
        if flag.title.is_empty() {
            return Err(Error::new("Flag must have a non-empty title."));
        }

        if self.flags.contains_key(&flag.title) {
            return Err(Error::new(&format!(
                "Action '{}' already contains a Flag '{}'.",
                self.keyword, &flag.title
            )));
        }

        if self.fields.contains_key(&flag.title) {
            return Err(Error::new(&format!(
                "Action '{}' already contains a Field '{}'.",
                self.keyword, &flag.title
            )));
        }

        if let Some(short) = flag.short {
            let value = String::from(short);
            if self.flags.contains_key(&value) {
                return Err(Error::new(&format!(
                    "Action '{}' already contains a Flag with short tag '{}'.",
                    self.keyword, &value
                )));
            }

            if self.fields.contains_key(&value) {
                return Err(Error::new(&format!(
                    "Action '{}' already contains a Field with short tag '{}'.",
                    self.keyword, &value
                )));
            }
        }

        if let Some(short) = flag.short {
            self.flags.insert(String::from(short), flag.clone());
        }
        self.flags.insert(flag.title.clone(), flag);
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
                "Action {{ \
                    keyword: {:?}, \
                    description: {:?}, \
                    arguments: {:?}, \
                    fields: {:?}, \
                    flags: {:?}, \
                    then: Some(fn(Request<T>) -> T) \
                }}",
                self.keyword, self.description, self.arguments, self.fields, self.flags
            ),
            None => write!(
                f,
                "Action {{ \
                    keyword: {:?}, \
                    description: {:?}, \
                    arguments: {:?}, \
                    fields: {:?}, \
                    flags: {:?}, \
                    then: None \
                }}",
                self.keyword, self.description, self.arguments, self.fields, self.flags
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
#[derive(Clone)]
pub struct Argument {
    /// The Argument title for use in help text.
    pub title: String,

    /// The Argument description for use in help text.
    description: Option<String>,

    /// The filter to determine if the provided value is valid.
    filter: Option<Rc<dyn Fn(&str) -> bool>>,
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
        self.filter = Some(Rc::new(filter));
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

/// Field.
///
/// Fields are optional Arguments. Flags are parsed by the Cherry instance by
/// using the full specifier `--title` or the short version `-t`, followed by
/// the Field value. If a Field is accidentally specified multiple times during
/// parsing of a command, the final value will remain..
///
/// # Example
/// ```rust
/// use cherry::{Action, Field, Cherry};
///
/// fn main() -> cherry::Result<()> {
///     let cherry = Cherry::new()
///         .insert(
///             Action::new("my_action")?
///                 .insert_field(
///                     Field::new("username")?
///                         .short('u')
///                         .description("The username, must be longer that 3 characters.")
///                         .default("Admin")
///                         .filter(|value| -> bool { value.len() > 3 })
///                 )?
///                 .then(|result| -> String {
///                     String::from(result.get_field("username").unwrap_or(&String::from("")))
///                 })
///         )?;
///
///      // Will provide the status of the field
///      cherry.parse_str("my_action --username Guest");
///      Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct Field {
    /// The Field title, the full specifier to utilise this Field.
    title: String,

    /// The Field description for use in help text.
    description: Option<String>,

    /// The single characer short specified for this Field.
    short: Option<char>,

    /// The Field default value.
    default: Option<String>,

    /// The filter to determine if the provided value is valid.
    filter: Option<Rc<dyn Fn(&str) -> bool>>,
}

impl Field {
    /// Create a new Field.
    ///
    /// Create a new Field instance.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Field;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let field = Field::new("username")?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Error
    /// Will error when a blank (empty) title is provided. Field must have a
    /// non-empty title assigned to them.
    pub fn new(title: &str) -> error::Result<Self> {
        if title.is_empty() {
            return Err(Error::new("Field must have a non-empty title."));
        }

        Ok(Self {
            title: String::from(title),
            description: None,
            short: None,
            default: None,
            filter: None,
        })
    }

    /// Set the default for thie Field.
    ///
    /// The default value of the Field can be set, to provide sane defaults to the
    /// application logic.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Field;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let field = Field::new("username")?
    ///        .default("Admin");
    ///     Ok(())
    /// }
    /// ```
    pub fn default(mut self, default: &str) -> Self {
        self.default = Some(String::from(default));
        self
    }

    /// Update the description.
    ///
    /// The description of the Field is used by the help text to assist users of
    /// the application to understand it. A good description text allows users to
    /// effectively use the application.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Field;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let field = Field::new("username")?
    ///        .description("The username to use.");
    ///     Ok(())
    /// }
    /// ```
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(String::from(description));
        self
    }

    /// Update the filter callback on the Field.
    ///
    /// The filter callback of the Field is the method or closure that is called
    /// when this Field is parsed from the input to determine if the input is
    /// valid.
    ///
    /// # Example
    /// ## Using a method
    /// ```rust
    /// use cherry::{Field};
    ///
    /// fn is_valid(value: &str) -> bool {
    ///     value == "Hello"
    /// }
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let field = Field::new("my_field")?
    ///         .filter(is_valid);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Using a closure
    /// ```rust
    /// use cherry::Field;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let field = Field::new("my_field")?
    ///         .filter(|val: &str| -> bool {
    ///             // Implement application logic.
    ///             true
    ///         });
    ///     Ok(())
    /// }
    /// ```
    pub fn filter(mut self, filter: impl Fn(&str) -> bool + 'static) -> Self {
        self.filter = Some(Rc::new(filter));
        self
    }

    /// Update the short tag.
    ///
    /// The short tag of the Field is used to activate the Field without requiring
    /// the full title to be used.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Field;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let field = Field::new("username")?
    ///        .short('u');
    ///     Ok(())
    /// }
    /// ```
    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
}

impl Debug for Field {
    /// Format a Field for debug.
    ///
    /// Formats the Field for debug printing.
    ///
    /// # Example
    /// ```
    /// use cherry::Field;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let argument = Field::new("argument")?;
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
                "Field {{ \
                    title: {:?}, \
                    description: {:?}, \
                    short: {:?}, \
                    default: {:?}, \
                    filter: Some(fn(&str) -> bool) \
                }}",
                self.title, self.description, self.short, self.default,
            ),
            None => write!(
                f,
                "Field {{ \
                    title: {:?}, \
                    description: {:?}, \
                    short: {:?}, \
                    default: {:?}, \
                    filter: None \
                }}",
                self.title, self.description, self.short, self.default,
            ),
        }
    }
}

impl Eq for Field {}

impl Ord for Field {
    /// Ordering implementation.
    ///
    /// Defines how Fields should be ordered using comparison operators.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Field;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let first = Field::new("a")?;
    ///     let last = Field::new("z")?;
    ///     assert!(first < last);
    ///     Ok(())
    /// }
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.cmp(&other.title)
    }
}

impl PartialEq for Field {
    /// Partial Equality implementation.
    ///
    /// Defines how Fields should be considered equal.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Field;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let first = Field::new("a")?;
    ///     let last = Field::new("a")?;
    ///     assert_eq!(first, last);
    ///     Ok(())
    /// }
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.description == other.description
            && self.short == other.short
            && self.default == other.default
    }
}

impl PartialOrd for Field {
    /// Partial Ordering implementation.
    ///
    /// Defines how Fields should be ordered using comparison operators.
    ///
    /// # Example
    /// ```rust
    /// use cherry::Field;
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let first = Field::new("a")?;
    ///     let last = Field::new("z")?;
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
/// `-abc`. If a Flag is accidentally engaged multiple times during parsing of
/// a command, it remains active.
///
/// # Example
/// ```rust
/// use cherry::{Action, Flag, Cherry};
///
/// fn main() -> cherry::Result<()> {
///     let cherry = Cherry::new()
///         .insert(
///             Action::new("my_action")?
///                 .insert_flag(
///                     Flag::new("verbose")?
///                         .short('v')
///                         .description("If this action is to be run in verbose mode.")
///                 )?
///                 .then(|result| -> bool { *result.get_flag("verbose").unwrap_or(&false) })
///         )?;
///
///      // Will provide the status of the field
///      cherry.parse_str("my_action --verbose");
///      Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Flag {
    /// The Flag title, the full specifier to utilise this Flag.
    title: String,

    /// The Flag description for use in help text.
    description: Option<String>,

    /// The single characer short specified for this Flag.
    short: Option<char>,
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
            description: None,
            short: None,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request<'a, T> {
    /// The Action this Request is bound to.
    action: &'a Action<T>,

    /// The Argument values loaded into this Request.
    arguments: Vec<String>,

    /// The Field values loaded into this Request.
    fields: HashMap<String, Option<String>>,

    /// The Flag values loaded into this Request.
    flags: HashMap<String, bool>,
}

impl<'a, T> Request<'a, T> {
    /// Create a new Request.
    ///
    /// Create a new Request instance.
    pub(crate) fn new(action: &'a Action<T>) -> Self {
        Self {
            action,
            arguments: Vec::new(),
            fields: action
                .fields
                .values()
                .map(|field| (field.title.to_owned(), field.default.clone()))
                .collect(),
            flags: action
                .flags
                .values()
                .map(|flag| (flag.title.to_owned(), false))
                .collect(),
        }
    }

    /// Get an Argument.
    ///
    /// Retrieve an Argument value at the specified index.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Argument, Cherry, Error};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let cherry = Cherry::new()
    ///         .insert(
    ///             Action::new("my_action")?
    ///                 .insert_argument(Argument::new("my_argument")?)?
    ///                 .then(|request| {
    ///                     // Do something...
    ///                 })
    ///         )?;
    ///     let request = cherry.parse_str("my_action value")?;
    ///     request.get_argument(0).ok_or_else(|| Error::new("Missing arugmnet 0."))?;
    ///     Ok(())
    /// }
    /// ```
    pub fn get_argument(&self, index: usize) -> Option<&String> {
        self.arguments.get(index)
    }

    /// Get a Field.
    ///
    /// Retrieve a Field value.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Cherry, Error, Field};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let cherry = Cherry::new()
    ///         .insert(
    ///             Action::new("my_action")?
    ///                 .insert_field(Field::new("my_field")?)?
    ///                 .then(|request| {
    ///                     // Do something...
    ///                 })
    ///         )?;
    ///     let request = cherry.parse_str("my_action --my_field value")?;
    ///     request.get_field("my_field").ok_or_else(|| Error::new("Missing field 'my_field'."))?;
    ///     Ok(())
    /// }
    /// ```
    pub fn get_field(&self, key: &str) -> Option<&String> {
        self.fields.get(key)?.as_ref()
    }

    /// Get a Flag.
    ///
    /// Retrieve a Flag value.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Cherry, Error, Flag};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let cherry = Cherry::new()
    ///         .insert(
    ///             Action::new("my_action")?
    ///                 .insert_flag(Flag::new("my_flag")?)?
    ///                 .then(|request| {
    ///                     // Do something...
    ///                 })
    ///         )?;
    ///     let request = cherry.parse_str("my_action --my_flag")?;
    ///     request.get_flag("my_flag").ok_or_else(|| Error::new("Missing flag 'my_flag'."))?;
    ///     Ok(())
    /// }
    /// ```
    pub fn get_flag(&self, key: &str) -> Option<&bool> {
        self.flags.get(key)
    }

    /// Query if an Argument exists.
    ///
    /// Query for an Argument at the specified index.
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
    ///                 .then(|request| {
    ///                     // Do something...
    ///                 })
    ///         )?;
    ///     let request = cherry.parse_str("my_action value")?;
    ///     if request.has_argument(0) {
    ///         // Do something...
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn has_argument(&self, index: usize) -> bool {
        index < self.arguments.len()
    }

    /// Query if a Field exists.
    ///
    /// Query for a Field with the specified key.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Cherry, Field};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let cherry = Cherry::new()
    ///         .insert(
    ///             Action::new("my_action")?
    ///                 .insert_field(Field::new("my_field")?)?
    ///                 .then(|request| {
    ///                     // Do something...
    ///                 })
    ///         )?;
    ///     let request = cherry.parse_str("my_action --my_field value")?;
    ///     if request.has_field("my_field") {
    ///         // Do something...
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn has_field(&self, key: &str) -> bool {
        self.action.fields.contains_key(key)
    }

    /// Query if a Flag exists.
    ///
    /// Query for a Flag with the specified key.
    ///
    /// # Example
    /// ```rust
    /// use cherry::{Action, Cherry, Flag};
    ///
    /// fn main() -> cherry::Result<()> {
    ///     let cherry = Cherry::new()
    ///         .insert(
    ///             Action::new("my_action")?
    ///                 .insert_flag(Flag::new("my_flag")?)?
    ///                 .then(|request| {
    ///                     // Do something...
    ///                 })
    ///         )?;
    ///     let request = cherry.parse_str("my_action --my_flag")?;
    ///     if request.has_flag("my_flag") {
    ///         // Do something...
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn has_flag(&self, key: &str) -> bool {
        self.action.flags.contains_key(key)
    }

    /// Insert an Argument.
    ///
    /// Insert an Argument into this Request. Arguments are defined on the Action
    /// and the actual Argument values loaded into the Request.
    ///
    /// # Error
    /// Will return an Error if attempting to add too many Arguments to the Request
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

    /// Insert a Field.
    ///
    /// Insert a Field into this Request. Fields are defined on the Action and the
    /// actual Field values loaded into the Request.
    ///
    /// # Error
    /// Will error if the Flag is not found in the Action, or if a Field filter
    /// method fails.
    pub(crate) fn insert_field(mut self, field: &str, value: &str) -> error::Result<Self> {
        let field = self
            .action
            .fields
            .get(field)
            .ok_or_else(|| Error::new("Todo: Help."))?;
        match &field.filter {
            Some(callback) if !callback(value) => Err(Error::new("Todo: Help.")),
            _ => {
                self.fields
                    .insert(field.title.to_owned(), Some(String::from(value)));
                Ok(self)
            }
        }
    }

    /// Insert a Flag.
    ///
    /// Insert a Flag into this Request. Flags are defined on the Action and the
    /// Flag values are loaded into the request.
    ///
    /// # Error
    /// Will error if the Flag is not found in the Action.
    pub(crate) fn insert_flag(mut self, flag: &str) -> error::Result<Self> {
        match self.action.flags.get(flag) {
            None => Err(Error::new("Todo: Help.")),
            Some(value) => {
                self.flags.insert(value.title.to_owned(), true);
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
            fields: HashMap::new(),
            flags: HashMap::new(),
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

    /// Action::insert_field must insert a Field.
    ///
    /// The insert field method must correctly insert a Field into the internal
    /// HashMap.
    #[test]
    fn action_insert_field() {
        let mut map = HashMap::new();
        map.insert(String::from("my_field"), Field::new("my_field").unwrap());

        let mut expected = Action::<()>::new("my_action").unwrap();
        expected.fields = map;

        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap())
            .unwrap();

        assert_eq!(expected, actual);
    }

    /// Action::insert_field must error with empty Field title.
    ///
    /// The insert field method must error when attempting to insert a Field with
    /// an empty string title.
    #[test]
    fn action_insert_field_empty() {
        let expected = Error::new("Field must have a non-empty title.");

        let action = Action::<()>::new("my_action").unwrap();
        let mut field = Field::new("my_field").unwrap();
        field.title = String::from("");
        let actual = action.insert_field(field);

        assert_eq!(expected, actual.unwrap_err());
    }

    /// Action::insert_field must error on a Field collision.
    ///
    /// The insert field method must error when attempting to insert a Field when a
    /// Field with that title already exists on the Action.
    #[test]
    fn action_insert_field_collide_field() {
        let expected = Error::new("Action 'my_action' already contains a Field 'my_field'.");
        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap())
            .unwrap()
            .insert_field(Field::new("my_field").unwrap())
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Action::insert_field must error on a Field short collision.
    ///
    /// The insert field method must error when attempting to insert a Field with a
    /// short when a Field with that short already exists on the Action.
    #[test]
    fn action_insert_field_collide_field_short() {
        let expected =
            Error::new("Action 'my_action' already contains a Field with short tag 'm'.");
        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap().short('m'))
            .unwrap()
            .insert_field(Field::new("another_field").unwrap().short('m'))
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Action::insert_field must error on a Flag collision.
    ///
    /// The insert field method must error when attempting to insert a Field when a
    /// Flag with that title already exists on the Action.
    #[test]
    fn action_insert_field_collide_flag() {
        let expected = Error::new("Action 'my_action' already contains a Flag 'collide'.");
        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("collide").unwrap())
            .unwrap()
            .insert_field(Field::new("collide").unwrap())
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Action::insert_field must error on a Flag short collision.
    ///
    /// The insert field method must error when attempting to insert a Field with a
    /// short when a Flag with that short already exists on the Action.
    #[test]
    fn action_insert_field_collide_flag_short() {
        let expected = Error::new("Action 'my_action' already contains a Flag with short tag 'm'.");
        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap().short('m'))
            .unwrap()
            .insert_field(Field::new("my_field").unwrap().short('m'))
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Action::insert_flag must insert a Flag.
    ///
    /// The insert flag method must correctly insert a Flag into the internal
    /// HashMap.
    #[test]
    fn action_insert_flag() {
        let mut map = HashMap::new();
        map.insert(String::from("my_flag"), Flag::new("my_flag").unwrap());

        let mut expected = Action::<()>::new("my_action").unwrap();
        expected.flags = map;

        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap())
            .unwrap();

        assert_eq!(expected, actual);
    }

    /// Action::insert_flag must error with empty Flag title.
    ///
    /// The insert flag method must error when attempting to insert a Flag with an
    /// empty string title.
    #[test]
    fn action_insert_flag_empty() {
        let expected = Error::new("Flag must have a non-empty title.");

        let action = Action::<()>::new("my_action").unwrap();
        let mut flag = Flag::new("my_flag").unwrap();
        flag.title = String::from("");
        let actual = action.insert_flag(flag);

        assert_eq!(expected, actual.unwrap_err());
    }

    /// Action::insert_flag must error on a Flag collision.
    ///
    /// The insert flag method must error when attempting to insert a Flag when a
    /// Flag with that title already exists on the Action.
    #[test]
    fn action_insert_flag_collide_flag() {
        let expected = Error::new("Action 'my_action' already contains a Flag 'my_flag'.");
        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap())
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap())
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Action::insert_flag must error on a Flag short collision.
    ///
    /// The insert flag method must error when attempting to insert a Flag with a
    /// short when a Flag with that short already exists on the Action.
    #[test]
    fn action_insert_flag_collide_flag_short() {
        let expected = Error::new("Action 'my_action' already contains a Flag with short tag 'm'.");
        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap().short('m'))
            .unwrap()
            .insert_flag(Flag::new("another_flag").unwrap().short('m'))
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Action::insert_flag must error on a Field collision.
    ///
    /// The insert flag method must error when attempting to insert a Flag when a
    /// Field with that title already exists on the Action.
    #[test]
    fn action_insert_flag_collide_field() {
        let expected = Error::new("Action 'my_action' already contains a Field 'collide'.");
        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("collide").unwrap())
            .unwrap()
            .insert_flag(Flag::new("collide").unwrap())
            .unwrap_err();

        assert_eq!(expected, actual);
    }

    /// Action::insert_flag must error on a Field short collision.
    ///
    /// The insert flag method must error when attempting to insert a Flag with a
    /// short when a Field with that short already exists on the Action.
    #[test]
    fn action_insert_flag_collide_field_short() {
        let expected =
            Error::new("Action 'my_action' already contains a Field with short tag 'm'.");
        let actual = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap().short('m'))
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap().short('m'))
            .unwrap_err();

        assert_eq!(expected, actual);
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
            .insert_field(Field::new("my_field").unwrap())
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap())
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
                fields: {\
                    \"my_field\": Field { \
                        title: \"my_field\", \
                        description: None, \
                        short: None, \
                        default: None, \
                        filter: None \
                    }\
                }, \
                flags: {\
                    \"my_flag\": Flag { \
                        title: \"my_flag\", \
                        description: None, \
                        short: None \
                    }\
                }, \
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
                fields: {}, \
                flags: {}, \
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
    fn argument_filter_closure() {
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

    /// Field::new must create as per struct initialisation.
    ///
    /// The new method on Field must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn field_new() {
        let expected = Field {
            title: String::from("Title"),
            description: None,
            short: None,
            default: None,
            filter: None,
        };
        let actual = Field::new("Title").unwrap();

        assert_eq!(expected, actual);
    }

    /// Field::new must error on empty title.
    ///
    /// The new method must correctly error when provided with an empty title
    /// during initialisation.
    #[test]
    fn field_new_empty() {
        let expected = Error::new("Field must have a non-empty title.");
        let actual = Field::new("");

        assert_eq!(expected, actual.unwrap_err());
    }

    /// Field::description must correctly set the description.
    ///
    /// The description method must correctly set the internal Field description
    /// to the provided text.
    #[test]
    fn field_description() {
        let field = Field::new("my_field")
            .unwrap()
            .description("My description.");

        assert_eq!(Some(String::from("My description.")), field.description);
    }

    /// Field::short must correctly set the short.
    ///
    /// The short method must correctly set the internal Field short to the
    /// provided character.
    #[test]
    fn field_short() {
        let field = Field::new("my_field").unwrap().short('m');

        assert_eq!(Some('m'), field.short);
    }

    /// Field::default must correctly set the default.
    ///
    /// The default method must correctly set the internal Field default to the
    /// provided text.
    #[test]
    fn field_default() {
        let field = Field::new("my_field").unwrap().default("My default.");

        assert_eq!(Some(String::from("My default.")), field.default);
    }

    /// Field::filter must correctly set the filter callback with a closure.
    ///
    /// The filter method must correctly set the internal Field filter callback when
    /// passed a closure.
    #[test]
    fn field_filter_closure() {
        let field = Field::new("my_field")
            .unwrap()
            .filter(|value: &str| -> bool { value == "Hello" });

        assert!(field.filter.is_some());
    }

    /// Field::filter must correctly set the filter callback with a method.
    ///
    /// The filter method must correctly set the internal Field filter callback when
    /// passed a method.
    #[test]
    fn field_filter_method() {
        fn callback(value: &str) -> bool {
            value == "Hello"
        }
        let field = Field::new("my_field").unwrap().filter(callback);

        assert!(field.filter.is_some());
    }

    /// Field::fmt must debug the Field.
    ///
    /// The custom implementation of the Debug::fmt method must correctly display
    /// the Field.
    #[test]
    fn field_fmt() {
        let field = Field::new("field")
            .unwrap()
            .description("Field description.")
            .short('f')
            .default("value")
            .filter(|_| -> bool { true });
        let expected = "Field { \
                title: \"field\", \
                description: Some(\"Field description.\"), \
                short: Some('f'), \
                default: Some(\"value\"), \
                filter: Some(fn(&str) -> bool) \
            }";
        let actual = format!("{:?}", field);

        assert_eq!(expected, actual);
    }

    /// Field::fmt must handle a missing Options.
    ///
    /// The custom implementation of the Debug::fmt method must correctly display
    /// the Field when all Options are None.
    #[test]
    fn field_fmt_missing_options() {
        let field = Field::new("field").unwrap();
        let expected = "Field { \
            title: \"field\", \
            description: None, \
            short: None, \
            default: None, \
            filter: None \
        }";
        let actual = format!("{:?}", field);

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
            description: None,
            short: None,
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
        let flag = Flag::new("verbose").unwrap().description("My description.");

        assert_eq!(Some(String::from("My description.")), flag.description);
    }

    /// Flag::short must correctly set the short tag.
    ///
    /// The short method must correctly set the internal Flag short tag to the
    /// provided character.
    #[test]
    fn flag_short() {
        let flag = Flag::new("verbose").unwrap().short('v');

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
            fields: HashMap::new(),
            flags: HashMap::new(),
        };
        let actual = Request::new(&action);

        assert_eq!(expected, actual);
    }

    /// Request::new must handle a full Action.
    ///
    /// The new method on Request must create an object, handling a fully
    /// initialised Action.
    #[test]
    fn request_new_full_action() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap()
            .insert_field(Field::new("my_field").unwrap().short('f'))
            .unwrap()
            .insert_field(Field::new("my_field_default").unwrap().default("default"))
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap().short('m'))
            .unwrap();

        let mut fields = HashMap::new();
        fields.insert(String::from("my_field"), None);
        fields.insert(
            String::from("my_field_default"),
            Some(String::from("default")),
        );

        let mut flags = HashMap::new();
        flags.insert(String::from("my_flag"), false);

        let expected = Request {
            action: &action,
            arguments: Vec::new(),
            fields,
            flags,
        };
        let actual = Request::new(&action);

        assert_eq!(expected, actual);
    }

    /// Request::get_argument must retrieve the Argument.
    ///
    /// The get argument method must retrieve the Argument at the index.
    #[test]
    fn request_get_argument() {
        let value = String::from("value");
        let expected = Some(&value);

        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap();
        let request = Request::new(&action).insert_argument("value").unwrap();
        let actual = request.get_argument(0);

        assert_eq!(expected, actual);
    }

    /// Request::get_argument must return None if the Field does not exist.
    ///
    /// The get argument method must retrieve None if the Argument does not exist.
    #[test]
    fn request_get_argument_not_exists() {
        let expected = None;
        let action = Action::<()>::new("my_action").unwrap();
        let request = Request::new(&action);
        let actual = request.get_argument(0);

        assert_eq!(expected, actual);
    }

    /// Request::get_field must retrieve the Field.
    ///
    /// The get field method must retrieve the Field at the index.
    #[test]
    fn request_get_field() {
        let value = String::from("value");
        let expected = Some(&value);

        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap())
            .unwrap();
        let request = Request::new(&action)
            .insert_field("my_field", "value")
            .unwrap();
        let actual = request.get_field("my_field");

        assert_eq!(expected, actual);
    }

    /// Request::get_field must return None if the Field does not exist.
    ///
    /// The get field method must retrieve None if the Field does not exist.
    #[test]
    fn request_get_field_not_exists() {
        let expected = None;
        let action = Action::<()>::new("my_action").unwrap();
        let request = Request::new(&action);
        let actual = request.get_field("my_field");

        assert_eq!(expected, actual);
    }

    /// Request::get_field must return None if the Field value is None.
    ///
    /// The get field method must retrieve None if the Field is None.
    #[test]
    fn request_get_field_no_value() {
        let expected = None;
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap())
            .unwrap();
        let request = Request::new(&action);
        let actual = request.get_field("my_field");

        assert_eq!(expected, actual);
    }

    /// Request::get_flag must retrieve the Flag.
    ///
    /// The get flag method must retrieve the Flag at the index.
    #[test]
    fn request_get_flag() {
        let expected = Some(&true);
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap())
            .unwrap();
        let request = Request::new(&action).insert_flag("my_flag").unwrap();
        let actual = request.get_flag("my_flag");

        assert_eq!(expected, actual);
    }

    /// Request::get_flag must return None if the Flag does not exist.
    ///
    /// The get flag method must retrieve None if the Flag does not exist.
    #[test]
    fn request_get_flag_not_exists() {
        let expected = None;
        let action = Action::<()>::new("my_action").unwrap();
        let request = Request::new(&action);
        let actual = request.get_flag("my_flag");

        assert_eq!(expected, actual);
    }

    /// Request::has_argument must return if the Argument exists.
    ///
    /// The has argument method must return true if the Argument exists.
    #[test]
    fn request_has_argument() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap();
        let request = Request::new(&action).insert_argument("value").unwrap();
        let actual = request.has_argument(0);
        assert!(actual);
    }

    /// Request::has_argument must return if the Argument does not exist.
    ///
    /// The has argument method must return false if the Argument does not exist.
    #[test]
    fn request_has_argument_false() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_argument(Argument::new("my_argument").unwrap())
            .unwrap();
        let request = Request::new(&action).insert_argument("value").unwrap();
        let actual = request.has_argument(1);
        assert!(!actual);
    }

    /// Request::has_field must return if the Field exists.
    ///
    /// The has field method must return true if the Field exists.
    #[test]
    fn request_has_field() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap())
            .unwrap();
        let request = Request::new(&action);
        let actual = request.has_field("my_field");
        assert!(actual);
    }

    /// Request::has_field must return if the Field exists using the short tag.
    ///
    /// The has field method must return true if the Field exists when provieded
    /// with the short tag.
    #[test]
    fn request_has_field_short() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap().short('m'))
            .unwrap();
        let request = Request::new(&action);
        let actual = request.has_field("m");
        assert!(actual);
    }

    /// Request::has_field must return if the Field does not exist.
    ///
    /// The has field method must return false if the Field does not exist.
    #[test]
    fn request_has_field_false() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap())
            .unwrap();
        let request = Request::new(&action);
        let actual = request.has_field("not_my_field");
        assert!(!actual);
    }

    /// Request::has_flag must return if the Flag exists.
    ///
    /// The has flag method must return true if the Flag exists.
    #[test]
    fn request_has_flag() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap())
            .unwrap();
        let request = Request::new(&action);
        let actual = request.has_flag("my_flag");
        assert!(actual);
    }

    /// Request::has_flag must return if the Flag exists using the short tag.
    ///
    /// The has flag method must return true if the Flag exists when provieded
    /// with the short tag.
    #[test]
    fn request_has_flag_short() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap().short('m'))
            .unwrap();
        let request = Request::new(&action);
        let actual = request.has_flag("m");
        assert!(actual);
    }

    /// Request::has_flag must return if the Flag does not exist.
    ///
    /// The has flag method must return false if the Flag does not exist.
    #[test]
    fn request_has_flag_false() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap())
            .unwrap();
        let request = Request::new(&action);
        let actual = request.has_flag("not_my_flag");
        assert!(!actual);
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

    /// Request::insert_field must insert a Field.
    ///
    /// The insert field method must insert a Field into the Request.
    #[test]
    fn request_insert_field() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap())
            .unwrap();

        let mut expected = Request::new(&action);
        expected
            .fields
            .insert(String::from("my_field"), Some(String::from("value")));

        let actual = Request::new(&action)
            .insert_field("my_field", "value")
            .unwrap();

        assert_eq!(expected, actual);
    }

    /// Request::insert_field must error if Field is not found.
    ///
    /// The insert field method must error if the Field does not exist on the Action.
    #[test]
    fn request_insert_field_not_found() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(Field::new("my_field").unwrap())
            .unwrap();

        let expected = Error::new("Todo: Help.");
        let actual = Request::new(&action)
            .insert_field("not_my_field", "value")
            .unwrap_err();
        assert_eq!(expected, actual);
    }

    /// Request::insert_field must error if Field filter fails.
    ///
    /// The insert field method must error if the Field's valdiation filter fails.
    #[test]
    fn request_insert_field_fail_filter() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_field(
                Field::new("my_field")
                    .unwrap()
                    .filter(|_| -> bool { false }),
            )
            .unwrap();

        let expected = Error::new("Todo: Help.");
        let actual = Request::new(&action)
            .insert_field("my_field", "value")
            .unwrap_err();
        assert_eq!(expected, actual);
    }

    /// Request::insert_flag must insert a Flag.
    ///
    /// The insert flag method must insert a Flag into the Request.
    #[test]
    fn request_insert_flag() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap())
            .unwrap();

        let mut expected = Request::new(&action);
        expected.flags.insert(String::from("my_flag"), true);

        let actual = Request::new(&action).insert_flag("my_flag").unwrap();

        assert_eq!(expected, actual);
    }

    /// Request::insert_flag must error if Flag is not found.
    ///
    /// The insert flag method must error if the Flag does not exist on the Action.
    #[test]
    fn request_insert_flag_not_found() {
        let action = Action::<()>::new("my_action")
            .unwrap()
            .insert_flag(Flag::new("my_flag").unwrap())
            .unwrap();

        let expected = Error::new("Todo: Help.");
        let actual = Request::new(&action)
            .insert_flag("not_my_flag")
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
