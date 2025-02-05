use std::collections::HashMap;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Modify {
    pub status: Option<u16>,
    pub headers: Option<HashMap<String, String>>
}

impl Modify {
}