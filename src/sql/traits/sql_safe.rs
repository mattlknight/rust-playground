//! This is documentation for the `mylib::sql::traits::sql_safe` module.
//!
//! # Examples

use regex::Regex;
// use std::error;
use std::result;

use ::errors::{SqlError};

type Result<T> = result::Result<T, SqlError>;

pub trait SqlSafe {
    fn is_sql_safe(&self) -> Result<&Self>;
}

lazy_static! {
    pub static ref SQL_CHECK: Regex =
        Regex::new(r"([^\d\w\s@\.-]+)").expect("FAULT  SQL_CHECK Regex::new()");
}

impl SqlSafe for str {
    fn is_sql_safe(&self) -> Result<&Self> {
        if self.is_empty() {
            Err(SqlError::EmptyString)
        } else if Regex::is_match(&SQL_CHECK, self) {
            let cap = SQL_CHECK.captures(self).expect("FAULT  SQL_CHECK.captures()");
            Err(SqlError::UnsafeCharacters(cap[1].to_string()))
        } else {
            Ok(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn sql_safe_err() {
        assert!("username;".is_sql_safe().is_err());
    }

    #[test]
    fn sql_safe_ok() {
        assert!("username".is_sql_safe().is_ok());
    }

    #[test]
    fn sql_safe_err() {
        assert!("".is_sql_safe().is_err());
    }

    #[test]
    #[should_panic]
    fn sql_safe_err_panic() {
        assert!("username".is_sql_safe().is_err());
    }

    #[test]
    #[should_panic]
    fn sql_safe_ok_panic() {
        assert!("username;".is_sql_safe().is_ok());
    }

    #[test]
    #[should_panic]
    fn sql_safe_ok_panic() {
        assert!("".is_sql_safe().is_ok());
    }

    #[bench]
    fn bench_sql_safe_unsafe(b: &mut Bencher) {
        b.iter(|| "username;".is_sql_safe());
    }

    #[bench]
    fn bench_sql_safe_safe(b: &mut Bencher) {
        b.iter(|| "username".is_sql_safe());
    }

    #[bench]
    fn bench_sql_safe_safe(b: &mut Bencher) {
        b.iter(|| "".is_sql_safe());
    }
}
