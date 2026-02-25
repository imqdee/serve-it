use std::path::Path;

pub fn content_type(path: &Path) -> &'static str {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "json" => "application/json",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "txt" => "text/plain",
        "xml" => "application/xml",
        "csv" => "text/csv",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "wasm" => "application/wasm",
        "pdf" => "application/pdf",
        "toml" => "application/toml",
        "yaml" | "yml" => "application/yaml",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn json() {
        assert_eq!(content_type(Path::new("data.json")), "application/json");
    }

    #[test]
    fn html() {
        assert_eq!(content_type(Path::new("index.html")), "text/html");
    }

    #[test]
    fn htm() {
        assert_eq!(content_type(Path::new("index.htm")), "text/html");
    }

    #[test]
    fn css() {
        assert_eq!(content_type(Path::new("style.css")), "text/css");
    }

    #[test]
    fn js() {
        assert_eq!(content_type(Path::new("app.js")), "application/javascript");
    }

    #[test]
    fn txt() {
        assert_eq!(content_type(Path::new("readme.txt")), "text/plain");
    }

    #[test]
    fn xml() {
        assert_eq!(content_type(Path::new("feed.xml")), "application/xml");
    }

    #[test]
    fn csv() {
        assert_eq!(content_type(Path::new("data.csv")), "text/csv");
    }

    #[test]
    fn svg() {
        assert_eq!(content_type(Path::new("icon.svg")), "image/svg+xml");
    }

    #[test]
    fn png() {
        assert_eq!(content_type(Path::new("photo.png")), "image/png");
    }

    #[test]
    fn jpg() {
        assert_eq!(content_type(Path::new("photo.jpg")), "image/jpeg");
    }

    #[test]
    fn jpeg() {
        assert_eq!(content_type(Path::new("photo.jpeg")), "image/jpeg");
    }

    #[test]
    fn gif() {
        assert_eq!(content_type(Path::new("anim.gif")), "image/gif");
    }

    #[test]
    fn wasm() {
        assert_eq!(content_type(Path::new("module.wasm")), "application/wasm");
    }

    #[test]
    fn pdf() {
        assert_eq!(content_type(Path::new("doc.pdf")), "application/pdf");
    }

    #[test]
    fn toml() {
        assert_eq!(content_type(Path::new("config.toml")), "application/toml");
    }

    #[test]
    fn yaml() {
        assert_eq!(content_type(Path::new("config.yaml")), "application/yaml");
    }

    #[test]
    fn yml() {
        assert_eq!(content_type(Path::new("config.yml")), "application/yaml");
    }

    #[test]
    fn unknown_extension() {
        assert_eq!(
            content_type(Path::new("data.xyz")),
            "application/octet-stream"
        );
    }

    #[test]
    fn no_extension() {
        assert_eq!(
            content_type(Path::new("Makefile")),
            "application/octet-stream"
        );
    }
}
