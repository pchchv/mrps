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
