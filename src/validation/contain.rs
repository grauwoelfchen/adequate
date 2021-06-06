use crate::validation::{handle, ValidationResult};

/// Check if the string contains another string.
pub fn contains(
    part: &'static str,
) -> Box<dyn Fn(&String) -> ValidationResult> {
    Box::new(move |s: &String| {
        handle(!s.contains(part), "contains", vec![part.to_string()])
    })
}

/// Check if a string contains another string when given.
pub fn contains_if_given(
    part: Option<&'static str>,
) -> Box<dyn Fn(&str) -> ValidationResult> {
    let p = part.unwrap_or_default();
    Box::new(move |s: &str| {
        handle(
            !p.is_empty() && !s.contains(&p),
            "contains",
            vec![p.to_string()],
        )
    })
}

/// Check if a string does not contain another string when given.
pub fn not_contain_if_given(
    part: Option<&'static str>,
) -> Box<dyn Fn(&str) -> ValidationResult> {
    let p = part.unwrap_or_default();
    Box::new(move |s: &str| {
        handle(
            !p.is_empty() && s.contains(&p),
            "not_contain",
            vec![p.to_string()],
        )
    })
}

/// Check if the string contains another string when it exists.
pub fn contains_if_present(
    part: &'static str,
) -> Box<dyn Fn(&Option<String>) -> ValidationResult> {
    Box::new(move |s: &Option<String>| match &s {
        Some(v) => contains(part)(&v),
        None => Ok(()),
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

    // contains_if_given
    #[test]
    fn test_contains_if_given_ok() {
        let part = "lorem ipsum".to_string();

        let f = contains_if_given(None);
        let result = f(&part);
        assert!(result.is_ok());

        let f = contains_if_given(Some(""));
        let result = f(&part);
        assert!(result.is_ok());

        let f = contains_if_given(Some("lorem"));
        let result = f(&part);
        assert!(result.is_ok());
    }

    #[test]
    fn test_contains_if_given_err() {
        let f = contains_if_given(Some("dolor sit amet"));
        let result = f(&"lorem ipsum".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_contains_if_given_err_message() {
        let f = contains_if_given(Some("dolor sit amet"));
        let result = f(&"lorem ipsum".to_string());
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must contain dolor sit amet".to_string())
        );
    }

    // not_contain_if_given
    #[test]
    fn test_not_contain_if_given_ok() {
        let part = "lorem ipsum".to_string();

        let f = not_contain_if_given(None);
        let result = f(&part);
        assert!(result.is_ok());

        let f = not_contain_if_given(Some(""));
        let result = f(&part);
        assert!(result.is_ok());

        let f = not_contain_if_given(Some("dolor sit amet"));
        let result = f(&part);
        assert!(result.is_ok());
    }

    #[test]
    fn test_not_contain_if_given_err() {
        let part = "lorem ipsum".to_string();

        let f = not_contain_if_given(Some("ipsum"));
        let result = f(&part);
        assert!(result.is_err());
    }

    #[test]
    fn test_not_contain_if_given_message() {
        let f = not_contain_if_given(Some("dolor"));
        let result = f(&"dolor sit amet".to_string());
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must not contain dolor".to_string())
        );
    }

    // contains_if_present
    #[test]
    fn test_contains_if_present_ok() {
        let f = contains_if_present("dolor sit amet");

        let result = f(&Some("lorem ipsum dolor sit amet".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_contains_if_present_err() {
        let f = contains_if_present("dolor sit amet");

        let result = f(&Some("lorem ipsum".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_contains_if_present_err_message() {
        let f = contains_if_present("dolor sit amet");

        let result = f(&Some("lorem ipsum".to_string()));
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must contain dolor sit amet".to_string())
        );
    }
}
