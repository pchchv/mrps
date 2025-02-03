use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Route {
    pub path: String,
    pub method: String,
    pub template: String
}
