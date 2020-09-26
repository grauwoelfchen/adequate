pub use crate::Message;

pub type ValidationResult = std::result::Result<(), Message>;

pub mod length;
