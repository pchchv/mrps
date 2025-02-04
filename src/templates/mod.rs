mod parse;
mod format;

use std::error::Error;
use std::path::PathBuf;
use minijinja::{Environment, path_loader};

pub fn new(dir: PathBuf) -> Result<Environment<'static>, Box<dyn Error>> {
    let mut env = Environment::new();
    env.set_loader(path_loader(dir));
    env.add_function("log", |message: &str| -> () {
        println!("{}", message);
        ()
    });

    Ok(env)
}