use std::error;
use std::fmt;

#[derive(Debug)]
pub enum FlarieError {
    PathsDontMatchError,
}

impl fmt::Display for FlarieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlarieError::PathsDontMatchError => write!(f, "Paths do not match"),
        }
    }
}

impl error::Error for FlarieError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            FlarieError::PathsDontMatchError => None,
        }
    }
}

