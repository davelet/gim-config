use std::{
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

/// Returns the application's config directory path (~/.config/gim/)
///
/// # Returns
/// `std::io::Result<PathBuf>` - On success, returns the path to the config directory
///
/// # Errors
/// Returns `std::io::Error` with `ErrorKind::NotFound` if the home directory cannot be determined
pub fn config_dir() -> Result<PathBuf> {
    let config_dir = dirs::home_dir();
    if config_dir.is_none() {
        return Err(Error::new(ErrorKind::NotFound, "Home directory not found"));
    }

    let config_dir = config_dir.unwrap().join(".config/gim/");
    Ok(config_dir)
}
