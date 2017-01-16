//! This is documentation for the `mylib::sql::errors::sql_error` module.
//!
//! # Examples

use std::error;
use std::fmt;
// use std::result;

// type Result<T> = result::Result<T, Box<error::Error>>;

#[derive(Debug, PartialEq)]
pub enum SqlError {
    UnsafeCharacters(String),
    EmptyString,
}

impl fmt::Display for SqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SqlError::UnsafeCharacters(ref msg) =>
                write!(f, "found unsafe sql characters in sql string `{}\'", &msg),
            SqlError::EmptyString =>
                write!(f, "found sql string empty"),
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
