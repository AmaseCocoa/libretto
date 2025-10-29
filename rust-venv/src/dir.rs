use std::path::{Path, PathBuf};
use std::env;
use std::io;

pub fn make_relative_to_current_dir(target_path_str: &str) -> Result<PathBuf, io::Error> {
    let current_dir = env::current_dir()?;

    let target_abs_path = Path::new(target_path_str).canonicalize()?;

    match target_abs_path.strip_prefix(&current_dir) {
        Ok(relative_path) => {
            Ok(relative_path.to_path_buf())
        },
        Err(_) => {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "The target path is not under the current directory.",
            ))
        }
    }
}