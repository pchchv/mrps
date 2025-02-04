mod parse;
mod fetch;
mod format;

use parse::parse;
use std::error::Error;
use std::path::PathBuf;
use format::{format, bytes};
use fetch::{get, delete, head, options, post, put, patch};

pub fn new(dir: PathBuf) -> Result<Environment<'static>, Box<dyn Error>> {
    let mut env = Environment::new();
    env.set_loader(path_loader(dir));
    env.add_filter("parse", parse);
    env.add_filter("bytes", bytes);
    env.add_filter("format", format);
    env.add_function("get", get);
    env.add_function("post", post);
    env.add_function("put", put);
    env.add_function("patch", patch);
    env.add_function("head", head);
    env.add_function("options", options);
    env.add_function("delete", delete);
    env.add_function("log", |message: &str| -> () {
        println!("{}", message);
        ()
    });

    Ok(env)
}