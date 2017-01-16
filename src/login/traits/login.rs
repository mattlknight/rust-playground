//! This is documentation for the `mylib::login::traits::login` module.
//!
//! # Examples

// use regex::Regex;
// use std::error;
// use std::result;
//
// use ::sql::traits::SqlSafe;
// use ::sql::errors::SqlError;
// use ::login::errors::LoginError;
// use ::login::types::{LoginUser, Username, Password};

// type Result<T> = result::Result<T, Box<error::Error>>;
//
// pub trait Login {
//     fn is_valid(&self) -> Result<&Self>;
// }
//
// impl<'a> Login for Username<'a> {
//     fn is_valid(&self) -> Result<&Self> {
//         match self.is_sql_safe() {
//             Ok(_) => Ok(self),
//             SqlError::EmptyString => Err(Box::new(LoginError::EmptyUsername)),
//         }
//     }
// }

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
