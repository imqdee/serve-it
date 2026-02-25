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
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
    let serve_path = format!("/{filename}");

    eprintln!("  Serving {}", path.display());
    eprintln!("  Content-Type: {content_type}");
    eprintln!("  http://127.0.0.1:{port}{serve_path}");
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

        if path_req != serve_path {
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
            continue;
        }

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
    use std::io::{Read, Write};
    use std::net::TcpStream;

    fn free_port() -> u16 {
        TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }

    fn request(port: u16, path: &str) -> String {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{port}")).unwrap();
        let req = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n");
        stream.write_all(req.as_bytes()).unwrap();
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();
        buf
    }

    #[test]
    fn serves_file_at_filename_path() {
        let dir = std::env::temp_dir().join("serve_test_ok");
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("data.json");
        fs::write(&file, r#"{"ok":true}"#).unwrap();

        let port = free_port();
        let file_clone = file.clone();
        std::thread::spawn(move || {
            let _ = start(&file_clone, port);
        });
        std::thread::sleep(std::time::Duration::from_millis(50));

        let resp = request(port, "/data.json");
        assert!(resp.starts_with("HTTP/1.1 200 OK"));
        assert!(resp.contains("application/json"));
        assert!(resp.contains(r#"{"ok":true}"#));

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn returns_404_for_root_path() {
        let dir = std::env::temp_dir().join("serve_test_root");
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("data.json");
        fs::write(&file, r#"{"ok":true}"#).unwrap();

        let port = free_port();
        let file_clone = file.clone();
        std::thread::spawn(move || {
            let _ = start(&file_clone, port);
        });
        std::thread::sleep(std::time::Duration::from_millis(50));

        let resp = request(port, "/");
        assert!(resp.starts_with("HTTP/1.1 404 Not Found"));

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn returns_404_for_wrong_path() {
        let dir = std::env::temp_dir().join("serve_test_wrong");
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("data.json");
        fs::write(&file, r#"{"ok":true}"#).unwrap();

        let port = free_port();
        let file_clone = file.clone();
        std::thread::spawn(move || {
            let _ = start(&file_clone, port);
        });
        std::thread::sleep(std::time::Duration::from_millis(50));

        let resp = request(port, "/other.json");
        assert!(resp.starts_with("HTTP/1.1 404 Not Found"));

        let _ = fs::remove_dir_all(dir);
    }

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
