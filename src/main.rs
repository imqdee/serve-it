mod mime;
mod server;

use std::path::PathBuf;
use std::process;

use clap::Parser;

#[derive(Parser)]
#[command(name = "serve")]
#[command(about = "Serve a single local file over HTTP")]
struct Cli {
    /// Path to the file to serve
    path: PathBuf,

    /// Port to listen on
    #[arg(default_value_t = 8000)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    if !cli.path.exists() {
        eprintln!("error: file not found: {}", cli.path.display());
        process::exit(1);
    }

    if !cli.path.is_file() {
        eprintln!("error: not a file: {}", cli.path.display());
        process::exit(1);
    }

    if let Err(err) = server::start(&cli.path, cli.port) {
        eprintln!("error: {err}");
        process::exit(1);
    }
}
