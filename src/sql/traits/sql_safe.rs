//! This is documentation for the `mylib::sql::traits::sql_safe` module.
//!
//! # Examples

use regex::Regex;

use ::errors::SqlError;
use ::types::SqlResult;


/// Trait to determine if an item is safe to submit to a SQL server
pub trait SqlSafe {
    /// Method to check for SQL syntax within an item
    fn is_sql_safe(&self) -> SqlResult<&Self>;
}

/// lazy_static Macro used here to create a Regex matcher, one time that lasts the
/// life of the program. Generally a `'static` lifetime cannot call a function. `lazy_static!`
/// allows you to evaluate the expression and then store it in a static variable.
lazy_static! {
    /// Regex matcher for SQL syntax, matches non alphanumeric, but ignores email
    /// symbols[., -, _, @] and spaces
    pub static ref SQL_CHECK: Regex =
        Regex::new(r"([^\d\w\s@\.-\_]+)").expect("FAULT  SQL_CHECK Regex::new()");
}

// TODO:  Figure out why I can't comment this impl in rustdoc

impl SqlSafe for str {
    /// Base implementation for checking for sql syntax in an `&str` string slice reference
    fn is_sql_safe(&self) -> SqlResult<&Self> {
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
    fn sql_safe_err2() {
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
    fn sql_safe_ok_panic2() {
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
    fn bench_sql_safe_safe2(b: &mut Bencher) {
        b.iter(|| "".is_sql_safe());
    }
}
