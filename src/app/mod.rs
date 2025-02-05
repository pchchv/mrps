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
}
