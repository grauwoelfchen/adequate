use crate::feedback::Feedback;

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

#[cfg(test)]
mod test {
    use super::*;

    use crate::message::Message;

    #[test]
    fn test_eq() {
        let a = Error(vec![Feedback {
            field: "name".to_string(),
            messages: Vec::new(),
        }]);
        assert!(a.eq(&a));

        let b = Error(vec![Feedback {
            field: "name".to_string(),
            messages: Vec::new(),
        }]);
        assert!(a.eq(&b));

        let c = Error(vec![Feedback {
            field: "description".to_string(),
            messages: Vec::new(),
        }]);
        assert!(!a.eq(&c));

        let d = Error(vec![Feedback {
            field: "name".to_string(),
            messages: vec![Message {
                text: "lorem ipsum {0}".to_string(),
                args: vec!["dolor sit amet".to_string()],
            }],
        }]);
        assert!(!a.eq(&d));
    }
}
