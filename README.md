
[![Version](https://img.shields.io/crates/v/mrps)](https://crates.io/crates/mrps)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-red)](https://github.com/pchchv/mrps/blob/main/LICENSE)
[![Downloads](https://img.shields.io/crates/d/mrps)](https://crates.io/crates/mrps)

# MRPS — *mini [reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy) server.*

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

## [MiniJinja](https://github.com/mitsuhiko/minijinja) templates
- [Reverse proxy](https://en.wikipedia.org/wiki/Reverse_proxy)
- Execute commands in the template
- Send HTTP requests in the template
- Parse and format to [JSON](https://www.json.org/json-en.html), [TOML](https://toml.io/en/) and [FormData](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST)
- Read, write and remove files from the filesystem
- Modify the response headers and status in the template

## Docs
### config
Command line arguments take priority over the configuration file if are both present.  

Command line argument paths are relative to the current working directory.

`config` paths are relative to your directory.

When making changes to `config`, the server must be restarted.

#### port: integer?
Optional integer port number on which the server will run, default: 3000

#### all: bool
Whether to display hidden files.

If confirmed via the command line or the `config` file, they will be
displayed.

#### ignore: [string]?
A list of files to ignore.

[glob](https://github.com/devongovett/glob-match) expressions are used.

If the -i option is passed on the command line, it will be added to the list.

Routes must be considered in relation to the assets folder, not the working directory.

#### cors: [string]?
Optional array of strings representing allowed origins for [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) requests.

An empty array allows all origins.

If this variable is not defined,[CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) will be disabled.

#### cert: string?
Optional string with the public key file path for the https server.

Only if the `cert` and `key` are available will the server run over https.

#### key: string?
Optional string containing the path to the private key file for the https server.

Only if the `cert` and `key` are available will the server run over https.

#### assets: string?
Optional string with the static files folder path.

#### templates: string?
Optional string with the path to the [minijinja](https://github.com/mitsuhiko/minijinja) templates folder.

#### data: string?
Optional string with the path where templates can `read`, `write` and `remove` files. If not passed, these functions will be unavailable to templates.

#### routes: [{method, path, template}]
Optional array of objects that define routes:
- `method` string: one of the http methods:
  - GET
  - POST
  - DELETE
  - PUT
  - PATCH
  - HEAD
  - OPTIONS
  - TRACE
  - CONNECT
- `path` string: path associated with the route, `:var` is acceptable for setting path variables (i.e: /api/user/:id).
- `template` string: path to the template associated with this route in the `templates` folder.

### Template variables

#### method: string
`method` associated with this `route`.
It is useful when the same template is used in many `routes`.

#### url: string
It is the junction of the `path` and the `route` `query`.

```http://localhost:3000/api/users?name=john#me => /api/users?name=john```

#### route: string
It is the `route` as declared in the `config` file.

```/api/user/:id```

#### path: string
Associated `path` passed by the client in the request.

```http://localhost:3000/api/users?name=john => /api/users```

#### query: string?
Associated `query` string passed by the client in the request.

```http://localhost:3000/api/users?name=john => name=john```

#### params: {name: value}
Associated object of the `path` `params` associated with the client request on a given `route`.

- `name` string: name of the parameter as declared in the `route`.
- `value` string: value of the parameter passed in the `path`.

```/api/user/:id => http://localhost:3000/api/user/25 => {"id": "25"}```

#### vars: {name: value}
Associated object of the `query` params associated with the client request.

- `name` string: The name of the parameter passed in the `query`
- `value` string: The value of the parameter passed in the `query`

```http://localhost:3000/api/users?name=john => {"name": "john"}```

#### headers: {name: value}
Associated object of the headers passed by the client in the request.

Note that all header keys are in **lowercase**.

- `name` string: name of the header passed in the request
- `value` string: value of the header passed in the request

```Content-Type: text/plain => {"content-type": "text/plain"}```

#### body: binary
Body passed by the client in the request.

### Template return state
Variables that, if defined, modify the behavior of the server response.

It only works if they are **declared outside the blocks** to be returned in the template's global state.

#### modify {status, headers: {name: value}}
The response body is always the result of the template, and this variable allows you to modify the status code and headers.

- `status` (integer?): new response status code, if not passed, will use 200 by default
- `headers` ({name: value}?): headers that should be changed in the response

An example of a redirect.
```jinja
{% set modify = {"status": 303, "headers": {"Location": "/new/location"}} %}
```

#### proxy {url, method, headers: {name, value}, body}
Uses a proxy instead of the template result.

- `url` (string): proxy URL, is required
- `method` (string?): method used for the proxy request (by default, the method passed in the original request)
- `headers` ({name: value}?): headers that should be changed in the proxy request (by default, do not change any header)
- `body` (binary?): body of the proxy request (by default, the original body)

A simple proxy that retains the request method, headers, body and path and just directs it to another host.
```jinja
{% set proxy = {"url": "https://another.host.ip"~url} %}
```

### Custom functions

#### command (cmd) -> {code, stdout, stdin}
Executes a command passed in the template.

This function does not raise errors, in case of failure it returns the `code` `999999`, and the error message.

- `cmd` string: command to be executed by the system
- `code` integer: response code, in general zero indicates OK, and a number greater than zero the error code
- `stdout` binary: standard output of the executed command
- `stderr` binary: error message returned

List files in the current directory on UNIX systems.
```jinja
{% set res = command("ls -l") %}
{% set output = res.stdout | parse("text") %}
```

#### read (file) -> data
Reads the contents of a file, if it does not exist returns `None`.

This function does not raise errors, any read error will return `None`.

It will only be available if the `config` file contains the `data` property with the folder that contains the files that can be read and modified.

- `file` string: path of the file to read
- `data` binary?: contents of the file or `None` in case of errors

```jinja
{% set content = read("some/file.json") | parse("json") %}
```

#### read (dir: string) -> [{...info}]
This function also works with a directory, which in this case will return an array with information about the files contained in it.

- `dir` string: if the path passed is a directory

**info**
- `accessed` string: last access date (%Y-%m-%d %H:%M:%S)
- `created` string: creation date (%Y-%m-%d %H:%M:%S)
- `modified` string: modification date (%Y-%m-%d %H:%M:%S)
- `is_dir` bool: 'true' if it is a directory
- `is_file` bool: 'true' if it is a file
- `is_symlink` bool: 'true' if it is a symbolic link
- `name` string: entry name
- `len` u64: size in bytes

```jinja
{% set content = read("some/dir") %}
{% for entry in content %}
  {{entry.name}}
{% endfor %}
```

#### write (file, data) -> error
Writes to a file. Create folders for the file if necessary. Always overwrites the contents if they exist.

If an error occurs, it returns the error text, otherwise `None`. Thus, it does not cause errors.

Will only be available if the `config` file contains a `data` property specifying a folder containing files that can be read and modified.

- `file` string: file path
- `data` binary: raw data to be written
- `error` string?: error message or `None`

```jinja
{% set data = "Hello world!" %}
{{write("some/file.txt", data | bytes)}}
```

#### remove (entry) -> error
Removes a file or directory recursively.

If an error occurred, the error text will be returned, otherwise `None`. Thus, it does not cause errors.

Will only be available if the `config` file contains a `data` property `data` property specifying a folder containing files that can be read and modified.

- `entry` string: path of the file or directory to be removed
- `error` string?: error message or `None`

```jinja
{{remove("some/dir")}}
```

```jinja
{{remove("some/file.txt")}}
```

#### {method} (url, body) -> {status, headers, body}
Sends a synchronous request to an external resource.

This function does not raise errors, any error in the request will be returned `status` with code `400` and a `body` containing an error message.

- `url` string: URL of the request
- `body` binary: body of the request
- `status` integer: HTTP status code of the response
- `headers` {`name` string: `value` string}: response headers
- `body` binary: response body
- `method`:
  - `get` (url) -> {status, headers, body}
  - `delete` (url) -> {status, headers, body}
  - `head` (url) -> {status, headers, body}
  - `options` (url) -> {status, headers, body}
  - `post` (url, body) -> {status, headers, body}
  - `put` (url, body) -> {status, headers, body}
  - `patch` (url, body) -> {status, headers, body}

```jinja
{% set response = get("https://some/api") %}
{% set data = response.body | parse("json") %}
```

```jinja
{% set body = "some data" %}
{% set response = post("https://some/api", body | bytes) %}
{% set message = response.body | parse("text") %}
```

#### log (message) -> ()
Prints a message from the template on the terminal.

- `message` string: content of the message

```jinja
{{ log("hi!") }}
```

### Custom filters

#### parse (data, encoding) -> result
Converts the raw data returned by some function into a template variable using the passed encoding.

This function outputs an `error` message if an unsupported encoding is used or if decoding fails.

In case of an error, it returns a request with the `status` code `500`.

- `data` binary: raw data returned from some function
- `encoding` string: encoding to be used when reading the data:
  - form: [FormData](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST)  
  - json: [JSON](https://www.json.org/json-en.html)
  - toml: [TOML](https://toml.io/en/)
  - text: just transforms the data into text
- `result`: value supported by the template with associated data

```jinja
{% set data = read("some/file.txt") | parse("text") %}
```

```jinja
{% set response = get("https://some/api") %}
{% set data = response.body | parse("json") %}
```

#### format (data, encoding) -> text
Converts a template variable to a formatted string.

This function raises an `error` if an unsupported encoding is used or if the encoding fails.

In case of an error, it returns a request with the `status` code `500`.

- `data`: any template variable
- `encoding` string: type of encoding to be adopted when formatting the text:
  - form: [FormData](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST)  
  - json: [JSON](https://www.json.org/json-en.html)
  - toml: [TOML](https://toml.io/en/)
  - debug: uses rust pretty print formatter
- `text` string: text after encoding

```jinja
{% set data = {"name": "John", "age": 30} %}
{% set text = data | format("form") %}
{{text}}
```

```name=John&age=30```

#### bytes (data) -> raw
Converts text to binary format.

- `data` string: any text
- `raw` binary: text converted to binary

```jinja
{% set error = write('hello.txt', 'Hello World!' | bytes) %}
```

```jinja
{% set response = post('http://ip/some/api', 'Hello World!' | bytes) %}
```

## Dependencies
- [clap](https://github.com/clap-rs/clap)
- [hurl](https://github.com/Orange-OpenSource/hurl)
- [axum](https://github.com/tokio-rs/axum)
- [serde](https://github.com/serde-rs/serde)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [minijinja](https://github.com/mitsuhiko/minijinja)
- [glob-match](https://github.com/devongovett/glob-match)

A huge thank you to everyone who contributed to these projects.
