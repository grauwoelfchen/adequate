use crate::validation::{handle, ValidationResult};

/// Check if the given string contains another string.
pub fn contains(
    part: &'static str,
) -> Box<dyn Fn(&String) -> ValidationResult> {
    Box::new(move |s: &String| {
        handle(!s.contains(part), "contains", vec![part.to_string()])
    })
}

#[cfg(test)]
mod test {
    use super::*;

    // contains

    #[test]
    fn test_contains_ok() {
        let f = contains("lorem");
        let result = f(&"lorem ipsum".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_contains_err() {
        let f = contains("dolor sit amet");
        let result = f(&"lorem ipsum".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_contains_err_message() {
        let f = contains("dolor sit amet");
        let result = f(&"lorem ipsum".to_string());
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must contain dolor sit amet".to_string())
        );
    }
}
