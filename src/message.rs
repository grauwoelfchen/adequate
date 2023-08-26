use std::collections::HashMap;
use std::fmt;

use strfmt::strfmt;

/// Message struct holds the validation error message and its arguments for
/// interpolation.
#[derive(Clone, Debug)]
pub struct Message {
    pub text: &'static str,
    pub args: Vec<String>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut args = HashMap::new();
        for (i, a) in self.args.iter().enumerate() {
            args.insert(i.to_string(), a);
        }
        // panic if identifiers in template text won't match
        let out = strfmt(self.text, &args).expect("message format is invalid");
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
        self.args == other.args
    }
}

// TODO: any idea for i18n?
lazy_static! {
    pub static ref MESSAGES: [(&'static str, &'static str); 5] = [
        ("max", "Must not have more characters than {0}"),
        ("min", "Must not have less characters than {0}"),
        ("within", "Must be chars length within a range of {0}-{1}"),
        ("contains", "Must contain {0}"),
        ("not_contain", "Must not contain {0}"),
    ];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fmt() {
        let m = Message {
            text: "lorem ipsum {0}",
            args: vec!["dolor sit amet".to_string()],
        };
        assert_eq!(format!("{}", m), "lorem ipsum dolor sit amet".to_string());
    }

    #[test]
    fn test_eq() {
        let a = Message {
            text: "lorem ipsum {0}",
            args: Vec::new(),
        };
        assert!(a.eq(&a));

        let b = Message {
            text: "lorem ipsum {0}",
            args: Vec::new(),
        };
        assert!(a.eq(&b));

        let c = Message {
            text: "",
            args: vec!["dolor sit amet".to_string()],
        };
        assert!(!a.eq(&c));

        let d = Message {
            text: "lorem ipsum {0}",
            args: vec!["dolor sit amet".to_string()],
        };
        assert!(!a.eq(&d));
    }
}
