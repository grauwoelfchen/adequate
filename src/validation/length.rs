use std::ops::Range;

use crate::validation::{handle, OptionalValidator, Validator};

/// Check if the given string has length more than max.
pub fn max(size: usize) -> Box<Validator> {
    Box::new(move |s: &str| {
        handle(s.len() > size, "max", vec![size.to_string()])
    })
}

/// Check if the given string has length more than max only when it exists.
pub fn max_if_present(size: usize) -> Box<OptionalValidator> {
    Box::new(move |s: Option<&str>| match s {
        Some(v) => max(size)(v),
        None => Ok(()),
    })
}

/// Check if the given string has length less than min.
pub fn min(size: usize) -> Box<Validator> {
    Box::new(move |s: &str| {
        handle(s.len() < size, "min", vec![size.to_string()])
    })
}

/// Check if the given string has length less than min only when it exists.
pub fn min_if_present(size: usize) -> Box<OptionalValidator> {
    Box::new(move |s: Option<&str>| match s {
        Some(v) => min(size)(v),
        None => Ok(()),
    })
}

/// Check if the given string has length in a range of start (inclusive) .. end
/// (exclusive).
pub fn within(r: Range<usize>) -> Box<Validator> {
    Box::new(move |s: &str| {
        handle(
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
        let result = f("test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_err() {
        let f = max(3);
        let result = f("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_max_err_message() {
        let f = max(3);
        let result = f("test");
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must not have more characters than 3".to_string())
        );
    }

    // max_if_present

    #[test]
    fn test_max_if_present_ok() {
        let f = max_if_present(9);

        let result = f(Some("test"));
        assert!(result.is_ok());

        let result = f(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_if_present_err() {
        let f = max_if_present(3);

        let result = f(Some("test"));
        assert!(result.is_err());
    }

    #[test]
    fn test_max_if_present_err_message() {
        let f = max_if_present(3);
        let result = f(Some("test"));
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must not have more characters than 3".to_string())
        );
    }

    // min

    #[test]
    fn test_min_ok() {
        let f = min(3);
        let result = f("test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_min_err() {
        let f = min(9);
        let result = f("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_min_err_message() {
        let f = min(9);
        let result = f("test");
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must not have less characters than 9".to_string())
        );
    }

    // min_if_present

    #[test]
    fn test_min_if_present_ok() {
        let f = min_if_present(3);

        let result = f(Some("test"));
        assert!(result.is_ok());

        let result = f(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_min_if_present_err() {
        let f = min_if_present(9);

        let result = f(Some("test"));
        assert!(result.is_err());
    }

    #[test]
    fn test_min_if_present_err_message() {
        let f = min_if_present(9);
        let result = f(Some("test"));
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must not have less characters than 9".to_string())
        );
    }

    // within

    #[test]
    fn test_within_ok() {
        let f = within(1..5);
        let result = f("test");
        assert!(result.is_ok());

        let f = within(4..5);
        let result = f("test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_within_err() {
        let f = within(0..4);
        let result = f("test");
        assert!(result.is_err());

        let f = within(9..18);
        let result = f("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_within_err_message() {
        let f = within(1..4);
        let result = f("test");
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must be chars length within a range of 1-3".to_string())
        );

        let f = within(5..10);
        let result = f("test");
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must be chars length within a range of 5-9".to_string())
        );
    }
}
