//! This is documentation for the `mylib::login::types` module.
//!
//! # Examples

// mod sql_safe;
// pub use self::sql_safe::SqlSafe;

// use rustc_serialize::{Decodable, Decoder};
use std::borrow::Cow;

pub type Username<'a> = Cow<'a, str>;
pub type Password<'a> = Cow<'a, str>;

#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct LoginUser<'a> {
    pub username: Username<'a>,
    pub password: Password<'a>,
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
