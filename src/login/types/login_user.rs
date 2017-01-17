// use rustc_serialize::{Decodable, Decoder};
use std::borrow::Cow;
use std::fmt;
use std::result;
// use std::error;
use regex::Regex;

use ::traits::{SqlSafe, SQL_CHECK};
use ::errors::{SqlError};

pub type Username<'a> = Cow<'a, str>;
pub type Password<'a> = Cow<'a, str>;

#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct LoginUser<'a> {
    pub username: Username<'a>,
    pub password: Password<'a>,
}

impl<'a> fmt::Display for LoginUser<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LoginUser username:{}  password:{}", self.username, self.password)
    }
}

type Result<T> = result::Result<T, SqlError>;

impl<'a> SqlSafe for LoginUser<'a> {
    fn is_sql_safe(&self) -> Result<&Self> {
        if self.username.is_empty() || self.password.is_empty() {
            Err(SqlError::EmptyString)
        } else if Regex::is_match(&SQL_CHECK, &self.username) {
            let cap = SQL_CHECK.captures(&self.username).expect("FAULT  SQL_CHECK.captures()");
            Err(SqlError::UnsafeCharacters(cap[1].to_string()))
        } else {
            Ok(self)
        }
    }
}

// impl<'a> Decodable for LoginUser<'a> {
//     fn decode<D: Decoder>(d: &'a mut D) -> Result<LoginUser<'a>, D::Error> {
//         d.read_struct("LoginUser", 2, |d| {
//             let username = try!(d.read_struct_field("username", 0, |d| { d.read_str() }));
//             let password = try!(d.read_struct_field("password", 1, |d| { d.read_str() }));
//             Ok(LoginUser{ username: username, password: password })
//         })
//     }
// }
