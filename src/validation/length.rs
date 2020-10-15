use crate::Message;
use crate::validation::ValidationResult;

/// Check if the given string has length more than max.
pub fn max(size: usize) -> Box<dyn Fn(&String) -> ValidationResult> {
    Box::new(move |s: &String| {
        if s.len() > size {
            Err(Message {
                text: "Must not contain more characters than {0}".to_string(),
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
    fn test_max_err() {
        let f = max(9);
        let result = f(&"test".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_ok() {
        let f = max(3);
        let result = f(&"test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_max_message() {
        let f = max(3);
        let result = f(&"test".to_string());
        assert_eq!(
            result.map_err(|e| e.to_string()),
            Err("Must not contain more characters than 3".to_string())
        );
    }
}
