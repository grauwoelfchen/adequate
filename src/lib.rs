#[macro_use]
extern crate lazy_static;
extern crate strfmt;

mod error;
pub use error::Error;

mod feedback;
pub use feedback::Feedback;

mod message;
pub use message::Message;

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
/// # use adequate::validation::length;
///
/// # fn main() {
///     let text = "lorem ipsum dolor sit amet";
///
///     let result = validate! {
///         "name" => text => [length::max(9)]
///     };
///     assert!(result.is_err());
///
///     let Error(out) = result.unwrap_err();
///     assert_eq!(out, vec![
///         Feedback {
///             field: "name",
///             messages: vec![
///                 Message {
///                   text: "Must not have more characters than {0}",
///                   args: vec!["9".to_string()]
///                 }
///             ]
///         }
///     ]);
///
///     let result = validate! {
///         "name" => text => [length::max(64)],
///         "description" => text => [length::max(255)]
///     };
///     assert!(result.is_ok());
/// # }
/// ```
#[macro_export]
macro_rules! validate {
    ( $( $n:expr => $v:expr => [ $( $c:expr ),* ] ),* ) => {{
        let errors = [$(
            Feedback {
                field: $n,
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
    use super::validation::Validator;

    #[test]
    fn test_message() {
        let m = Message {
            text: "lorem ipsum",
            args: Vec::new(),
        };
        assert_eq!(m.to_string(), "lorem ipsum");

        let m = Message {
            text: "lorem {0}",
            args: vec!["ipsum".to_string()],
        };
        assert_eq!(m.to_string(), "lorem ipsum");
    }

    #[test]
    #[should_panic]
    fn test_message_panic_with_non_numeric_tmpl_ident() {
        let m = Message {
            text: "lorem ipsum {}",
            args: vec!["dolor".to_string()],
        };
        m.to_string();
    }

    #[test]
    #[should_panic]
    fn test_message_panic_with_missing_ident() {
        let m = Message {
            text: "lorem ipsum",
            args: vec!["dolor".to_string()],
        };
        m.to_string();
    }

    #[test]
    #[should_panic]
    fn test_message_panic_with_missing_arg() {
        let m = Message {
            text: "lorem ipsum {0} {1}",
            args: vec!["dolor".to_string()],
        };
        m.to_string();
    }

    #[test]
    fn test_feedback_with_positive_result() {
        let f = Feedback {
            field: "dummy",
            messages: vec![],
        };
        assert!(!f.is_negative());
    }

    #[test]
    fn test_feedback_with_negative_result() {
        let m = Message {
            text: "lorem ipsum {0}",
            args: vec!["dolor".to_string()],
        };
        let f = Feedback {
            field: "dummy",
            messages: vec![m],
        };
        assert!(f.is_negative());
    }

    #[test]
    fn test_failure() {
        let dummy = "".to_string();
        let validation = || -> Box<Validator> {
            Box::new(move |_: &str| {
                Err(Message {
                    text: "Error",
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
        let validation =
            || -> Box<Validator> { Box::new(move |_: &str| Ok(())) };

        let result = validate! {
            "input" => dummy => [validation()]
        };
        assert!(result.is_ok());
    }
}
