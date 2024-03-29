use std::collections::HashMap;

use crate::Message;
use crate::message::MESSAGES;

type ValidationResult = std::result::Result<(), Message>;

pub type Validator = dyn Fn(&str) -> ValidationResult;
pub type OptionalValidator = dyn Fn(Option<&str>) -> ValidationResult;

pub mod contain;
pub mod length;

fn handle(err: bool, key: &str, args: Vec<String>) -> ValidationResult {
    if err {
        let m: HashMap<&str, &str> = MESSAGES.iter().cloned().collect();
        Err(Message {
            text: m.get(key).unwrap_or(&""),
            args,
        })
    } else {
        Ok(())
    }
}
