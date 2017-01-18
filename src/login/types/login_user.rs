use regex::Regex;
use rustc_serialize::{Decoder, Decodable};
use std::fmt;

use ::types::{Username, Password, SqlResult};
use ::traits::{SqlSafe, SQL_CHECK};
use ::errors::{SqlError};


#[derive(Debug, PartialEq, RustcEncodable)]
/// A struct to hold user login credentials
pub struct LoginUser<'a> {
    /// username field, Username type
    pub username: Username<'a>,
    /// username field, Password type
    pub password: Password<'a>,
}

impl<'a> fmt::Display for LoginUser<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LoginUser username:{}  password:{}", self.username, self.password)
    }
}

impl<'a> SqlSafe for LoginUser<'a> {
    fn is_sql_safe(&self) -> SqlResult<&Self> {
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

impl<'a> Decodable for LoginUser<'a> {
    fn decode<D: Decoder>(d: &mut D) -> Result<LoginUser<'a>, D::Error> {
        d.read_struct("LoginUser", 2, |d| {
            let username = try!(d.read_struct_field("username", 0, |d| { d.read_str() }));
            let password = try!(d.read_struct_field("password", 1, |d| { d.read_str() }));
            Ok(LoginUser{ username: From::from(username), password: From::from(password) })
        })
    }
}
