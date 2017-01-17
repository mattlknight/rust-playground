//! This is documentation for the `mylib::sql::errors::sql_error` module.
//!
//! # Examples

use std::error;
use std::fmt;
use rustc_serialize::json::{DecoderError};
use ::errors::{LoginError, SqlError};
use std::io;

// use std::result;

// type Result<T> = result::Result<T, Box<error::Error>>;

#[derive(Debug)]
pub enum ServerError {
    LoginError(LoginError),
    DecoderError(DecoderError),
    SqlError(SqlError),
    Io(io::Error),
}

impl From<LoginError> for ServerError {
    fn from(err: LoginError) -> ServerError {
        ServerError::LoginError(err)
    }
}

impl From<DecoderError> for ServerError {
    fn from(err: DecoderError) -> ServerError {
        ServerError::DecoderError(err)
    }
}

impl From<SqlError> for ServerError {
    fn from(err: SqlError) -> ServerError {
        ServerError::SqlError(err)
    }
}

impl From<io::Error> for ServerError {
    fn from(err: io::Error) -> ServerError {
        ServerError::Io(err)
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::LoginError(ref err) => err.fmt(f),
            ServerError::DecoderError(ref err) => err.fmt(f),
            ServerError::SqlError(ref err) => err.fmt(f),
            ServerError::Io(ref err) => err.fmt(f),
            // _ => write!(f, "unimplemented code"),
        }
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::LoginError(ref err) => err.description(),
            ServerError::DecoderError(ref err) => err.description(),
            ServerError::SqlError(ref err) => err.description(),
            ServerError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ServerError::LoginError(ref err) => err.cause(),
            ServerError::DecoderError(ref err) => err.cause(),
            ServerError::SqlError(ref err) => err.cause(),
            ServerError::Io(ref err) => err.cause(),
        }
    }
}
