use regex::Regex;
use std::error;
use std::result;

use ::sql::errors::SqlError;

type Result<T> = result::Result<T, Box<error::Error>>;

pub trait SqlSafe {
    fn is_sql_safe(&self) -> Result<&Self>;
}

lazy_static! {
    static ref SQL_CHECK: Regex =
        Regex::new(r"([^\d\w\s@\.-]+)").expect("FAULT  SQL_CHECK");
}

impl<'a> SqlSafe for &'a str {
    fn is_sql_safe<'b>(&self) -> Result<&Self> {
        if Regex::is_match(&SQL_CHECK, self) {
            let cap = SQL_CHECK.captures(self).expect("FAULT  SQL_CHECK");
            Err(Box::new(SqlError::UnsafeCharacters(cap[1].to_string())))
        } else {
            Ok(self)
        }
    }
}
