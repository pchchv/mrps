use minijinja::Value;
use serde_derive::Serialize;
use std::collections::HashMap;
use reqwest::blocking::Response;

#[derive(Serialize)]
struct Res {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

impl Res {
    fn new(response: Response) -> Value {
        let status = response.status().as_u16();
        let mut headers = HashMap::new();
        for key in response.headers().keys() {
            if let Some(value) = response.headers().get(key) {
                if let Ok(value) = value.to_str() {
                    headers.insert(key.to_string(), value.to_string());
                }
            }
        }

        match response.bytes() {
            Ok(body) => Value::from_serialize(Res {
                status,
                headers,
                body: body.to_vec()
            }),
            Err(_) => Value::from_serialize(Res {
                status,
                headers,
                body: Vec::new()
            })
        }
    }

    fn err(message: String) -> Value {
        Value::from_serialize(Res {
            status: 400,
            headers: HashMap::new(),
            body: message.as_bytes().to_vec()
        })
    }
}