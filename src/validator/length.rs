pub use crate::Message;
pub use crate::validator::ValidationResult;

/// Check if the given string has length more than max.
pub fn max(size: usize) -> Box<dyn Fn(&String) -> ValidationResult> {
    Box::new(move |s: &String| {
        if s.len() > size {
            Err(Message {
                text: "Must not contain more characters than %1.".to_string(),
                args: vec![size.to_string()],
            })
        } else {
            Ok(())
        }
    })
}
