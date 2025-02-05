use std::collections::HashMap;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Proxy {
    url: String,
    method: Option<String>,
    headers: Option<HashMap<String, String>>,
    body: Option<Vec<u8>>
}

impl Proxy {
}