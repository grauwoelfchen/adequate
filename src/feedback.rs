use crate::message::Message;

/// Feedback struct contains target field name and multiple Message objects if
/// the context is negative (otherwise it will be an empty vector).
#[derive(Clone, Debug)]
pub struct Feedback {
    pub field: String,
    pub messages: Vec<Message>,
}

impl Feedback {
    pub fn is_negative(&self) -> bool {
        !self.messages.is_empty()
    }
}

impl PartialEq for Feedback {
    fn eq(&self, other: &Self) -> bool {
        if self.field != other.field {
            return false;
        }
        self.messages == other.messages
    }
}
