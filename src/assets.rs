use std::fs::read;
use std::error::Error;
use std::path::{Path, PathBuf};
use axum::{response::Response, http::StatusCode, http::header::{HeaderValue, CONTENT_TYPE}};
use glob_match::glob_match;
use crate::debug::debug;

#[derive(Clone)]
pub struct Assets {
    all: bool,
    dir: PathBuf,
    ignore: Vec<String>
}

impl Assets {
    pub fn new(all: bool, dir: PathBuf, ignore: Vec<String>) -> Result<Assets, Box<dyn Error>> {
        let p = dir.as_path();
        if !p.is_dir() {
            Err(format!("assets is not a dir: {}", p.display()).into())
        } else {
            Ok(Assets {
                all,
                dir,
                ignore
            })
        }
    }

    fn getter(&self, path_str: &str) -> Result<Response, StatusCode> {
        let path = Path::new(path_str);
        let dir = self.dir.as_path();
        let mut file = dir.join(path);
        if file.is_dir() {
            file = file.join("index.html");
        }

        if !file.starts_with(dir) || !file.is_file() {
            return Err(StatusCode::NOT_FOUND);
        }

        let path_str = path.to_str().unwrap_or("");
        for glob in &self.ignore {
            if glob_match(glob, path_str) {
                return Err(StatusCode::NOT_FOUND);
            }
        }

        if !self.all {
            for component in path.components() {
                let name = component.as_os_str().to_str().unwrap_or("");
                
                if name.is_empty() || (
                    name.len() > 1 &&
                    name.as_bytes()[0] == b'.'
                ) {
                    return Err(StatusCode::NOT_FOUND);
                } 
            }
        }
        
        let mut response: Response;
        match read(&file) {
            Err(_) => {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            },
            Ok(body) => {
                response = Response::new(body.into());
            }
        };

        let mime = mime_guess::from_path(&file).first_raw().unwrap_or("");
        if !mime.is_empty() {
            if let Ok(mime) = HeaderValue::from_str(mime) {
                response.headers_mut().insert(CONTENT_TYPE, mime);
            };
        }

        Ok(response)
    }

    pub fn get(&self, path_str: &str) -> Result<Response, StatusCode> {
        let path = format!("/{}", path_str);
        match self.getter(path_str) {
            Ok(response) => {
                debug("GET", &path, Some(200), "");
                Ok(response)
            },
            Err(status) => {
                debug("GET", &path, Some(status.as_u16()), "");
                Err(status)
            }
        }
    }
}