use crate::validation::{handle, OptionalValidator, Validator};

/// Returns a function that checks if the given &str contains another &str.
pub fn contains(part: &'static str) -> Box<Validator> {
    Box::new(move |s: &str| {
        handle(!s.contains(part), "contains", vec![part.to_string()])
    })
}

/// Returns a function that checks if the given &str contains optional &str.
pub fn contains_if_given(part: Option<&'static str>) -> Box<Validator> {
    let p = part.unwrap_or_default();
    Box::new(move |s: &str| {
        handle(
            !p.is_empty() && !s.contains(p),
            "contains",
            vec![p.to_string()],
        )
    })
}

/// Returns a function that checks if the given string does not contain
/// optional &str.
pub fn not_contain_if_given(part: Option<&'static str>) -> Box<Validator> {
    let p = part.unwrap_or_default();
    Box::new(move |s: &str| {
        handle(
            !p.is_empty() && s.contains(p),
            "not_contain",
            vec![p.to_string()],
        )
    })
}

/// Returns a function that checks if the given string contains another string
/// when the one exists.
pub fn contains_if_present(part: &'static str) -> Box<OptionalValidator> {
    Box::new(move |s: Option<&str>| match s {
        Some(v) => contains(part)(v),
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
        let result = f("lorem ipsum");
        assert!(result.is_ok());
    }

    #[test]
    fn test_contains_err() {
        let f = contains("dolor sit amet");
        let result = f("lorem ipsum");
        assert!(result.is_err());
    }

    #[test]
    fn test_contains_err_message() {
        let f = contains("dolor sit amet");
        let result = f("lorem ipsum");
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must contain dolor sit amet".to_string())
        );
    }

    // contains_if_given
    #[test]
    fn test_contains_if_given_ok() {
        let part = "lorem ipsum";

        let f = contains_if_given(None);
        let result = f(part);
        assert!(result.is_ok());

        let f = contains_if_given(Some(""));
        let result = f(part);
        assert!(result.is_ok());

        let f = contains_if_given(Some("lorem"));
        let result = f(part);
        assert!(result.is_ok());
    }

    #[test]
    fn test_contains_if_given_err() {
        let f = contains_if_given(Some("dolor sit amet"));
        let result = f("lorem ipsum");
        assert!(result.is_err());
    }

    #[test]
    fn test_contains_if_given_err_message() {
        let f = contains_if_given(Some("dolor sit amet"));
        let result = f("lorem ipsum");
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must contain dolor sit amet".to_string())
        );
    }

    // not_contain_if_given
    #[test]
    fn test_not_contain_if_given_ok() {
        let part = "lorem ipsum";

        let f = not_contain_if_given(None);
        let result = f(part);
        assert!(result.is_ok());

        let f = not_contain_if_given(Some(""));
        let result = f(part);
        assert!(result.is_ok());

        let f = not_contain_if_given(Some("dolor sit amet"));
        let result = f(part);
        assert!(result.is_ok());
    }

    #[test]
    fn test_not_contain_if_given_err() {
        let part = "lorem ipsum";

        let f = not_contain_if_given(Some("ipsum"));
        let result = f(part);
        assert!(result.is_err());
    }

    #[test]
    fn test_not_contain_if_given_message() {
        let f = not_contain_if_given(Some("dolor"));
        let result = f("dolor sit amet");
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must not contain dolor".to_string())
        );
    }

    // contains_if_present
    #[test]
    fn test_contains_if_present_ok() {
        let f = contains_if_present("dolor sit amet");

        let result = f(Some("lorem ipsum dolor sit amet"));
        assert!(result.is_ok());

        let result = f(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_contains_if_present_err() {
        let f = contains_if_present("dolor sit amet");

        let result = f(Some("lorem ipsum"));
        assert!(result.is_err());
    }

    #[test]
    fn test_contains_if_present_err_message() {
        let f = contains_if_present("dolor sit amet");

        let result = f(Some("lorem ipsum"));
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must contain dolor sit amet".to_string())
        );
    }
}
