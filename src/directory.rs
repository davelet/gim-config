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

    let config_dir = config_dir.unwrap().join(".config").join("gim");
    Ok(config_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir() {
        let result = config_dir();
        assert!(result.is_ok(), "config_dir should return Ok result");
        
        let path = result.unwrap();
        assert!(path.ends_with(".config/gim") || path.ends_with(".config\\gim"), 
                "Path should end with .config/gim or .config\\gim");
        
        // Check that the path contains the home directory
        let home = dirs::home_dir().unwrap();
        assert!(path.starts_with(home), "Config path should start with home directory");
    }
}
