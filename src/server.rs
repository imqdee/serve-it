use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::Path;

use crate::mime;

pub enum ServeError {
    Io(std::io::Error),
}

impl std::fmt::Display for ServeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServeError::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::fmt::Debug for ServeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServeError::Io(err) => write!(f, "ServeError::Io({err:?})"),
        }
    }
}

impl std::error::Error for ServeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServeError::Io(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for ServeError {
    fn from(err: std::io::Error) -> Self {
        ServeError::Io(err)
    }
}

pub fn start(path: &Path, port: u16) -> Result<(), ServeError> {
    let content_type = mime::content_type(path);
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;

    eprintln!("  Serving {}", path.display());
    eprintln!("  Content-Type: {content_type}");
    eprintln!("  http://127.0.0.1:{port}");
    eprintln!();

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(err) => {
                eprintln!("  connection error: {err}");
                continue;
            }
        };

        let reader = BufReader::new(&stream);
        let request_line = reader.lines().next().unwrap_or(Ok(String::new()))?;
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("???");
        let path_req = parts.next().unwrap_or("/");

        match fs::read(path) {
            Ok(body) => {
                let response = format!(
                    "HTTP/1.1 200 OK\r\n\
                     Content-Type: {content_type}\r\n\
                     Content-Length: {}\r\n\
                     Access-Control-Allow-Origin: *\r\n\
                     Cache-Control: no-cache\r\n\
                     Connection: close\r\n\
                     \r\n",
                    body.len()
                );
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.write_all(&body);
                eprintln!("  {method} {path_req} 200");
            }
            Err(_) => {
                let body = b"404 Not Found";
                let response = format!(
                    "HTTP/1.1 404 Not Found\r\n\
                     Content-Type: text/plain\r\n\
                     Content-Length: {}\r\n\
                     Connection: close\r\n\
                     \r\n",
                    body.len()
                );
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.write_all(body);
                eprintln!("  {method} {path_req} 404");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serve_error_display() {
        let err = ServeError::Io(std::io::Error::new(
            std::io::ErrorKind::AddrInUse,
            "port taken",
        ));
        assert_eq!(err.to_string(), "port taken");
    }

    #[test]
    fn serve_error_debug() {
        let err = ServeError::Io(std::io::Error::new(
            std::io::ErrorKind::AddrInUse,
            "port taken",
        ));
        let debug = format!("{err:?}");
        assert!(debug.contains("ServeError::Io"));
    }

    #[test]
    fn serve_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
        let err: ServeError = io_err.into();
        assert_eq!(err.to_string(), "gone");
    }

    #[test]
    fn serve_error_source() {
        use std::error::Error;
        let err = ServeError::Io(std::io::Error::new(std::io::ErrorKind::Other, "inner"));
        assert!(err.source().is_some());
    }
}
