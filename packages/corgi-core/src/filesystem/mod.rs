mod ls;
pub mod path;

use std::ffi::OsStr;

pub use ls::*;

pub fn is_hidden(file_name: &OsStr) -> bool {
    file_name
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
