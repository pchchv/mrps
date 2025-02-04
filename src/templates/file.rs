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
} 