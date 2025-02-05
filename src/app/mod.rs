mod proxy;
mod modify;
mod context;

use minijinja::Environment;
use axum::http::HeaderValue;

type Env = Environment<'static>;

#[derive(Clone)]
pub struct AppState {
    env: Env,
    template: String,
    mime: Option<HeaderValue>
}

impl AppState {
    pub fn new (env: &Env, template: &str) -> AppState {
        AppState {
            env: env.clone(),
            template: template.to_string(),
            mime: match mime_guess::from_path(template).first_raw() {
                Some(mime) => match HeaderValue::from_str(mime) {
                    Ok(mime) => Some(mime),
                    Err(_) => None
                },
                None => None
            }
        }
    }
}