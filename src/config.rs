use std::path::PathBuf;
use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Route {
    pub path: String,
    pub method: String,
    pub template: String
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub all: Option<bool>,
    pub key: Option<PathBuf>,
    pub cors: Option<Vec<String>>,
    pub port: Option<u16>,
    pub cert: Option<PathBuf>,
    pub data: Option<PathBuf>, 
    pub ignore: Option<Vec<String>>,
    pub assets: Option<PathBuf>, 
    pub routes: Option<Vec<Route>>,
    pub templates: Option<PathBuf>
}
