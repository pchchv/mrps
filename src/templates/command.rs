use serde_derive::Serialize;

#[derive(Serialize)]
struct Output {
    code: i32,
    stdout: Vec<u8>,
    stderr: Vec<u8>
}
