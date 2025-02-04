mod parse;
mod format;

use parse::parse;
use std::error::Error;
use std::path::PathBuf;
use format::{format, bytes};
use minijinja::{Environment, path_loader};

pub fn new(dir: PathBuf) -> Result<Environment<'static>, Box<dyn Error>> {
    let mut env = Environment::new();
    env.set_loader(path_loader(dir));
    env.add_filter("parse", parse);
    env.add_filter("bytes", bytes);
    env.add_filter("format", format);
    env.add_function("log", |message: &str| -> () {
        println!("{}", message);
        ()
    });

    Ok(env)
}