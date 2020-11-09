use std::collections::HashMap;

use crate::Message;
use crate::validation::ValidationResult;

// TODO: any idea for i18n?
lazy_static! {
    static ref MESSAGES: [(&'static str, &'static str); 2] = [
        ("max", "Must not contain more characters than {0}"),
        ("min", "Must not contain less characters than {0}"),
    ];
}

/// Check if the given string has length more than max.
pub fn max(size: usize) -> Box<dyn Fn(&String) -> ValidationResult> {
    Box::new(move |s: &String| {
        if s.len() > size {
            let m: HashMap<&str, &str> = MESSAGES.iter().cloned().collect();
            Err(Message {
                text: m.get("max").unwrap_or(&"").to_string(),
                args: vec![size.to_string()],
            })
        } else {
            Ok(())
        }
    })
}

/// Check if the given string has length less than min.
pub fn min(size: usize) -> Box<dyn Fn(&String) -> ValidationResult> {
    Box::new(move |s: &String| {
        if s.len() < size {
            let m: HashMap<&str, &str> = MESSAGES.iter().cloned().collect();
            Err(Message {
                text: m.get("min").unwrap_or(&"").to_string(),
                args: vec![size.to_string()],
            })
        } else {
            Ok(())
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

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
}
