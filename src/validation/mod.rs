use std::collections::HashMap;

use crate::Message;
use crate::message::MESSAGES;

pub type ValidationResult = std::result::Result<(), Message>;

pub mod contain;
pub mod length;

fn handle(err: bool, key: &str, args: Vec<String>) -> ValidationResult {
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
