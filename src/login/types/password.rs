use std::borrow::Cow;

/// Type to define a user's password, accepts either `&str` or `String`
pub type Password<'a> = Cow<'a, str>;
