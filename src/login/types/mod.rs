//! This is documentation for the `mylib::login::types` module.
//!
//! # Examples

// mod sql_safe;
// pub use self::sql_safe::SqlSafe;

pub type Username<'a> = &'a str;
pub type Password<'a> = &'a str;

pub struct LoginUser<'a> {
    pub username: Username<'a>,
    pub password: Password<'a>,
}
