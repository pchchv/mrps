use std::error::Error;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Assets {
    all: bool,
    dir: PathBuf,
    ignore: Vec<String>
}

impl Assets {
    pub fn new (all: bool, dir: PathBuf, ignore: Vec<String>) -> Result<Assets, Box<dyn Error>> {
        let p = dir.as_path();
        if !p.is_dir() {
            Err(format!("assets is not a dir: {}", p.display()).into())
        } else {
            Ok(Assets {
                all,
                dir,
                ignore
            })
        }
    }
}