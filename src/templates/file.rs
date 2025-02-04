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
