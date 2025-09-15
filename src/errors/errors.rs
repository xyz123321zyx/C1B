use std::fmt;

#[derive(Debug)]
pub enum C1BError{
    IconImageNotFound,
    EmptyState,
    StateNotFound,
}

impl fmt::Display for C1BError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            C1BError::IconImageNotFound => write!(f, "Icon image not found"),
            C1BError::EmptyState => write!(f, "Empty state encountered"),
            C1BError::StateNotFound => write!(f, "State not found"),
        }
    }
}


impl std::error::Error for C1BError {}