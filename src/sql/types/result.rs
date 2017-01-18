use std::result;

use ::errors::SqlError;

/// Type to define a SqlError Result
pub type SqlResult<T> = result::Result<T, SqlError>;
