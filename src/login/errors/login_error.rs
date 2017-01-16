//! This is documentation for the `mylib::login::errors::login_error` module.
//!
//! # Examples

use std::error;
use std::fmt;
// use std::result;

// type Result<T> = result::Result<T, Box<error::Error>>;

#[derive(Debug, PartialEq)]
pub enum LoginError {
    EmptyUsername,
    InvalidUsername,
    EmptyPassword,
    InvalidPassword,
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoginError::EmptyUsername =>
                write!(f, "please provide a non-empty username"),
            LoginError::InvalidUsername =>
                write!(f, "please provide a valid username"),
            LoginError::EmptyPassword =>
                write!(f, "please provide a non-empty password"),
            LoginError::InvalidPassword =>
                write!(f, "please provide a valid password"),
        }
    }
}

impl error::Error for LoginError {
    fn description(&self) -> &str {
        match *self {
            LoginError::EmptyUsername =>
                "empty username not allowed",
            LoginError::InvalidUsername =>
                "invalid username not allowed",
            LoginError::EmptyPassword =>
                "empty password not allowed",
            LoginError::InvalidPassword =>
                "invalid password not allowed",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LoginError::EmptyUsername => None,
            LoginError::InvalidUsername => None,
            LoginError::EmptyPassword => None,
            LoginError::InvalidPassword => None,
        }
    }
}
