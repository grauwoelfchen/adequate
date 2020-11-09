#[macro_use]
extern crate lazy_static;
extern crate strfmt;

use std::collections::HashMap;
use std::fmt;

use strfmt::strfmt;

/// Message struct holds validation error message and its arguments as
/// interpolation.
#[derive(Clone, Debug)]
pub struct Message {
    pub text: String,
    pub args: Vec<String>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut args = HashMap::new();
        for (i, a) in self.args.iter().enumerate() {
            args.insert(i.to_string(), a);
        }
        // panic if identifiers in template text won't match
        let out = strfmt(&self.text, &args).expect("message format is invalid");
        if !args.is_empty() && self.text == out {
            panic!("message does not have expected number of identifiers");
        }
        write!(f, "{}", out)
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        if self.text != other.text {
            return false;
        }
        for (i, a) in self.args.iter().enumerate() {
            if a != &other.args[i] {
                return false;
            }
        }
        true
    }
}

/// Feedback struct contains target field name and multiple Message objects if
/// the context is negative (otherwise it will be an empty vector).
#[derive(Clone, Debug)]
pub struct Feedback {
    pub field: String,
    pub messages: Vec<Message>,
}

impl Feedback {
    pub fn is_negative(&self) -> bool {
        !self.messages.is_empty()
    }
}

impl PartialEq for Feedback {
    fn eq(&self, other: &Self) -> bool {
        if self.field != other.field {
            return false;
        }
        for (i, m) in self.messages.iter().enumerate() {
            if m != &other.messages[i] {
                return false;
            }
        }
        true
    }
}

/// Error is an enum struct wraps multiple feedback.
#[derive(Clone, Debug)]
pub struct Error(pub Vec<Feedback>);

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        for (i, f) in self.0.iter().enumerate() {
            if f != &other.0[i] {
                return false;
            }
        }
        true
    }
}

pub mod validation;

/// validate! macro validates given fields and its inputs.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate adequate;
///
/// # use adequate::{Error, Feedback, Message};
/// # use adequate::validation::max;
///
/// # fn main() {
///     let text = "lorem ipsum dolor sit amet".to_string();
///
///     let result = validate! {
///         "name" => text => [max(9)]
///     };
///     assert!(result.is_err());
///
///     let Error(out) = result.unwrap_err();
///     assert_eq!(out, vec![
///         Feedback {
///             field: "name".to_string(),
///             messages: vec![
///                 Message {
///                   text: "Must not contain more characters than {0}"
///                     .to_string(),
///                   args: vec!["9".to_string()]
///                 }
///             ]
///         }
///     ]);
///
///     let result = validate! {
///         "name" => text => [max(64)],
///         "description" => text => [max(255)]
///     };
///     assert!(result.is_ok());
/// # }
/// ```
#[macro_export]
macro_rules! validate {
    ( $( $n:expr => $v:expr => [ $( $c:expr ),* ] ),* ) => {{
        let errors = vec![$(
            Feedback {
                field: $n.to_string(),
                messages: [ $( $c(&$v) ),* ]
                    .iter()
                    .cloned()
                    .filter_map(|c| c.err())
                    .collect::<Vec<_>>()
            }
        ),*]
            .iter()
            .cloned()
            .filter(|f| f.is_negative())
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            Err(Error(errors))
        } else {
            Ok(())
        }
    }};
}

#[cfg(test)]
mod test {
    use super::*;
    use super::validation::ValidationResult;

    #[test]
    fn test_message() {
        let m = Message {
            text: "lorem ipsum".to_string(),
            args: Vec::new(),
        };
        assert_eq!(m.to_string(), "lorem ipsum");

        let m = Message {
            text: "lorem {0}".to_string(),
            args: vec!["ipsum".to_string()],
        };
        assert_eq!(m.to_string(), "lorem ipsum");
    }

    #[test]
    #[should_panic]
    fn test_message_panic_with_non_numeric_tmpl_ident() {
        let m = Message {
            text: "lorem ipsum {}".to_string(),
            args: vec!["dolor".to_string()],
        };
        m.to_string();
    }

    #[test]
    #[should_panic]
    fn test_message_panic_with_missing_ident() {
        let m = Message {
            text: "lorem ipsum".to_string(),
            args: vec!["dolor".to_string()],
        };
        m.to_string();
    }

    #[test]
    #[should_panic]
    fn test_message_panic_with_missing_arg() {
        let m = Message {
            text: "lorem ipsum {0} {1}".to_string(),
            args: vec!["dolor".to_string()],
        };
        m.to_string();
    }

    #[test]
    fn test_feedback_with_positive_result() {
        let f = Feedback {
            field: "dummy".to_string(),
            messages: vec![],
        };
        assert!(!f.is_negative());
    }

    #[test]
    fn test_feedback_with_negative_result() {
        let m = Message {
            text: "lorem ipsum {0}".to_string(),
            args: vec!["dolor".to_string()],
        };
        let f = Feedback {
            field: "dummy".to_string(),
            messages: vec![m],
        };
        assert!(f.is_negative());
    }

    #[test]
    fn test_failure() {
        let dummy = "".to_string();
        let validation = || -> Box<dyn Fn(&String) -> ValidationResult> {
            Box::new(move |_: &String| {
                Err(Message {
                    text: "Error".to_string(),
                    args: vec![],
                })
            })
        };

        let result = validate! {
            "input" => dummy => [validation()]
        };
        assert!(result.is_err());
    }

    #[test]
    fn test_success() {
        let dummy = "".to_string();
        let validation = || -> Box<dyn Fn(&String) -> ValidationResult> {
            Box::new(move |_: &String| Ok(()))
        };

        let result = validate! {
            "input" => dummy => [validation()]
        };
        assert!(result.is_ok());
    }
}
