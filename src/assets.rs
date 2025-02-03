use std::path::PathBuf;

#[derive(Clone)]
pub struct Assets {
    all: bool,
    dir: PathBuf,
    ignore: Vec<String>
}

impl Assets {
}