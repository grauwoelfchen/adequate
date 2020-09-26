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

pub mod validator;

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
                    .filter(|c| c.err())
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
