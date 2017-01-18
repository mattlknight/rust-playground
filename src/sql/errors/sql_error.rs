use std::error;
use std::fmt;


#[derive(Debug, PartialEq)]
/// Error for handling SQL specific issues
pub enum SqlError {
    /// SQL Syntax Error for finding SQL syntax in a string
    UnsafeCharacters(String),
    /// SQL Error for an empty string
    EmptyString,
}

impl fmt::Display for SqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SqlError::UnsafeCharacters(ref msg) =>
                write!(f, "found unsafe sql characters in string `{}\'", &msg),
            SqlError::EmptyString =>
                write!(f, "empty sql string not allowed"),
        }
    }
}

impl error::Error for SqlError {
    fn description(&self) -> &str {
        match *self {
            SqlError::UnsafeCharacters(_) => "sql syntax not allowed",
            SqlError::EmptyString => "empty sql string not allowed",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SqlError::UnsafeCharacters(_) => None,
            SqlError::EmptyString => None,
        }
    }
}
