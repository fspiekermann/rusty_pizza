use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveError {
    NotFound,
}

impl fmt::Display for RemoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RemoveError::*;
        match self {
            NotFound => write!(f, "Not found"),
        }
    }
}

impl Error for RemoveError {}

