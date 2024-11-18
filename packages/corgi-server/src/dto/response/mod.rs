mod error;

pub use error::*;

pub type ResponseResult<T = ()> = std::result::Result<T, ErrorResponse>;
