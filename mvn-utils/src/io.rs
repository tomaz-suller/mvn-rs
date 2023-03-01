use std::{io, fs};
use std::path::{Path, PathBuf};

pub fn file_exists(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    if let Ok(exists) = path.try_exists() {
        if exists {
            return Ok(path.to_path_buf());
        }
    }
    Err("input file does not exist")
}

pub fn read_file_to_string(path: &PathBuf) -> String {
    fs::read_to_string(path)
        .expect("failed to read file")
        .to_uppercase()
}

pub fn read_stdin_to_string() -> String {
    io::stdin()
        .lines()
        .map(|result| result.expect("unable to read from stdin"))
        .reduce(|mut acc, result| {
            acc.push('\n');
            acc.push_str(&result);
            acc
        })
        .unwrap_or_default()
        .to_uppercase()
}
