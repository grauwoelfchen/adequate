#[derive(Debug, Clone)]
pub struct Message {
    text: String,
    args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Feedback {
    pub field: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone)]
pub struct Error(pub Vec<Feedback>);

pub mod validation;

/// validate! macro validates given fields and its inputs.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate adequate;
///
/// # use adequate::{Error, Feedback};
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
        use adequate::Feedback;

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
            .filter(|f| f.messages.len() > 0 )
            .collect::<Vec<_>>();
        if errors.len() > 0 {
            Err(Error(errors))
        } else {
            Ok(())
        }
    }};
}
