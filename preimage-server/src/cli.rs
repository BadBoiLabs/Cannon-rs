use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to pre-image json file to load or directory to scan
    pub path: PathBuf,
}
