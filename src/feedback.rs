use crate::message::Message;

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
        self.messages == other.messages
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_negative() {
        let f = Feedback {
            field: "name".to_string(),
            messages: Vec::new(),
        };
        assert!(!f.is_negative());

        let m = Message {
            text: "lorem ipsum".to_string(),
            args: Vec::new(),
        };
        let f = Feedback {
            field: "name".to_string(),
            messages: vec![m],
        };
        assert!(f.is_negative());
    }

    #[test]
    fn test_eq() {
        let a = Feedback {
            field: "name".to_string(),
            messages: vec![Message {
                text: "lorem ipsum".to_string(),
                args: Vec::new(),
            }],
        };
        assert!(a.eq(&a));

        let b = Feedback {
            field: "name".to_string(),
            messages: vec![Message {
                text: "lorem ipsum".to_string(),
                args: Vec::new(),
            }],
        };
        assert!(a.eq(&b));

        let c = Feedback {
            field: "description".to_string(),
            messages: vec![Message {
                text: "lorem ipsum".to_string(),
                args: Vec::new(),
            }],
        };
        assert!(!a.eq(&c));

        let d = Feedback {
            field: "name".to_string(),
            messages: vec![Message {
                text: "lorem ipsum {0}".to_string(),
                args: vec!["dolor sit amet".to_string()],
            }],
        };
        assert!(!a.eq(&d));
    }
}
