mod account;
mod error;
mod users;

pub use account::*;
pub use error::*;
pub use users::*;

pub type ResponseResult<T = ()> = std::result::Result<T, ErrorResponse>;
