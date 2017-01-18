use std::borrow::Cow;

/// Type to define a user's username, accepts either `&str` or `String`
pub type Username<'a> = Cow<'a, str>;
