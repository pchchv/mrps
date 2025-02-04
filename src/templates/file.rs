use std::fs;
use std::error::Error;
use std::path::{Path, PathBuf};
use serde_derive::Serialize;

#[derive(Serialize)]
struct File {
    accessed: String,
    created: String,
    modified: String,
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
    name: String,
    len: u64
}

#[derive(Clone)]
pub struct IO {
    dir: PathBuf
}

impl IO {
    pub fn new(dir: PathBuf) -> Result<IO, Box<dyn Error>> {
        let p = dir.as_path();
        if p.is_dir() {
            Ok(IO {dir})
        } else {
            Err(format!("Data must be a directory: {}", p.display()).into())
        }
    }

    fn get_path(&self, path: &str) -> Option<PathBuf> {
        let dir = self.dir.as_path();
        let mut path = Path::new(path);
        if let Ok(p) = path.strip_prefix("/") {
            path = p
        }

        let path = dir.join(path);
        if path.starts_with(dir) {
            Some(path)
        } else {
            None
        }
    }

    pub fn remove(&self, entry: &str) -> Option<String> {
        let path = match self.get_path(entry) {
            Some(path) => path,
            None => {
                return Some(format!("Unable to resolve path: {}", entry))
            }
        };
        let path = path.as_path();
        if path.is_dir() {
            match fs::remove_dir_all(path) {
                Ok(_) => None,
                Err(err) => Some(format!(
                    "Unable to remove dir: {}\n{:#}", entry, err
                ))
            }
        } else {
            match fs::remove_file(path) {
                Ok(_) => None,
                Err(err) => Some(format!(
                    "Unable to remove file: {}\n{:#}", entry, err
                ))
            }
        }
    }
} 