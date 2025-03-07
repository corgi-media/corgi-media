use std::path::Path;

pub fn check_dir(path: &str) -> Result<String, crate::error::Error> {
    let os_path = Path::new(&path);
    let path = os_path.to_string_lossy().into_owned();

    if !os_path.is_dir() {
        return Err(crate::error::Error::InvalidPath(path));
    }

    Ok(path)
}
