# Mini RPS

Mini [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) server.

## Features
 - [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
 - HTTPS
 - Safe rust
 - Static file server
 - Optional configuration file can be written in
[JSON](https://www.json.org/json-en.html) or
[TOML](https://toml.io/en/)
 - Additional [minijinja](https://github.com/mitsuhiko/minijinja) templates with custom functions
 - No panics after startup (every panic is a bug)
 - Good debugging experience (server displays requests and error messages in human-readable templates)

## Dependencies:
 - [clap](https://github.com/clap-rs/clap)
 - [hurl](https://github.com/Orange-OpenSource/hurl)
 - [axum](https://github.com/tokio-rs/axum)
 - [serde](https://github.com/serde-rs/serde)
 - [reqwest](https://github.com/seanmonstar/reqwest)
 - [minijinja](https://github.com/mitsuhiko/minijinja)
 - [glob-match](https://github.com/devongovett/glob-match)

A huge thank you to everyone who contributed to these projects.