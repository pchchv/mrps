use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Context {
    pub method: String,
    pub url: String,
    route: String,
    path: String,
    query: String,
    params: HashMap<String, String>,
    vars: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

impl Context {
}