use std::error;
use std::fmt;
use std::result;

type Result<T> = result::Result<T, Box<error::Error>>;

#[derive(Debug)]
pub enum SqlError {
    UnsafeCharacters(String),
}

impl fmt::Display for SqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SqlError::UnsafeCharacters(ref msg) =>
                write!(f, "found unsafe sql characters in sql string `{}\'", &msg),
        }
    }
}

impl error::Error for SqlError {
    fn description<'a>(&'a self) -> &'a str {
        match *self {
            SqlError::UnsafeCharacters(_) => "sql syntax not allowed",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SqlError::UnsafeCharacters(_) => None,
        }
    }
}
