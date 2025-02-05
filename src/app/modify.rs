use minijinja::Value;
use std::error::Error;
use serde::Deserialize;
use std::collections::HashMap;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Modify {
    pub status: Option<u16>,
    pub headers: Option<HashMap<String, String>>
}

impl Modify {
    pub fn new (modify: &Value) -> Result<Modify, Box<dyn Error>> {
        Ok(Modify::deserialize(modify)?)
    }
}