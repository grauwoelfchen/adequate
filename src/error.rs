use crate::feedback::Feedback;

/// Error is an enum struct wraps multiple feedback.
#[derive(Clone, Debug)]
pub struct Error(pub Vec<Feedback>);

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        for (i, f) in self.0.iter().enumerate() {
            dbg!(f != &other.0[i]);
            if f != &other.0[i] {
                return false;
            }
        }
        true
    }
}
