use std::collections::HashMap;
use std::ops::Range;

use crate::Message;
use crate::validation::ValidationResult;

// TODO: any idea for i18n?
lazy_static! {
    static ref MESSAGES: [(&'static str, &'static str); 3] = [
        ("max", "Must not contain more characters than {0}"),
        ("min", "Must not contain less characters than {0}"),
        ("within", "Must be chars length within a range of {0}-{1}"),
    ];
}

fn len(err: bool, key: &str, args: Vec<String>) -> ValidationResult {
    if err {
        let m: HashMap<&str, &str> = MESSAGES.iter().cloned().collect();
        Err(Message {
            text: m.get(key).unwrap_or(&"").to_string(),
            args,
        })
    } else {
        Ok(())
    }
}

/// Check if the given string has length more than max.
pub fn max(size: usize) -> Box<dyn Fn(&String) -> ValidationResult> {
    Box::new(move |s: &String| {
        len(s.len() > size, "max", vec![size.to_string()])
    })
}

/// Check if the given string has length less than min.
pub fn min(size: usize) -> Box<dyn Fn(&String) -> ValidationResult> {
    Box::new(move |s: &String| {
        len(s.len() < size, "min", vec![size.to_string()])
    })
}

/// Check if the given string has length in a range of start (inclusive) .. end
/// (exclusive).
pub fn within(r: Range<usize>) -> Box<dyn Fn(&String) -> ValidationResult> {
    Box::new(move |s: &String| {
        len(
            !r.contains(&s.len()),
            "within",
            vec![r.start.to_string(), (r.end - 1).to_string()],
        )
    })
}

#[cfg(test)]
mod test {
    use super::*;

    // max

    #[test]
    fn test_max_ok() {
        let f = max(9);
        let result = f(&"test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_err() {
        let f = max(3);
        let result = f(&"test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_max_err_message() {
        let f = max(3);
        let result = f(&"test".to_string());
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must not contain more characters than 3".to_string())
        );
    }

    // min

    #[test]
    fn test_min_ok() {
        let f = min(3);
        let result = f(&"test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_min_err() {
        let f = min(9);
        let result = f(&"test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_min_err_message() {
        let f = min(9);
        let result = f(&"test".to_string());
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must not contain less characters than 9".to_string())
        );
    }

    // within

    #[test]
    fn test_within_ok() {
        let f = within(1..5);
        let result = f(&"test".to_string());
        assert!(result.is_ok());

        let f = within(4..5);
        let result = f(&"test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_within_err() {
        let f = within(0..4);
        let result = f(&"test".to_string());
        assert!(result.is_err());

        let f = within(9..18);
        let result = f(&"test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_within_err_message() {
        let f = within(1..4);
        let result = f(&"test".to_string());
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must be chars length within a range of 1-3".to_string())
        );

        let f = within(5..10);
        let result = f(&"test".to_string());
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must be chars length within a range of 5-9".to_string())
        );
    }
}
