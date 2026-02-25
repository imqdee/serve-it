# serve

Serve a single local file over HTTP. One file in, one URL out.

## Install

```sh
cargo install serve-it
```

## Usage

```sh
# Serve a file on the default port (8000)
serve ./data.json

# Serve on a custom port
serve ./index.html 9000
```

## Options

| Argument | Type      | Required | Default | Description               |
| -------- | --------- | -------- | ------- | ------------------------- |
| `PATH`   | `PathBuf` | yes      |         | Path to the file to serve |
| `PORT`   | `u16`     | no       | `8000`  | Port to listen on         |

## Response Headers

Every response includes:

- `Content-Type` matching the file extension
- `Content-Length`
- `Access-Control-Allow-Origin: *` (CORS)
- `Cache-Control: no-cache`

## Supported MIME Types

| Extension       | Content-Type               |
| --------------- | -------------------------- |
| `.json`         | `application/json`         |
| `.html`, `.htm` | `text/html`                |
| `.css`          | `text/css`                 |
| `.js`           | `application/javascript`   |
| `.txt`          | `text/plain`               |
| `.xml`          | `application/xml`          |
| `.csv`          | `text/csv`                 |
| `.svg`          | `image/svg+xml`            |
| `.png`          | `image/png`                |
| `.jpg`, `.jpeg` | `image/jpeg`               |
| `.gif`          | `image/gif`                |
| `.wasm`         | `application/wasm`         |
| `.pdf`          | `application/pdf`          |
| `.toml`         | `application/toml`         |
| `.yaml`, `.yml` | `application/yaml`         |
| (other)         | `application/octet-stream` |

## Development

```sh
# Build
cargo build

# Run tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format check
cargo fmt -- --check
```

Git hooks via [lefthook](https://github.com/evilmartians/lefthook):

```sh
lefthook install
```

## License

MIT
