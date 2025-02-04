use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Res {
    status: u16,
    body: Vec<u8>,
    headers: HashMap<String, String>
}

impl Res {
}
