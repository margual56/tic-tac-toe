use std::error::Error;
use std::fmt;

pub enum BoardError {
    FullColumn,
    Other,
}

// Allow the use of "{}" format specifier
impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BoardError::FullColumn => write!(f, "The column is full"),
            BoardError::Other => write!(f, "Unknown error!"),
        }
    }
}

impl fmt::Debug for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

// Allow this type to be treated like an error
impl Error for BoardError {
    fn description(&self) -> &str {
        match *self {
            BoardError::FullColumn => "The column is full",
            BoardError::Other => "Unknown error!",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            BoardError::FullColumn => None,
            BoardError::Other => None,
        }
    }
}
