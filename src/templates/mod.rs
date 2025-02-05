mod file;
mod parse;
mod fetch;
mod format;
mod command;

use file::IO;
use parse::parse;
use std::error::Error;
use command::command;
use std::path::PathBuf;
use format::{format, bytes};
use minijinja::{Environment, path_loader, Value};
use fetch::{get, delete, head, options, post, put, patch};

pub fn new(dir: PathBuf, data: Option<PathBuf>) -> Result<Environment<'static>, Box<dyn Error>> {
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
    env.add_function("command", command);
    env.add_function("log", |message: &str| -> () {
        println!("{}", message);
        ()
    });
    if let Some(data) = data {
        let io1 = IO::new(data)?;
        let io2 = io1.clone();
        let io3 = io1.clone();
        env.add_function("read", move |entry: &str| -> Option<Value> {
            io1.read(entry)
        });
        env.add_function("write", move |
            file: &str,
            data: &Vec<u8>
        | -> Option<String> {
            io2.write(file, data)
        });
        env.add_function("remove", move |entry: &str| -> Option<String> {
            io3.remove(entry)
        });
    }

    Ok(env)
}